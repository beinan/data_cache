use alluxio_grpc::grpc_client::Client;
use structopt::StructOpt;

use crate::file::alluxio_file_system;

#[derive(StructOpt, Debug)]
pub struct LocationOptions {
    path: String,
}

pub async fn location(client: Client, options: &LocationOptions) -> Result<String, String> {
    match alluxio_file_system::create(client)
        .unwrap()
        .get_status(String::from(options.path.clone()))
        .await
    {
        Ok(file_info) => {
            println!("get status {:#?}", file_info.unwrap());
            Ok(String::from("ok"))
        }
        Err(err) => {
            println!("List status error {:?}", err.to_string());
            Err(err.to_string())
        }
    }
}
