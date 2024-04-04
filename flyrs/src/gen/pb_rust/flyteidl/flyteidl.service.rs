/// Generated client implementations.
pub mod signal_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// SignalService defines an RPC Service that may create, update, and retrieve signal(s).
    #[derive(Debug, Clone)]
    pub struct SignalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SignalServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SignalServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SignalServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SignalServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Fetches or creates a :ref:`ref_flyteidl.admin.Signal`.
        pub async fn get_or_create_signal(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::SignalGetOrCreateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::Signal>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.SignalService/GetOrCreateSignal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.SignalService",
                        "GetOrCreateSignal",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.Signal` definitions.
        pub async fn list_signals(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::SignalListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::SignalList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.SignalService/ListSignals",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.SignalService", "ListSignals"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the value on a :ref:`ref_flyteidl.admin.Signal` definition
        pub async fn set_signal(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::SignalSetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::SignalSetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.SignalService/SetSignal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.SignalService", "SetSignal"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Represents a request structure to create task.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskCreateRequest {
    /// The inputs required to start the execution. All required inputs must be
    /// included in this map. If not required and not provided, defaults apply.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub inputs: ::core::option::Option<super::core::LiteralMap>,
    /// Template of the task that encapsulates all the metadata of the task.
    #[prost(message, optional, tag = "2")]
    pub template: ::core::option::Option<super::core::TaskTemplate>,
    /// Prefix for where task output data will be written. (e.g. s3://my-bucket/randomstring)
    #[prost(string, tag = "3")]
    pub output_prefix: ::prost::alloc::string::String,
}
/// Represents a create response structure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskCreateResponse {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
/// A message used to fetch a job state from backend plugin server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskGetRequest {
    /// A predefined yet extensible Task type identifier.
    #[prost(string, tag = "1")]
    pub task_type: ::prost::alloc::string::String,
    /// The unique id identifying the job.
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
}
/// Response to get an individual task state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskGetResponse {
    /// The state of the execution is used to control its visibility in the UI/CLI.
    #[prost(enumeration = "State", tag = "1")]
    pub state: i32,
    /// The outputs of the execution. It's typically used by sql task. Flyteplugins service will create a
    /// Structured dataset pointing to the query result table.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub outputs: ::core::option::Option<super::core::LiteralMap>,
}
/// A message used to delete a task.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskDeleteRequest {
    /// A predefined yet extensible Task type identifier.
    #[prost(string, tag = "1")]
    pub task_type: ::prost::alloc::string::String,
    /// The unique id identifying the job.
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
}
/// Response to delete a task.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskDeleteResponse {}
/// The state of the execution is used to control its visibility in the UI/CLI.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    RetryableFailure = 0,
    PermanentFailure = 1,
    Pending = 2,
    Running = 3,
    Succeeded = 4,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::RetryableFailure => "RETRYABLE_FAILURE",
            State::PermanentFailure => "PERMANENT_FAILURE",
            State::Pending => "PENDING",
            State::Running => "RUNNING",
            State::Succeeded => "SUCCEEDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RETRYABLE_FAILURE" => Some(Self::RetryableFailure),
            "PERMANENT_FAILURE" => Some(Self::PermanentFailure),
            "PENDING" => Some(Self::Pending),
            "RUNNING" => Some(Self::Running),
            "SUCCEEDED" => Some(Self::Succeeded),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod external_plugin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ExternalPluginService defines an RPC Service that allows propeller to send the request to the backend plugin server.
    #[derive(Debug, Clone)]
    pub struct ExternalPluginServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExternalPluginServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ExternalPluginServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ExternalPluginServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ExternalPluginServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Send a task create request to the backend plugin server.
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::TaskCreateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TaskCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.ExternalPluginService/CreateTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.ExternalPluginService",
                        "CreateTask",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get job status.
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::TaskGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TaskGetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.ExternalPluginService/GetTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.ExternalPluginService", "GetTask"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete the task resource.
        pub async fn delete_task(
            &mut self,
            request: impl tonic::IntoRequest<super::TaskDeleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TaskDeleteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.ExternalPluginService/DeleteTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.ExternalPluginService",
                        "DeleteTask",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUploadLocationResponse {
    /// SignedUrl specifies the url to use to upload content to (e.g. <https://my-bucket.s3.amazonaws.com/randomstring/suffix.tar?X-...>)
    #[prost(string, tag = "1")]
    pub signed_url: ::prost::alloc::string::String,
    /// NativeUrl specifies the url in the format of the configured storage provider (e.g. s3://my-bucket/randomstring/suffix.tar)
    #[prost(string, tag = "2")]
    pub native_url: ::prost::alloc::string::String,
    /// ExpiresAt defines when will the signed URL expires.
    #[prost(message, optional, tag = "3")]
    pub expires_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Data proxy generates these headers for client, and they have to add these headers to the request when uploading the file.
    #[prost(map = "string, string", tag = "4")]
    pub headers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// CreateUploadLocationRequest specified request for the CreateUploadLocation API.
/// The implementation in data proxy service will create the s3 location with some server side configured prefixes,
/// and then:
///    - project/domain/(a deterministic str representation of the content_md5)/filename (if present); OR
///    - project/domain/filename_root (if present)/filename (if present).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUploadLocationRequest {
    /// Project to create the upload location for
    /// +required
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// Domain to create the upload location for.
    /// +required
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    /// Filename specifies a desired suffix for the generated location. E.g. `file.py` or `pre/fix/file.zip`.
    /// +optional. By default, the service will generate a consistent name based on the provided parameters.
    #[prost(string, tag = "3")]
    pub filename: ::prost::alloc::string::String,
    /// ExpiresIn defines a requested expiration duration for the generated url. The request will be rejected if this
    /// exceeds the platform allowed max.
    /// +optional. The default value comes from a global config.
    #[prost(message, optional, tag = "4")]
    pub expires_in: ::core::option::Option<::prost_types::Duration>,
    /// ContentMD5 restricts the upload location to the specific MD5 provided. The ContentMD5 will also appear in the
    /// generated path.
    /// +required
    #[prost(bytes = "vec", tag = "5")]
    pub content_md5: ::prost::alloc::vec::Vec<u8>,
    /// If present, data proxy will use this string in lieu of the md5 hash in the path. When the filename is also included
    /// this makes the upload location deterministic. The native url will still be prefixed by the upload location prefix
    /// in data proxy config. This option is useful when uploading multiple files.
    /// +optional
    #[prost(string, tag = "6")]
    pub filename_root: ::prost::alloc::string::String,
    /// If true, the data proxy will add content_md5 to the metadata to the signed URL and
    /// it will force clients to add this metadata to the object.
    /// This make sure dataproxy is backward compatible with the old flytekit.
    #[prost(bool, tag = "7")]
    pub add_content_md5_metadata: bool,
    /// Optional, org key applied to the resource.
    #[prost(string, tag = "8")]
    pub org: ::prost::alloc::string::String,
}
/// CreateDownloadLocationRequest specified request for the CreateDownloadLocation API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLocationRequest {
    /// NativeUrl specifies the url in the format of the configured storage provider (e.g. s3://my-bucket/randomstring/suffix.tar)
    #[prost(string, tag = "1")]
    pub native_url: ::prost::alloc::string::String,
    /// ExpiresIn defines a requested expiration duration for the generated url. The request will be rejected if this
    /// exceeds the platform allowed max.
    /// +optional. The default value comes from a global config.
    #[prost(message, optional, tag = "2")]
    pub expires_in: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLocationResponse {
    /// SignedUrl specifies the url to use to download content from (e.g. <https://my-bucket.s3.amazonaws.com/randomstring/suffix.tar?X-...>)
    #[prost(string, tag = "1")]
    pub signed_url: ::prost::alloc::string::String,
    /// ExpiresAt defines when will the signed URL expires.
    #[prost(message, optional, tag = "2")]
    pub expires_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// CreateDownloadLinkRequest defines the request parameters to create a download link (signed url)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkRequest {
    /// ArtifactType of the artifact requested.
    #[prost(enumeration = "ArtifactType", tag = "1")]
    pub artifact_type: i32,
    /// ExpiresIn defines a requested expiration duration for the generated url. The request will be rejected if this
    /// exceeds the platform allowed max.
    /// +optional. The default value comes from a global config.
    #[prost(message, optional, tag = "2")]
    pub expires_in: ::core::option::Option<::prost_types::Duration>,
    #[prost(oneof = "create_download_link_request::Source", tags = "3")]
    pub source: ::core::option::Option<create_download_link_request::Source>,
}
/// Nested message and enum types in `CreateDownloadLinkRequest`.
pub mod create_download_link_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// NodeId is the unique identifier for the node execution. For a task node, this will retrieve the output of the
        /// most recent attempt of the task.
        #[prost(message, tag = "3")]
        NodeExecutionId(super::super::core::NodeExecutionIdentifier),
    }
}
/// CreateDownloadLinkResponse defines the response for the generated links
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkResponse {
    /// SignedUrl specifies the url to use to download content from (e.g. <https://my-bucket.s3.amazonaws.com/randomstring/suffix.tar?X-...>)
    #[deprecated]
    #[prost(string, repeated, tag = "1")]
    pub signed_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ExpiresAt defines when will the signed URL expire.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub expires_at: ::core::option::Option<::prost_types::Timestamp>,
    /// New wrapper object containing the signed urls and expiration time
    #[prost(message, optional, tag = "3")]
    pub pre_signed_urls: ::core::option::Option<PreSignedUrLs>,
}
/// Wrapper object since the message is shared across this and the GetDataResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreSignedUrLs {
    /// SignedUrl specifies the url to use to download content from (e.g. <https://my-bucket.s3.amazonaws.com/randomstring/suffix.tar?X-...>)
    #[prost(string, repeated, tag = "1")]
    pub signed_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ExpiresAt defines when will the signed URL expire.
    #[prost(message, optional, tag = "2")]
    pub expires_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// General request artifact to retrieve data from a Flyte artifact url.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataRequest {
    /// A unique identifier in the form of flyte://<something> that uniquely, for a given Flyte
    /// backend, identifies a Flyte artifact (\[i\]nput, \[o\]output, flyte \[d\]eck, etc.).
    /// e.g. flyte://v1/proj/development/execid/n2/0/i (for 0th task execution attempt input)
    ///       flyte://v1/proj/development/execid/n2/i (for node execution input)
    ///       flyte://v1/proj/development/execid/n2/o/o3 (the o3 output of the second node)
    #[prost(string, tag = "1")]
    pub flyte_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataResponse {
    #[prost(oneof = "get_data_response::Data", tags = "1, 2, 3")]
    pub data: ::core::option::Option<get_data_response::Data>,
}
/// Nested message and enum types in `GetDataResponse`.
pub mod get_data_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// literal map data will be returned
        #[prost(message, tag = "1")]
        LiteralMap(super::super::core::LiteralMap),
        /// Flyte deck html will be returned as a signed url users can download
        #[prost(message, tag = "2")]
        PreSignedUrls(super::PreSignedUrLs),
        /// Single literal will be returned. This is returned when the user/url requests a specific output or input
        /// by name. See the o3 example above.
        #[prost(message, tag = "3")]
        Literal(super::super::core::Literal),
    }
}
/// ArtifactType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ArtifactType {
    /// ARTIFACT_TYPE_UNDEFINED is the default, often invalid, value for the enum.
    Undefined = 0,
    /// ARTIFACT_TYPE_DECK refers to the deck html file optionally generated after a task, a workflow or a launch plan
    /// finishes executing.
    Deck = 1,
}
impl ArtifactType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ArtifactType::Undefined => "ARTIFACT_TYPE_UNDEFINED",
            ArtifactType::Deck => "ARTIFACT_TYPE_DECK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ARTIFACT_TYPE_UNDEFINED" => Some(Self::Undefined),
            "ARTIFACT_TYPE_DECK" => Some(Self::Deck),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod data_proxy_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// DataProxyService defines an RPC Service that allows access to user-data in a controlled manner.
    #[derive(Debug, Clone)]
    pub struct DataProxyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataProxyServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DataProxyServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DataProxyServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DataProxyServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// CreateUploadLocation creates a signed url to upload artifacts to for a given project/domain.
        pub async fn create_upload_location(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUploadLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUploadLocationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.DataProxyService/CreateUploadLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.DataProxyService",
                        "CreateUploadLocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateDownloadLocation creates a signed url to download artifacts.
        pub async fn create_download_location(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDownloadLocationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.DataProxyService/CreateDownloadLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.DataProxyService",
                        "CreateDownloadLocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateDownloadLocation creates a signed url to download artifacts.
        pub async fn create_download_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDownloadLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.DataProxyService/CreateDownloadLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.DataProxyService",
                        "CreateDownloadLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.DataProxyService/GetData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.DataProxyService", "GetData"));
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoRequest {}
/// See the OpenID Connect spec at <https://openid.net/specs/openid-connect-core-1_0.html#UserInfoResponse> for more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoResponse {
    /// Locally unique and never reassigned identifier within the Issuer for the End-User, which is intended to be consumed
    /// by the Client.
    #[prost(string, tag = "1")]
    pub subject: ::prost::alloc::string::String,
    /// Full name
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Shorthand name by which the End-User wishes to be referred to
    #[prost(string, tag = "3")]
    pub preferred_username: ::prost::alloc::string::String,
    /// Given name(s) or first name(s)
    #[prost(string, tag = "4")]
    pub given_name: ::prost::alloc::string::String,
    /// Surname(s) or last name(s)
    #[prost(string, tag = "5")]
    pub family_name: ::prost::alloc::string::String,
    /// Preferred e-mail address
    #[prost(string, tag = "6")]
    pub email: ::prost::alloc::string::String,
    /// Profile picture URL
    #[prost(string, tag = "7")]
    pub picture: ::prost::alloc::string::String,
    /// Additional claims
    #[prost(message, optional, tag = "8")]
    pub additional_claims: ::core::option::Option<::prost_types::Struct>,
}
/// Generated client implementations.
pub mod identity_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// IdentityService defines an RPC Service that interacts with user/app identities.
    #[derive(Debug, Clone)]
    pub struct IdentityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IdentityServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IdentityServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> IdentityServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            IdentityServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Retrieves user information about the currently logged in user.
        pub async fn user_info(
            &mut self,
            request: impl tonic::IntoRequest<super::UserInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UserInfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.IdentityService/UserInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.IdentityService", "UserInfo"));
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuth2MetadataRequest {}
/// OAuth2MetadataResponse defines an RFC-Compliant response for /.well-known/oauth-authorization-server metadata
/// as defined in <https://tools.ietf.org/html/rfc8414>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuth2MetadataResponse {
    /// Defines the issuer string in all JWT tokens this server issues. The issuer can be admin itself or an external
    /// issuer.
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    /// URL of the authorization server's authorization endpoint \[RFC6749\]. This is REQUIRED unless no grant types are
    /// supported that use the authorization endpoint.
    #[prost(string, tag = "2")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    /// URL of the authorization server's token endpoint \[RFC6749\].
    #[prost(string, tag = "3")]
    pub token_endpoint: ::prost::alloc::string::String,
    /// Array containing a list of the OAuth 2.0 response_type values that this authorization server supports.
    #[prost(string, repeated, tag = "4")]
    pub response_types_supported: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// JSON array containing a list of the OAuth 2.0 \[RFC6749\] scope values that this authorization server supports.
    #[prost(string, repeated, tag = "5")]
    pub scopes_supported: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// JSON array containing a list of client authentication methods supported by this token endpoint.
    #[prost(string, repeated, tag = "6")]
    pub token_endpoint_auth_methods_supported: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// URL of the authorization server's JWK Set \[JWK\] document. The referenced document contains the signing key(s) the
    /// client uses to validate signatures from the authorization server.
    #[prost(string, tag = "7")]
    pub jwks_uri: ::prost::alloc::string::String,
    /// JSON array containing a list of Proof Key for Code Exchange (PKCE) \[RFC7636\] code challenge methods supported by
    /// this authorization server.
    #[prost(string, repeated, tag = "8")]
    pub code_challenge_methods_supported: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// JSON array containing a list of the OAuth 2.0 grant type values that this authorization server supports.
    #[prost(string, repeated, tag = "9")]
    pub grant_types_supported: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL of the authorization server's device authorization endpoint, as defined in Section 3.1 of \[RFC8628\]
    #[prost(string, tag = "10")]
    pub device_authorization_endpoint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicClientAuthConfigRequest {}
/// FlyteClientResponse encapsulates public information that flyte clients (CLIs... etc.) can use to authenticate users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicClientAuthConfigResponse {
    /// client_id to use when initiating OAuth2 authorization requests.
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// redirect uri to use when initiating OAuth2 authorization requests.
    #[prost(string, tag = "2")]
    pub redirect_uri: ::prost::alloc::string::String,
    /// scopes to request when initiating OAuth2 authorization requests.
    #[prost(string, repeated, tag = "3")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Authorization Header to use when passing Access Tokens to the server. If not provided, the client should use the
    /// default http `Authorization` header.
    #[prost(string, tag = "4")]
    pub authorization_metadata_key: ::prost::alloc::string::String,
    /// ServiceHttpEndpoint points to the http endpoint for the backend. If empty, clients can assume the endpoint used
    /// to configure the gRPC connection can be used for the http one respecting the insecure flag to choose between
    /// SSL or no SSL connections.
    #[prost(string, tag = "5")]
    pub service_http_endpoint: ::prost::alloc::string::String,
    /// audience to use when initiating OAuth2 authorization requests.
    #[prost(string, tag = "6")]
    pub audience: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod auth_metadata_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The following defines an RPC service that is also served over HTTP via grpc-gateway.
    /// Standard response codes for both are defined here: https://github.com/grpc-ecosystem/grpc-gateway/blob/master/runtime/errors.go
    /// RPCs defined in this service must be anonymously accessible.
    #[derive(Debug, Clone)]
    pub struct AuthMetadataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthMetadataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AuthMetadataServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AuthMetadataServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AuthMetadataServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Anonymously accessible. Retrieves local or external oauth authorization server metadata.
        pub async fn get_o_auth2_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::OAuth2MetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OAuth2MetadataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AuthMetadataService/GetOAuth2Metadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AuthMetadataService",
                        "GetOAuth2Metadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Anonymously accessible. Retrieves the client information clients should use when initiating OAuth2 authorization
        /// requests.
        pub async fn get_public_client_config(
            &mut self,
            request: impl tonic::IntoRequest<super::PublicClientAuthConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PublicClientAuthConfigResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AuthMetadataService/GetPublicClientConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AuthMetadataService",
                        "GetPublicClientConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod sync_agent_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// SyncAgentService defines an RPC Service that allows propeller to send the request to the agent server synchronously.
    #[derive(Debug, Clone)]
    pub struct SyncAgentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SyncAgentServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SyncAgentServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SyncAgentServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SyncAgentServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// ExecuteTaskSync streams the create request and inputs to the agent service and streams the outputs back.
        pub async fn execute_task_sync(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::admin::ExecuteTaskSyncRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::admin::ExecuteTaskSyncResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.SyncAgentService/ExecuteTaskSync",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.SyncAgentService",
                        "ExecuteTaskSync",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod async_agent_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// AsyncAgentService defines an RPC Service that allows propeller to send the request to the agent server asynchronously.
    #[derive(Debug, Clone)]
    pub struct AsyncAgentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AsyncAgentServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AsyncAgentServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AsyncAgentServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AsyncAgentServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// CreateTask sends a task create request to the agent service.
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::CreateTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::CreateTaskResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AsyncAgentService/CreateTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AsyncAgentService", "CreateTask"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get job status.
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::GetTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::GetTaskResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AsyncAgentService/GetTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AsyncAgentService", "GetTask"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete the task resource.
        pub async fn delete_task(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::DeleteTaskRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::DeleteTaskResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AsyncAgentService/DeleteTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AsyncAgentService", "DeleteTask"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetTaskMetrics returns one or more task execution metrics, if available.
        ///
        /// Errors include
        ///  * OutOfRange if metrics are not available for the specified task time range
        ///  * various other errors
        pub async fn get_task_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::GetTaskMetricsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::GetTaskMetricsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AsyncAgentService/GetTaskMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AsyncAgentService",
                        "GetTaskMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetTaskLogs returns task execution logs, if available.
        pub async fn get_task_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::GetTaskLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::admin::GetTaskLogsResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AsyncAgentService/GetTaskLogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AsyncAgentService", "GetTaskLogs"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod agent_metadata_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// AgentMetadataService defines an RPC service that is also served over HTTP via grpc-gateway.
    /// This service allows propeller or users to get the metadata of agents.
    #[derive(Debug, Clone)]
    pub struct AgentMetadataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AgentMetadataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AgentMetadataServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AgentMetadataServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AgentMetadataServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Fetch a :ref:`ref_flyteidl.admin.Agent` definition.
        pub async fn get_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::GetAgentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::GetAgentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AgentMetadataService/GetAgent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AgentMetadataService", "GetAgent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.Agent` definitions.
        pub async fn list_agents(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ListAgentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ListAgentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AgentMetadataService/ListAgents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AgentMetadataService",
                        "ListAgents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The following defines an RPC service that is also served over HTTP via grpc-gateway.
    /// Standard response codes for both are defined here: https://github.com/grpc-ecosystem/grpc-gateway/blob/master/runtime/errors.go
    #[derive(Debug, Clone)]
    pub struct AdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AdminServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AdminServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AdminServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AdminServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Create and upload a :ref:`ref_flyteidl.admin.Task` definition
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::TaskCreateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::TaskCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/CreateTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.AdminService", "CreateTask"));
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a :ref:`ref_flyteidl.admin.Task` definition.
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ObjectGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::Task>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.AdminService", "GetTask"));
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.NamedEntityIdentifier` of task objects.
        pub async fn list_task_ids(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::NamedEntityIdentifierListRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NamedEntityIdentifierList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListTaskIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.AdminService", "ListTaskIds"));
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.Task` definitions.
        pub async fn list_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ResourceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::TaskList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListTasks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.AdminService", "ListTasks"));
            self.inner.unary(req, path, codec).await
        }
        /// Create and upload a :ref:`ref_flyteidl.admin.Workflow` definition
        pub async fn create_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::WorkflowCreateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::WorkflowCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/CreateWorkflow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "CreateWorkflow"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a :ref:`ref_flyteidl.admin.Workflow` definition.
        pub async fn get_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ObjectGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::Workflow>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetWorkflow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.AdminService", "GetWorkflow"));
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.NamedEntityIdentifier` of workflow objects.
        pub async fn list_workflow_ids(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::NamedEntityIdentifierListRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NamedEntityIdentifierList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListWorkflowIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "ListWorkflowIds"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.Workflow` definitions.
        pub async fn list_workflows(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ResourceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::WorkflowList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListWorkflows",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "ListWorkflows"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create and upload a :ref:`ref_flyteidl.admin.LaunchPlan` definition
        pub async fn create_launch_plan(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::LaunchPlanCreateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::LaunchPlanCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/CreateLaunchPlan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "CreateLaunchPlan"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a :ref:`ref_flyteidl.admin.LaunchPlan` definition.
        pub async fn get_launch_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ObjectGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::LaunchPlan>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetLaunchPlan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "GetLaunchPlan"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch the active version of a :ref:`ref_flyteidl.admin.LaunchPlan`.
        pub async fn get_active_launch_plan(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ActiveLaunchPlanRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::LaunchPlan>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetActiveLaunchPlan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "GetActiveLaunchPlan",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List active versions of :ref:`ref_flyteidl.admin.LaunchPlan`.
        pub async fn list_active_launch_plans(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ActiveLaunchPlanListRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::LaunchPlanList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListActiveLaunchPlans",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "ListActiveLaunchPlans",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.NamedEntityIdentifier` of launch plan objects.
        pub async fn list_launch_plan_ids(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::NamedEntityIdentifierListRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NamedEntityIdentifierList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListLaunchPlanIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "ListLaunchPlanIds"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.LaunchPlan` definitions.
        pub async fn list_launch_plans(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ResourceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::LaunchPlanList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListLaunchPlans",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "ListLaunchPlans"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the status of a registered :ref:`ref_flyteidl.admin.LaunchPlan`.
        pub async fn update_launch_plan(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::LaunchPlanUpdateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::LaunchPlanUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/UpdateLaunchPlan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "UpdateLaunchPlan"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Triggers the creation of a :ref:`ref_flyteidl.admin.Execution`
        pub async fn create_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ExecutionCreateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ExecutionCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/CreateExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "CreateExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Triggers the creation of an identical :ref:`ref_flyteidl.admin.Execution`
        pub async fn relaunch_execution(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ExecutionRelaunchRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ExecutionCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/RelaunchExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "RelaunchExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Recreates a previously-run workflow execution that will only start executing from the last known failure point.
        /// In Recover mode, users cannot change any input parameters or update the version of the execution.
        /// This is extremely useful to recover from system errors and byzantine faults like - Loss of K8s cluster, bugs in platform or instability, machine failures,
        /// downstream system failures (downstream services), or simply to recover executions that failed because of retry exhaustion and should complete if tried again.
        /// See :ref:`ref_flyteidl.admin.ExecutionRecoverRequest` for more details.
        pub async fn recover_execution(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ExecutionRecoverRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ExecutionCreateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/RecoverExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "RecoverExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a :ref:`ref_flyteidl.admin.Execution`.
        pub async fn get_execution(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::WorkflowExecutionGetRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::Execution>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "GetExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update execution belonging to project domain :ref:`ref_flyteidl.admin.Execution`.
        pub async fn update_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ExecutionUpdateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ExecutionUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/UpdateExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "UpdateExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches input and output data for a :ref:`ref_flyteidl.admin.Execution`.
        pub async fn get_execution_data(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::WorkflowExecutionGetDataRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::WorkflowExecutionGetDataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetExecutionData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "GetExecutionData"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.Execution`.
        pub async fn list_executions(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ResourceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ExecutionList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListExecutions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "ListExecutions"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Terminates an in-progress :ref:`ref_flyteidl.admin.Execution`.
        pub async fn terminate_execution(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ExecutionTerminateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ExecutionTerminateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/TerminateExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "TerminateExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a :ref:`ref_flyteidl.admin.NodeExecution`.
        pub async fn get_node_execution(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::NodeExecutionGetRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NodeExecution>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetNodeExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "GetNodeExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a :ref:`ref_flyteidl.admin.DynamicNodeWorkflowResponse`.
        pub async fn get_dynamic_node_workflow(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::GetDynamicNodeWorkflowRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::DynamicNodeWorkflowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetDynamicNodeWorkflow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "GetDynamicNodeWorkflow",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.NodeExecution`.
        pub async fn list_node_executions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::NodeExecutionListRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NodeExecutionList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListNodeExecutions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "ListNodeExecutions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.NodeExecution` launched by the reference :ref:`ref_flyteidl.admin.TaskExecution`.
        pub async fn list_node_executions_for_task(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::NodeExecutionForTaskListRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NodeExecutionList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListNodeExecutionsForTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "ListNodeExecutionsForTask",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches input and output data for a :ref:`ref_flyteidl.admin.NodeExecution`.
        pub async fn get_node_execution_data(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::NodeExecutionGetDataRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NodeExecutionGetDataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetNodeExecutionData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "GetNodeExecutionData",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Registers a :ref:`ref_flyteidl.admin.Project` with the Flyte deployment.
        pub async fn register_project(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ProjectRegisterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ProjectRegisterResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/RegisterProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "RegisterProject"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing :ref:`ref_flyteidl.admin.Project`
        /// flyteidl.admin.Project should be passed but the domains property should be empty;
        /// it will be ignored in the handler as domains cannot be updated via this API.
        pub async fn update_project(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::Project>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ProjectUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/UpdateProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "UpdateProject"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a :ref:`ref_flyteidl.admin.Project`
        pub async fn get_project(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ProjectGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::Project>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.AdminService", "GetProject"));
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a list of :ref:`ref_flyteidl.admin.Project`
        pub async fn list_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ProjectListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::Projects>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListProjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "ListProjects"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Indicates a :ref:`ref_flyteidl.event.WorkflowExecutionEvent` has occurred.
        pub async fn create_workflow_event(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::WorkflowExecutionEventRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::WorkflowExecutionEventResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/CreateWorkflowEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "CreateWorkflowEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Indicates a :ref:`ref_flyteidl.event.NodeExecutionEvent` has occurred.
        pub async fn create_node_event(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::NodeExecutionEventRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NodeExecutionEventResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/CreateNodeEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "CreateNodeEvent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Indicates a :ref:`ref_flyteidl.event.TaskExecutionEvent` has occurred.
        pub async fn create_task_event(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::TaskExecutionEventRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::TaskExecutionEventResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/CreateTaskEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "CreateTaskEvent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a :ref:`ref_flyteidl.admin.TaskExecution`.
        pub async fn get_task_execution(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::TaskExecutionGetRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::TaskExecution>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetTaskExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "GetTaskExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a list of :ref:`ref_flyteidl.admin.TaskExecution`.
        pub async fn list_task_executions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::TaskExecutionListRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::TaskExecutionList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListTaskExecutions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "ListTaskExecutions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches input and output data for a :ref:`ref_flyteidl.admin.TaskExecution`.
        pub async fn get_task_execution_data(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::TaskExecutionGetDataRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::TaskExecutionGetDataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetTaskExecutionData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "GetTaskExecutionData",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or updates custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` for a project and domain.
        pub async fn update_project_domain_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ProjectDomainAttributesUpdateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ProjectDomainAttributesUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/UpdateProjectDomainAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "UpdateProjectDomainAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` for a project and domain.
        pub async fn get_project_domain_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ProjectDomainAttributesGetRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ProjectDomainAttributesGetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetProjectDomainAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "GetProjectDomainAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` for a project and domain.
        pub async fn delete_project_domain_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ProjectDomainAttributesDeleteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ProjectDomainAttributesDeleteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/DeleteProjectDomainAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "DeleteProjectDomainAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or updates custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` at the project level
        pub async fn update_project_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ProjectAttributesUpdateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ProjectAttributesUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/UpdateProjectAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "UpdateProjectAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` for a project and domain.
        pub async fn get_project_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ProjectAttributesGetRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ProjectAttributesGetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetProjectAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "GetProjectAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` for a project and domain.
        pub async fn delete_project_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ProjectAttributesDeleteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ProjectAttributesDeleteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/DeleteProjectAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "DeleteProjectAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or updates custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` for a project, domain and workflow.
        pub async fn update_workflow_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::WorkflowAttributesUpdateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::WorkflowAttributesUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/UpdateWorkflowAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "UpdateWorkflowAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` for a project, domain and workflow.
        pub async fn get_workflow_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::WorkflowAttributesGetRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::WorkflowAttributesGetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetWorkflowAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "GetWorkflowAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` for a project, domain and workflow.
        pub async fn delete_workflow_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::WorkflowAttributesDeleteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::WorkflowAttributesDeleteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/DeleteWorkflowAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "DeleteWorkflowAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists custom :ref:`ref_flyteidl.admin.MatchableAttributesConfiguration` for a specific resource type.
        pub async fn list_matchable_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::ListMatchableAttributesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::ListMatchableAttributesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListMatchableAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "ListMatchableAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a list of :ref:`ref_flyteidl.admin.NamedEntity` objects.
        pub async fn list_named_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::NamedEntityListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NamedEntityList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListNamedEntities",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "ListNamedEntities"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a :ref:`ref_flyteidl.admin.NamedEntity` object.
        pub async fn get_named_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::NamedEntityGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NamedEntity>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetNamedEntity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "GetNamedEntity"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a :ref:`ref_flyteidl.admin.NamedEntity` object.
        pub async fn update_named_entity(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::NamedEntityUpdateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::NamedEntityUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/UpdateNamedEntity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flyteidl.service.AdminService", "UpdateNamedEntity"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::GetVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::GetVersionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.service.AdminService", "GetVersion"));
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a :ref:`ref_flyteidl.admin.DescriptionEntity` object.
        pub async fn get_description_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::super::admin::ObjectGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::DescriptionEntity>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetDescriptionEntity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "GetDescriptionEntity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a list of :ref:`ref_flyteidl.admin.DescriptionEntity` definitions.
        pub async fn list_description_entities(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::DescriptionEntityListRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::DescriptionEntityList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/ListDescriptionEntities",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "ListDescriptionEntities",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches runtime metrics for a :ref:`ref_flyteidl.admin.Execution`.
        pub async fn get_execution_metrics(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::admin::WorkflowExecutionGetMetricsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::admin::WorkflowExecutionGetMetricsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flyteidl.service.AdminService/GetExecutionMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.service.AdminService",
                        "GetExecutionMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
