use std::fmt::Error;

use structopt::StructOpt;

use alluxio_common::settings::Settings;
use lib::grpc_client::Client;

mod cmds;

#[tokio::main]
async fn main() -> Result<(), String> {
    let cmd = cmds::Opt::from_args();
    match Settings::new() {
        Ok(settings) => {
            match Client::connect_with_simple_auth(settings.master, |client: Client| {
                cmd.execute(client)
            })
            .await
            {
                Ok(result) => Ok(()),
                Err(err) => Err(err),
            }
        }
        Err(configError) => Err(configError.to_string()),
    }
}
