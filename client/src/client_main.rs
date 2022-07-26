use std::fmt::Error;
use alluxio_common::settings::Settings;

use alluxio_grpc::grpc_client::Client;

#[tokio::main]
async fn main() -> Result<(), String> {
    match Settings::new() {
        Ok(settings) => {
            match Client::connect_with_simple_auth(settings.master, ls).await {
                Ok(result) => Ok(()),
                Err(err) => Err(err)
            }

        },
        Err(configError) => Err(configError.to_string()),
    }
}

async fn ls (client : Client) -> Result<String, String> {
    println!("List Status start");
    match client.master_client().unwrap().listStatus(String::from("/")).await {
        Ok(vec) => Ok(String::from("ok")),
        Err(err) => {
            println!("List status error {:?}", err.to_string());
            Err(err.to_string())
        }
    }
}