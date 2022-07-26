use std::fs::File;
use structopt::StructOpt;
use alluxio_grpc::grpc_client::Client;
use alluxio_grpc::grpc::file::FileInfo;

use tabled::{Tabled, Table};

#[derive(StructOpt, Debug)]
pub struct ListOptions {
    #[structopt(short)]
    recursive: bool,
    path: String,
}

#[derive(Tabled)]
struct FileRow {
    id: i64,
    name: String,
    ufs_path: String,
    length: i64,
}

pub async fn ls(client : Client, options: &ListOptions) -> Result<String, String> {
    match client.master_client().unwrap().listStatus(String::from(options.path.clone())).await {
        Ok(fileInfos) => {
            // for file in fileInfos {
            //     println!("{:?}", file);
            // }
            let rows: Vec<FileRow> = fileInfos.into_iter().map(|fileInfo|FileRow {
                id: fileInfo.file_id.unwrap(),
                name: fileInfo.name.unwrap(),
                ufs_path: fileInfo.ufs_path.unwrap(),
                length: fileInfo.length.unwrap(),
            }).collect();
            println!("{}", Table::new(rows).to_string());
            Ok(String::from("ok"))
        },
        Err(err) => {
            println!("List status error {:?}", err.to_string());
            Err(err.to_string())
        }
    }
}