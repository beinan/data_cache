use structopt::StructOpt;

use alluxio_grpc::grpc_client::Client;
use crate::cmds::ls::ListOptions;
use crate::cmds::location::LocationOptions;

mod ls;
mod location;

#[derive(StructOpt, Debug)]
#[structopt(name = "alluxio")]
/// The native alluxio client
pub enum Opt {
    /// File System Operations
    FS (FsCommand),
    #[structopt(help = "prints the configured value for the given key")]
    GetConf {
        #[structopt(short)]
        master: bool,
        #[structopt(short)]
        source: bool,
    },
}

#[derive(StructOpt, Debug)]
pub enum FsCommand {
    #[structopt(name = "ls")]
    List (ListOptions),
    Location (LocationOptions),
}

impl Opt {
    pub async fn execute(&self, client : Client) -> Result<String, String> {
        match self {
            Opt::FS(FsCommand::List(options)) => crate::cmds::ls::ls(client, options).await,
            Opt::FS(FsCommand::Location(options)) => crate::cmds::location::location(client, options).await,
            _ => Err(String::from("Unimplemented cmds"))
        }
    }
}

