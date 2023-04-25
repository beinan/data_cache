/// The check request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRequest {
}
/// The check response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResponse {
}
/// The data chunk.
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    #[prost(bytes="vec", optional, tag="1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// The read request.
/// next available id: 9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub offset: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub length: ::core::option::Option<i64>,
    /// Whether the block should be promoted before reading
    #[prost(bool, optional, tag="4")]
    pub promote: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="5")]
    pub chunk_size: ::core::option::Option<i64>,
    /// This is only set for UFS block read.
    #[prost(message, optional, tag="6")]
    pub open_ufs_block_options: ::core::option::Option<super::super::proto::dataserver::OpenUfsBlockOptions>,
    /// Read receipt
    #[prost(int64, optional, tag="7")]
    pub offset_received: ::core::option::Option<i64>,
    /// Is position read to a small buffer
    #[prost(bool, optional, tag="8")]
    pub position_short: ::core::option::Option<bool>,
}
/// The read response.
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    #[prost(message, optional, tag="1")]
    pub chunk: ::core::option::Option<Chunk>,
}
/// The write request command.
/// next available id: 11
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequestCommand {
    #[prost(enumeration="RequestType", optional, tag="1")]
    pub r#type: ::core::option::Option<i32>,
    /// The block ID or UFS file ID.
    #[prost(int64, optional, tag="2")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub offset: ::core::option::Option<i64>,
    /// This is only applicable for block write.
    #[prost(int32, optional, tag="4")]
    pub tier: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="5")]
    pub flush: ::core::option::Option<bool>,
    /// Cancel, close and error will be handled by standard gRPC stream APIs.
    #[prost(message, optional, tag="6")]
    pub create_ufs_file_options: ::core::option::Option<super::super::proto::dataserver::CreateUfsFileOptions>,
    #[prost(message, optional, tag="7")]
    pub create_ufs_block_options: ::core::option::Option<super::super::proto::dataserver::CreateUfsBlockOptions>,
    #[prost(string, optional, tag="8")]
    pub medium_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="9")]
    pub pin_on_create: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="10")]
    pub space_to_reserve: ::core::option::Option<i64>,
}
/// The write request.
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(oneof="write_request::Value", tags="1, 2")]
    pub value: ::core::option::Option<write_request::Value>,
}
/// Nested message and enum types in `WriteRequest`.
pub mod write_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="1")]
        Command(super::WriteRequestCommand),
        #[prost(message, tag="2")]
        Chunk(super::Chunk),
    }
}
/// The write response.
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {
    /// Errors will be handled by standard gRPC stream APIs.
    #[prost(int64, optional, tag="1")]
    pub offset: ::core::option::Option<i64>,
}
/// Request for caching a block asynchronously
/// Deprecated and will be removed in v3.0
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncCacheRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    /// TODO(calvin): source host and port should be replace with WorkerNetAddress
    #[prost(string, optional, tag="2")]
    pub source_host: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub source_port: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub open_ufs_block_options: ::core::option::Option<super::super::proto::dataserver::OpenUfsBlockOptions>,
    #[prost(int64, optional, tag="5")]
    pub length: ::core::option::Option<i64>,
}
/// Request for caching a block synchronously/asynchronously
/// next available id: 7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    /// TODO(calvin): source host and port should be replace with WorkerNetAddress
    #[prost(string, optional, tag="2")]
    pub source_host: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub source_port: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub open_ufs_block_options: ::core::option::Option<super::super::proto::dataserver::OpenUfsBlockOptions>,
    #[prost(int64, optional, tag="5")]
    pub length: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="6")]
    pub r#async: ::core::option::Option<bool>,
}
/// Request for load a block into alluxio
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadRequest {
    #[prost(message, repeated, tag="1")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
    #[prost(message, required, tag="2")]
    pub options: UfsReadOptions,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UfsReadOptions {
    #[prost(string, required, tag="1")]
    pub tag: ::prost::alloc::string::String,
    /// is position short or not, used for HDFS performance optimization.
    /// When the client buffer size is large ( > 2MB) and reads are guaranteed to be somewhat
    /// sequential, the `pread` API to HDFS is not as efficient as simple `read`.
    /// We introduce a heuristic to choose which API to use.
    #[prost(bool, required, tag="2")]
    pub position_short: bool,
    #[prost(int64, optional, tag="3")]
    pub bandwidth: ::core::option::Option<i64>,
    #[prost(string, optional, tag="4")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(int64, required, tag="1")]
    pub block_id: i64,
    /// The block length.
    #[prost(int64, required, tag="2")]
    pub length: i64,
    #[prost(string, optional, tag="3")]
    pub ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    /// The offset of the block in within ufs the file.
    #[prost(int64, optional, tag="4")]
    pub offset_in_file: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="5")]
    pub mount_id: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadResponse {
    #[prost(enumeration="TaskStatus", required, tag="1")]
    pub status: i32,
    #[prost(message, repeated, tag="2")]
    pub block_status: ::prost::alloc::vec::Vec<BlockStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreeWorkerRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreeWorkerResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockStatus {
    #[prost(message, required, tag="1")]
    pub block: Block,
    /// The status code, which should be an enum value of \[google.rpc.Code][google.rpc.Code\].
    #[prost(int32, required, tag="2")]
    pub code: i32,
    /// A developer-facing error message
    #[prost(string, optional, tag="3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="4")]
    pub retryable: ::core::option::Option<bool>,
}
/// Response for an async cache request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncCacheResponse {
}
/// Response for an async cache request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheResponse {
}
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenLocalBlockRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="2")]
    pub promote: ::core::option::Option<bool>,
}
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenLocalBlockResponse {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// next available id: 9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocalBlockRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="3")]
    pub tier: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="4")]
    pub space_to_reserve: ::core::option::Option<i64>,
    /// If set, only reserve space for the block.
    #[prost(bool, optional, tag="5")]
    pub only_reserve_space: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="6")]
    pub cleanup_on_failure: ::core::option::Option<bool>,
    #[prost(string, optional, tag="7")]
    pub medium_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="8")]
    pub pin_on_create: ::core::option::Option<bool>,
}
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocalBlockResponse {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBlockRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBlockResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveBlockRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag="2")]
    pub medium_type: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveBlockResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearMetricsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearMetricsResponse {
}
/// The read/write request type. It can either be an Alluxio block operation or a UFS file operation.
/// next available id: 3
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RequestType {
    AlluxioBlock = 0,
    UfsFile = 1,
    UfsFallbackBlock = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskStatus {
    Success = 0,
    Failure = 1,
    PartialFailure = 2,
}
/// Generated client implementations.
pub mod block_worker_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The block worker service
    #[derive(Debug, Clone)]
    pub struct BlockWorkerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BlockWorkerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BlockWorkerClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BlockWorkerClient<InterceptedService<T, F>>
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
            BlockWorkerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        ///*
        /// Returns the status of the file or directory.
        pub async fn get_status(
            &mut self,
            request: impl tonic::IntoRequest<super::super::file::GetStatusPRequest>,
        ) -> Result<
            tonic::Response<super::super::file::GetStatusPResponse>,
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
                "/alluxio.grpc.block.BlockWorker/GetStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// If the path points to a file, the method returns a singleton with its file information.
        /// If the path points to a directory, the method returns a list with file information for the
        /// directory contents.
        pub async fn list_status(
            &mut self,
            request: impl tonic::IntoRequest<super::super::file::ListStatusPRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::file::ListStatusPResponse>,
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
                "/alluxio.grpc.block.BlockWorker/ListStatus",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn read_block(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ReadRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReadResponse>>,
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
                "/alluxio.grpc.block.BlockWorker/ReadBlock",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn write_block(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::WriteRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::WriteResponse>>,
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
                "/alluxio.grpc.block.BlockWorker/WriteBlock",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Replaces ShortCircuitBlockReadHandler.
        pub async fn open_local_block(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::OpenLocalBlockRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::OpenLocalBlockResponse>>,
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
                "/alluxio.grpc.block.BlockWorker/OpenLocalBlock",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Replaces ShortCircuitBlockWriteHandler.
        pub async fn create_local_block(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::CreateLocalBlockRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CreateLocalBlockResponse>>,
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
                "/alluxio.grpc.block.BlockWorker/CreateLocalBlock",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn async_cache(
            &mut self,
            request: impl tonic::IntoRequest<super::AsyncCacheRequest>,
        ) -> Result<tonic::Response<super::AsyncCacheResponse>, tonic::Status> {
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
                "/alluxio.grpc.block.BlockWorker/AsyncCache",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cache(
            &mut self,
            request: impl tonic::IntoRequest<super::CacheRequest>,
        ) -> Result<tonic::Response<super::CacheResponse>, tonic::Status> {
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
                "/alluxio.grpc.block.BlockWorker/Cache",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn load(
            &mut self,
            request: impl tonic::IntoRequest<super::LoadRequest>,
        ) -> Result<tonic::Response<super::LoadResponse>, tonic::Status> {
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
                "/alluxio.grpc.block.BlockWorker/Load",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_block(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveBlockRequest>,
        ) -> Result<tonic::Response<super::RemoveBlockResponse>, tonic::Status> {
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
                "/alluxio.grpc.block.BlockWorker/RemoveBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn move_block(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveBlockRequest>,
        ) -> Result<tonic::Response<super::MoveBlockResponse>, tonic::Status> {
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
                "/alluxio.grpc.block.BlockWorker/MoveBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TODO(lu) Move to metrics worker
        pub async fn clear_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearMetricsRequest>,
        ) -> Result<tonic::Response<super::ClearMetricsResponse>, tonic::Status> {
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
                "/alluxio.grpc.block.BlockWorker/ClearMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn free_worker(
            &mut self,
            request: impl tonic::IntoRequest<super::FreeWorkerRequest>,
        ) -> Result<tonic::Response<super::FreeWorkerResponse>, tonic::Status> {
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
                "/alluxio.grpc.block.BlockWorker/FreeWorker",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
