use std::fs::File;
use structopt::StructOpt;
use alluxio_grpc::grpc_client::Client;
use alluxio_grpc::grpc::file::FileInfo;

use tabled::{Tabled, Table};

#[derive(StructOpt, Debug)]
pub struct LocationOptions {
    path: String,
}

pub async fn location(client : Client, options: &LocationOptions) -> Result<String, String> {
    match client.master_client().unwrap().getStatus(String::from(options.path.clone())).await {
        Ok(file_info) => {
            println!("get status {:#?}", file_info.unwrap());
            Ok(String::from("ok"))
        },
        Err(err) => {
            println!("List status error {:?}", err.to_string());
            Err(err.to_string())
        }
    }
}