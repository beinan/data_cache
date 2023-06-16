use futures::Future;
use tokio::sync::mpsc;
use tonic::transport::Channel;

use crate::auth::{AuthClient, AuthInterceptor};

#[derive(Debug)]
pub struct Client {
    pub channel: Channel,
    pub interceptor: AuthInterceptor,
}

impl Client {
    pub async fn connect_with_simple_auth<F, Fut, T>(
        server_addr: String,
        server_port: i32,
        f: F,
    ) -> Result<T, String>
    where
        F: FnOnce(Client) -> Fut,
        Fut: Future<Output = T>,
    {
        let address = format!("http://{}:{}", server_addr, server_port);
        match Channel::from_static(Box::leak(address.into_boxed_str()))
            .connect()
            .await
        {
            Ok(channel) => {
                let interceptor = AuthInterceptor::new();
                let client = Client {
                    channel,
                    interceptor,
                };
                let (mut tx, mut rx) = mpsc::channel(4);
                let mut sasl_client = client.sasl_client().unwrap();
                let mut handler = tokio::spawn(async move {
                    sasl_client.send_auth_request(tx).await;
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
}
