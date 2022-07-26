use structopt::StructOpt;

use alluxio_grpc::grpc_client::Client;
use crate::cmds::ls::ListOptions;

mod ls;

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
    Location,
}

impl Opt {
    pub async fn execute(&self, client : Client) -> Result<String, String> {
        match self {
            Opt::FS(FsCommand::List(options)) => crate::cmds::ls::ls(client, options).await,
            _ => Err(String::from("Unimplemented cmds"))
        }
    }
}

