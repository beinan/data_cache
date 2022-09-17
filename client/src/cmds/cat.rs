use alluxio_grpc::grpc_client::Client;
use structopt::StructOpt;

use crate::file::alluxio_file_system;
use tabled::{Table, Tabled};

#[derive(StructOpt, Debug)]
pub struct CatOptions {
    path: String,
}

pub async fn cat(client: Client, options: &CatOptions) -> Result<String, String> {
    let mut file_system = alluxio_file_system::create(client).unwrap();
    match file_system
        .get_status(String::from(options.path.clone()))
        .await
    {
        Ok(file_info) => {
            match file_info {
                Some(info) => {
                    if info.folder() {
                        println!("File {} does not exit", options.path)
                    } else {
                        // TODO 完成file_system.open_file(options.path.clone());
                        // file_system.open_file(options.path.clone());
                    }
                }
                None => println!("File {} does not exit", options.path),
            }
            Ok(String::from("ok"))
        }
        Err(err) => {
            println!("Get status error {:?}", err.to_string());
            Err(err.to_string())
        }
    }
}
