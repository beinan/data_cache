use alluxio_grpc::grpc_client::Client;
use structopt::StructOpt;

use crate::file::alluxio_file_system;
use tabled::{Table, Tabled};

#[derive(StructOpt, Debug)]
pub struct StatOptions {
    path: String,
}

#[derive(Tabled)]
struct FileRow {
    id: i64,
    name: String,
    ufs_path: String,
    length: i64,
}

pub async fn stat(client: Client, options: &StatOptions) -> Result<String, String> {
    match alluxio_file_system::create(client)
        .unwrap()
        .get_status(String::from(options.path.clone()))
        .await
    {
        Ok(file_info) => {
            match file_info {
                Some(info) => {
                    let file_row = FileRow {
                        id: info.file_id.unwrap(),
                        name: info.name.unwrap(),
                        ufs_path: info.ufs_path.unwrap(),
                        length: info.length.unwrap(),
                    };
                    println!("{}", Table::new(vec![file_row]).to_string());
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
