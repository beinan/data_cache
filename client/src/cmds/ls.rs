use structopt::StructOpt;
use alluxio_grpc::grpc_client::Client;

#[derive(StructOpt, Debug)]
pub struct ListOptions {
    #[structopt(short)]
    recursive: bool,
    path: String,
}

pub async fn ls(client : Client, options: &ListOptions) -> Result<String, String> {
    println!("List Status start");
    match client.master_client().unwrap().listStatus(String::from(options.path.clone())).await {
        Ok(fileInfos) => {
            for file in fileInfos {
                println!("{:?}", file);
            }
            Ok(String::from("ok"))
        },
        Err(err) => {
            println!("List status error {:?}", err.to_string());
            Err(err.to_string())
        }
    }
}