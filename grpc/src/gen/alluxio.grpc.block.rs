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
/// The data chunk.
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    #[prost(bytes="vec", optional, tag="1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Generated client implementations.
pub mod block_worker_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
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
    }
}
