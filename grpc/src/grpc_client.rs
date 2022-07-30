use futures::Future;
use futures_util::SinkExt;
use std::{thread, time};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::service::interceptor::InterceptedService;
use tonic::service::Interceptor;
use tonic::{transport::Channel, Request};

use crate::alluxio::grpc::file::file_system_master_client_service_client::FileSystemMasterClientServiceClient;
use crate::alluxio::grpc::file::{ListStatusPOptions, ListStatusPRequest};
use crate::alluxio::grpc::sasl::sasl_authentication_service_client::SaslAuthenticationServiceClient;
use crate::alluxio::grpc::sasl::{ChannelAuthenticationScheme, SaslMessage, SaslMessageType};
use crate::auth::{AuthClient, AuthInterceptor};
use crate::file_system_master::MasterClient;

use tonic::metadata::{Ascii, AsciiMetadataValue, MetadataValue};
use tonic::transport::Error;
use uuid::Uuid;

#[derive(Debug)]
pub struct Client {
    channel: Channel,
    interceptor: AuthInterceptor,
}

impl Client {
    pub async fn connect_with_simple_auth<F, Fut, T>(server_addr: String, f: F) -> Result<T, String>
    where
        F: Fn(Client) -> Fut,
        Fut: Future<Output = T>,
    {
        match Channel::from_shared(server_addr).unwrap().connect().await {
            Ok(channel) => {
                let interceptor = AuthInterceptor::new();
                let client = Client {
                    channel,
                    interceptor,
                };
                let (mut tx, mut rx) = mpsc::channel(4);
                let mut sasl_client = client.sasl_client().unwrap();
                let mut handler = tokio::spawn(async move {
                    sasl_client.sendAuthRequest(tx).await;
                });
                let sasl = rx.recv().await;
                let result = f(client);

                //handler.join().expect("The auth thread has panicked");
                return Ok(result.await);
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn sasl_client(&self) -> Result<AuthClient, &'static str> {
        return Ok(AuthClient::create(self.channel.clone(), self.interceptor.clone()).unwrap());
    }

    pub fn master_client(&self) -> Result<MasterClient, &'static str> {
        return Ok(MasterClient::create(self.channel.clone(), self.interceptor.clone()).unwrap());
    }
}
