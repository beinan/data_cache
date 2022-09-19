use std::fs::File;
use structopt::StructOpt;
use alluxio_grpc::{grpc_client::Client, alluxio::grpc::file::FileInfo};



use crate::file::alluxio_file_system;
use tabled::{Table, Tabled};

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

pub async fn ls(client: Client, options: &ListOptions) -> Result<String, String> {
    match alluxio_file_system::create(client)
        .unwrap()
        .list_status(String::from(options.path.clone()))
        .await
    {
        Ok(file_infos) => {
            let rows: Vec<FileRow> = file_infos
                .into_iter()
                .map(|file_info| FileRow {
                    id: file_info.file_id.unwrap(),
                    name: file_info.name.unwrap(),
                    ufs_path: file_info.ufs_path.unwrap(),
                    length: file_info.length.unwrap(),
                })
                .collect();
            println!("{}", Table::new(rows).to_string());
            Ok(String::from("ok"))
        }
        Err(err) => {
            println!("List status error {:?}", err.to_string());
            Err(err.to_string())
        }
    }
}
