///
/// Additional metadata as key-value pairs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyMapMetadata {
    /// Additional metadata as key-value pairs
    #[prost(map = "string, string", tag = "1")]
    pub values: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
/// Metadata for cached outputs, including the source identifier and timestamps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// Source task or workflow identifier
    #[prost(message, optional, tag = "1")]
    pub source_identifier: ::core::option::Option<super::core::Identifier>,
    /// Additional metadata as key-value pairs
    #[prost(message, optional, tag = "2")]
    pub key_map: ::core::option::Option<KeyMapMetadata>,
    /// Creation timestamp
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Last update timestamp
    #[prost(message, optional, tag = "4")]
    pub last_updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
///
/// Represents cached output, either as literals or an URI, with associated metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CachedOutput {
    /// Associated metadata
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(oneof = "cached_output::Output", tags = "1, 2")]
    pub output: ::core::option::Option<cached_output::Output>,
}
/// Nested message and enum types in `CachedOutput`.
pub mod cached_output {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        /// Output literals
        #[prost(message, tag = "1")]
        OutputLiterals(super::super::core::LiteralMap),
        /// URI to output data
        #[prost(string, tag = "2")]
        OutputUri(::prost::alloc::string::String),
    }
}
///
/// Request to retrieve cached data by key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCacheRequest {
    /// Cache key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
///
/// Response with cached data for a given key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCacheResponse {
    /// Cached output
    #[prost(message, optional, tag = "1")]
    pub output: ::core::option::Option<CachedOutput>,
}
///
/// Request to store/update cached data by key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutCacheRequest {
    /// Cache key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Output to cache
    #[prost(message, optional, tag = "2")]
    pub output: ::core::option::Option<CachedOutput>,
    /// Overwrite flag
    #[prost(bool, tag = "3")]
    pub overwrite: bool,
}
///
/// Response message of cache store/update operation.
///
/// Empty, success indicated by no errors
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutCacheResponse {}
///
/// Request to delete cached data by key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCacheRequest {
    /// Cache key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
///
/// Response message of cache deletion operation.
///
/// Empty, success indicated by no errors
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCacheResponse {}
/// A reservation including owner, heartbeat interval, expiration timestamp, and various metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reservation {
    /// The unique ID for the reservation - same as the cache key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The unique ID of the owner for the reservation
    #[prost(string, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
    /// Requested reservation extension heartbeat interval
    #[prost(message, optional, tag = "3")]
    pub heartbeat_interval: ::core::option::Option<::prost_types::Duration>,
    /// Expiration timestamp of this reservation
    #[prost(message, optional, tag = "4")]
    pub expires_at: ::core::option::Option<::prost_types::Timestamp>,
}
///
/// Request to get or extend a reservation for a cache key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrExtendReservationRequest {
    /// The unique ID for the reservation - same as the cache key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The unique ID of the owner for the reservation
    #[prost(string, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
    /// Requested reservation extension heartbeat interval
    #[prost(message, optional, tag = "3")]
    pub heartbeat_interval: ::core::option::Option<::prost_types::Duration>,
}
///
/// Request to get or extend a reservation for a cache key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrExtendReservationResponse {
    /// The reservation that was created or extended
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
///
/// Request to release the reservation for a cache key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseReservationRequest {
    /// The unique ID for the reservation - same as the cache key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The unique ID of the owner for the reservation
    #[prost(string, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
}
///
/// Response message of release reservation operation.
///
/// Empty, success indicated by no errors
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseReservationResponse {}
/// Generated client implementations.
pub mod cache_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    /// CacheService defines operations for cache management including retrieval, storage, and deletion of cached task/workflow outputs.
    #[derive(Debug, Clone)]
    pub struct CacheServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CacheServiceClient<tonic::transport::Channel> {
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
    impl<T> CacheServiceClient<T>
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
        ) -> CacheServiceClient<InterceptedService<T, F>>
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
            CacheServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves cached data by key.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCacheRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCacheResponse>,
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
                "/flyteidl.cacheservice.CacheService/Get",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.cacheservice.CacheService", "Get"));
            self.inner.unary(req, path, codec).await
        }
        /// Stores or updates cached data by key.
        pub async fn put(
            &mut self,
            request: impl tonic::IntoRequest<super::PutCacheRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutCacheResponse>,
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
                "/flyteidl.cacheservice.CacheService/Put",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.cacheservice.CacheService", "Put"));
            self.inner.unary(req, path, codec).await
        }
        /// Deletes cached data by key.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCacheRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCacheResponse>,
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
                "/flyteidl.cacheservice.CacheService/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flyteidl.cacheservice.CacheService", "Delete"));
            self.inner.unary(req, path, codec).await
        }
        /// Get or extend a reservation for a cache key
        pub async fn get_or_extend_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrExtendReservationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrExtendReservationResponse>,
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
                "/flyteidl.cacheservice.CacheService/GetOrExtendReservation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.cacheservice.CacheService",
                        "GetOrExtendReservation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Release the reservation for a cache key
        pub async fn release_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseReservationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReleaseReservationResponse>,
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
                "/flyteidl.cacheservice.CacheService/ReleaseReservation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flyteidl.cacheservice.CacheService",
                        "ReleaseReservation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
