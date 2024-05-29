pub mod GrpcClient {

    use flyteidl::flyteidl::admin;
    use flyteidl::flyteidl::service::admin_service_client::AdminServiceClient;
    use keyring::Entry;
    use prost::{DecodeError, EncodeError, Message};
    use pyo3::exceptions::{PyOSError, PyValueError};
    use pyo3::prelude::*;
    use pyo3::types::{PyBytes, PyDict};
    use pyo3::PyErr;
    use std::fmt;
    use std::sync::Arc;
    use tokio::runtime::{Builder, Runtime};
    use tonic::{
        metadata::MetadataValue,
        service::interceptor::InterceptedService,
        service::Interceptor,
        transport::{Channel, Uri},
        Request, Response, Status,
    };

    use crate::auth::Authenticator;

    // Foreign Rust error types: https://pyo3.rs/main/function/error-handling#foreign-rust-error-types
    // Create a newtype wrapper, e.g. MyOtherError. Then implement From<MyOtherError> for PyErr (or PyErrArguments), as well as From<OtherError> for MyOtherError.

    // Failed at encoding responded object to bytes string
    struct MessageEncodeError(EncodeError);
    // Failed at decoding requested object from bytes string
    struct MessageDecodeError(DecodeError);

    impl fmt::Display for MessageEncodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "")
        }
    }

    impl fmt::Display for MessageDecodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "")
        }
    }

    impl std::convert::From<MessageEncodeError> for PyErr {
        fn from(err: MessageEncodeError) -> PyErr {
            PyOSError::new_err(err.to_string())
        }
    }

    impl std::convert::From<MessageDecodeError> for PyErr {
        fn from(err: MessageDecodeError) -> PyErr {
            PyOSError::new_err(err.to_string())
        }
    }

    impl std::convert::From<EncodeError> for MessageEncodeError {
        fn from(other: EncodeError) -> Self {
            Self(other)
        }
    }

    impl std::convert::From<DecodeError> for MessageDecodeError {
        fn from(other: DecodeError) -> Self {
            Self(other)
        }
    }

    // You can also use the `Interceptor` trait to create an interceptor type
    // that is easy to name
    pub struct UnaryAuthInterceptor {
        _access_token: String,
    }

    impl Interceptor for UnaryAuthInterceptor {
        fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
            println!("access_token:\t{}\n", self._access_token.clone());
            let metadata_token: MetadataValue<_> =
                match format!("Bearer {}", self._access_token).parse::<MetadataValue<_>>() {
                    Ok(metadata_token) => metadata_token,
                    Err(error) => panic!("{}", error),
                };
            // let metadata_token: MetadataValue<_> = match "Bearer <self._access_token>".parse() {
            //     Ok(metadata_token) => metadata_token,
            //     Err(error) => panic!("{}", error),
            // };
            println!("metadata_token:\t{:?}\n", metadata_token.clone());
            request
                .metadata_mut()
                .insert("flyte-authorization", metadata_token.clone());
            Ok(request)
        }
    }

    /// A Python class constructs the gRPC service stubs and a Tokio asynchronous runtime in Rust.
    #[pyclass(subclass)]
    pub struct FlyteClient {
        admin_service: AdminServiceClient<InterceptedService<Channel, UnaryAuthInterceptor>>,
        runtime: Runtime,
    }

    pub fn decode_proto<T>(bytes_obj: &PyBytes) -> Result<T, MessageDecodeError>
    where
        T: Message + Default,
    {
        let bytes = bytes_obj.as_bytes();
        let de = Message::decode(&bytes.to_vec()[..]);
        Ok(de?)
    }

    pub fn encode_proto<T>(res: T) -> Result<Vec<u8>, MessageEncodeError>
    where
        T: Message + Default,
    {
        let mut buf = vec![];
        res.encode(&mut buf)?;
        Ok(buf)
    }

    #[pymethods]
    impl FlyteClient {
        #[new] // Without this, you cannot construct the underlying class in Python.
        #[pyo3(signature = (endpoint))]
        pub fn new(endpoint: &str) -> PyResult<FlyteClient> {
            // Use Atomic Reference Counting abstractions as a cheap way to pass string reference into another thread that outlives the scope.
            let s = Arc::new(endpoint);
            // Check details for constructing Tokio asynchronous `runtime`: https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html#method.new_current_thread
            let rt = match Builder::new_current_thread().enable_all().build() {
                Ok(rt) => rt,
                Err(error) => panic!("Failed to initiate Tokio multi-thread runtime: {:?}", error),
            };
            // Check details for constructing `channel`: https://docs.rs/tonic/latest/tonic/transport/struct.Channel.html#method.builder
            // TODO: generally handle more protocols, e.g., `https://`
            let endpoint_uri = match format!("http://{}", *s.clone()).parse::<Uri>() {
                Ok(uri) => uri,
                Err(error) => panic!(
                    "Got invalid endpoint when parsing endpoint_uri: {:?}",
                    error
                ),
            };
            // `Channel::builder(endpoint_uri)` returns type `tonic::transport::Endpoint`.
            let channel = match rt.block_on(Channel::builder(endpoint_uri).connect()) {
                Ok(ch) => ch,
                Err(error) => panic!(
                    "Failed at connecting to endpoint when constructing channel: {:?}",
                    error
                ),
            };

            Authenticator::PKCEAuthenticator::authenticate();
            let credentials_for_endpoint = *s.clone(); // default key: flyte-default
            let credentials_access_token_key = "access_token";
            println!("{:?}", credentials_for_endpoint);
            let entry = match Entry::new(credentials_for_endpoint, credentials_access_token_key) {
                Ok(entry) => entry,
                Err(err) => {
                    println!("{}", credentials_access_token_key);
                    panic!("Failed at initializing keyring, not available.");
                }
            };

            let access_token = match entry.get_password() {
                Ok(access_token) => {
                    println!("keyring retrieved successfully.");
                    access_token
                }
                Err(error) => panic!("Failed at retrieving keyring: {:?}", error),
            };
            // Binding connected channel into service client stubs.
            // let stub = AdminServiceClient::new(channel);
            // let stub = AdminServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            //     req.metadata_mut().insert("authorization", access_token.clone());
            //     Ok(req)
            // });
            let mut interceptor = UnaryAuthInterceptor {
                _access_token: access_token,
            };

            let stub = AdminServiceClient::with_interceptor(channel, interceptor);

            Ok(FlyteClient {
                runtime: rt, // The tokio runtime is used in a blocking manner for now.
                admin_service: stub,
            })
        }

        pub fn get_task(&mut self, py: Python, bytes_obj: &PyBytes) -> PyResult<PyObject> {
            // Receive the request object in bytes from Python: flytekit remote
            let bytes = bytes_obj.as_bytes();
            // Deserialize bytes object into flyteidl type
            let decoded: admin::ObjectGetRequest = decode_proto(bytes_obj)?;
            // Prepare request object for gRPC services
            let req = Request::new(decoded);

            // Interacting with the gRPC service server: flyteadmin
            let res = (match self.runtime.block_on(self.admin_service.get_task(req)) {
                Ok(res) => res,
                Err(error) => panic!(
                    "Failed at awaiting response from gRPC service server: {:?}",
                    error
                ),
            })
            .into_inner();

            // Serialize service response object into flyteidl bytes buffer
            let buf: Vec<u8> = encode_proto(res)?;

            // Returning bytes buffer back to Python: flytekit remote for further parsing.
            Ok(PyBytes::new_bound(py, &buf).into())
        }

        pub fn create_task(&mut self, py: Python, bytes_obj: &PyBytes) -> PyResult<PyObject> {
            let decoded: admin::TaskCreateRequest = decode_proto(bytes_obj)?;
            let req = tonic::Request::new(decoded);

            let res = (match self.runtime.block_on(self.admin_service.create_task(req)) {
                Ok(res) => res,
                Err(error) => panic!(
                    "Failed at awaiting response from  gRPC service server: {:?}",
                    error
                ),
            })
            .into_inner();

            let buf: Vec<u8> = encode_proto(res)?;

            Ok(PyBytes::new_bound(py, &buf).into())
        }

        pub fn list_task_ids_paginated(
            &mut self,
            py: Python,
            bytes_obj: &PyBytes,
        ) -> PyResult<PyObject> {
            let decoded: admin::NamedEntityIdentifierListRequest = decode_proto(bytes_obj)?;
            let req = tonic::Request::new(decoded);

            let res = (match self.runtime.block_on(self.admin_service.list_task_ids(req)) {
                Ok(res) => res,
                Err(error) => panic!(
                    "Failed at awaiting response from  gRPC service server: {:?}",
                    error
                ),
            })
            .into_inner();

            let buf: Vec<u8> = encode_proto(res)?;

            Ok(PyBytes::new_bound(py, &buf).into())
        }

        pub fn list_tasks_paginated(
            &mut self,
            py: Python,
            bytes_obj: &PyBytes,
        ) -> PyResult<PyObject> {
            let decoded: admin::ResourceListRequest = decode_proto(bytes_obj)?;
            let req = tonic::Request::new(decoded);

            let res = (match self.runtime.block_on(self.admin_service.list_tasks(req)) {
                Ok(res) => res,
                Err(error) => panic!(
                    "Failed at awaiting response from  gRPC service server: {:?}",
                    error
                ),
            })
            .into_inner();

            let buf: Vec<u8> = encode_proto(res)?;

            Ok(PyBytes::new_bound(py, &buf).into())
        }
    }
}
