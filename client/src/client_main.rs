#![feature(async_closure)]

use structopt::StructOpt;

use alluxio_common::settings::Settings;
use alluxio_grpc::grpc_client::Client;

mod cmds;
mod file;

#[tokio::main]
async fn main() -> Result<(), String> {
    let cmd = cmds::Opt::from_args();
    match Settings::new() {
        Ok(settings) => {
            match Client::connect_with_simple_auth(settings.master, 19998, |client: Client| {
                cmd.execute(client)
            })
            .await
            {
                Ok(result) => Ok(()),
                Err(err) => Err(err),
            }
        }
        Err(config_error) => Err(config_error.to_string()),
    }
}
