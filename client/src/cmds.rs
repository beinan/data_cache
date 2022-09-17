use structopt::StructOpt;

mod cat;
mod location;
mod ls;
mod stat;

use alluxio_grpc::grpc_client::Client;
use cat::{cat, CatOptions};
use location::{location, LocationOptions};
use ls::{ls, ListOptions};
use stat::{stat, StatOptions};

#[derive(StructOpt, Debug)]
#[structopt(name = "alluxio")]
/// The native alluxio client
pub enum Opt {
    /// File System Operations
    FS(FsCommand),
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
    List(ListOptions),
    #[structopt(name = "location")]
    Location(LocationOptions),
    #[structopt(name = "stat")]
    Stat(StatOptions),
    #[structopt(name = "cat")]
    Cat(CatOptions),
}

impl Opt {
    pub async fn execute(&self, client: Client) -> Result<String, String> {
        match self {
            Opt::FS(FsCommand::List(options)) => ls(client, options).await,
            Opt::FS(FsCommand::Location(options)) => location(client, options).await,
            Opt::FS(FsCommand::Stat(options)) => stat(client, options).await,
            Opt::FS(FsCommand::Cat(options)) => cat(client, options).await,
            _ => Err(String::from("Unimplemented cmds")),
        }
    }
}
