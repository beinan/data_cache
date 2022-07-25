#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaslMessage {
    #[prost(enumeration="SaslMessageType", optional, tag="1")]
    pub message_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="3")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="ChannelAuthenticationScheme", optional, tag="4")]
    pub authentication_scheme: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub channel_ref: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SaslMessageType {
    Challenge = 0,
    Success = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelAuthenticationScheme {
    Nosasl = 0,
    Simple = 1,
    Custom = 2,
}
/// Generated client implementations.
pub mod sasl_authentication_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SaslAuthenticationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SaslAuthenticationServiceClient<tonic::transport::Channel> {
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
    impl<T> SaslAuthenticationServiceClient<T>
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
        ) -> SaslAuthenticationServiceClient<InterceptedService<T, F>>
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
            SaslAuthenticationServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Used to drive Sasl negotiation with clients.
        pub async fn authenticate(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SaslMessage>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SaslMessage>>,
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
                "/alluxio.grpc.sasl.SaslAuthenticationService/authenticate",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
