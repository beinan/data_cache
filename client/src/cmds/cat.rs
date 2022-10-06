use std::io::Cursor;

use crate::file::common::{AlluxioFileInStream, InputStream, LocalFirstPolicy};
use alluxio_common::exception::AlluxioException;
use alluxio_grpc::grpc_client::Client;
use structopt::StructOpt;

use crate::file::alluxio_file_system;
// use tabled::{Table, Tabled};

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
                        println!("File {} does not exit", options.path);
                    } else {
                        let mut buffer: Cursor<Vec<u8>> = Cursor::new(vec![]);
                        let file_in_stream: Result<
                            AlluxioFileInStream<LocalFirstPolicy>,
                            AlluxioException,
                        > = file_system.open_file(options.path.clone()).await;
                        match file_in_stream {
                            Ok(mut in_stream) => {
                                let mut read_result =
                                    in_stream.read(&mut buffer, 0, info.length()).await;
                                let is_ok = read_result.as_ref().is_ok();
                                assert!(is_ok);
                                let mut read = read_result.unwrap();
                                while read != -1 {
                                    read_result =
                                        in_stream.read(&mut buffer, 0, info.length()).await;
                                    assert!(is_ok);
                                    read = read_result.unwrap();
                                }
                                println!(
                                    "{:?}",
                                    String::from_utf8_lossy(buffer.get_ref().as_slice())
                                );
                            }
                            Err(err) => return Err(err.to_string()),
                        }
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
