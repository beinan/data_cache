use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;
use tonic::metadata::{Ascii, AsciiMetadataValue, MetadataValue};
use tonic::service::interceptor::InterceptedService;
use tonic::service::Interceptor;
use tonic::{transport::Channel, Request, Status};
use uuid::Uuid;

use crate::alluxio::grpc::sasl::sasl_authentication_service_client::SaslAuthenticationServiceClient;
use crate::alluxio::grpc::sasl::{ChannelAuthenticationScheme, SaslMessage, SaslMessageType};

#[derive(Clone, Debug)]
pub struct AuthInterceptor {
    pub channel_id: String,
    pub token: MetadataValue<Ascii>,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request
            .metadata_mut()
            .insert("channel-id", self.token.clone());
        Ok(request)
    }
}

impl AuthInterceptor {
    pub fn new() -> AuthInterceptor {
        let channel_id = Uuid::new_v4().to_string();
        let token = AsciiMetadataValue::try_from(channel_id.to_string()).unwrap();
        AuthInterceptor { channel_id, token }
    }
}

#[derive(Debug)]
pub struct AuthClient {
    client: SaslAuthenticationServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    channel_id: String,
}

impl AuthClient {
    pub fn create(
        channel: Channel,
        interceptor: AuthInterceptor,
    ) -> Result<AuthClient, &'static str> {
        return Ok(AuthClient {
            channel_id: interceptor.channel_id.clone(),
            client: SaslAuthenticationServiceClient::with_interceptor(channel, interceptor),
        });
    }
    pub async fn send_auth_request(
        &mut self,
        result_tx: Sender<SaslMessage>,
    ) -> Result<(), String> {
        let (mut tx, rx) = mpsc::channel(4);
        tx.send(SaslMessage {
            authentication_scheme: Some(ChannelAuthenticationScheme::Simple as i32),
            channel_ref: None,
            client_id: Some(self.channel_id.clone()),
            message: Some(String::from("beinan\0beinan\0beinan").as_bytes().to_vec()),
            message_type: Some(SaslMessageType::Challenge as i32),
        })
        .await
        .unwrap();
        let auth_request = Request::new(ReceiverStream::new(rx));
        match self.client.authenticate(auth_request).await {
            Ok(response) => {
                // println!("Auth response = {:?}", response);
                let mut stream = response.into_inner();
                while let Ok(Some(r)) = stream.message().await {
                    // println!("RESPONSE streaming = {:?}", r);
                    result_tx.send(r).await;
                }
                return Ok(());
            }
            Err(err) => {
                println!("Auth error = {:?}", err);
                Err(err.to_string())
            }
        }
    }
}
