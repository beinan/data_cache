use futures::stream;
use tonic::{metadata::MetadataValue, transport::Channel, Request};

use alluxio::grpc::block::block_worker_client::BlockWorkerClient;
use alluxio::grpc::block::ReadRequest;

pub mod alluxio {
    pub mod grpc {
        pub mod block {
            tonic::include_proto!("alluxio.grpc.block");
        }
    }
    pub mod proto {
        pub mod dataserver {
            tonic::include_proto!("alluxio.proto.dataserver");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:29999").connect().await?;
    let token: MetadataValue<_> = "af28ed5a-dca6-11ec-9d64-0242ac120002".parse()?;
    let mut client = BlockWorkerClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("channel-id", token.clone());
        Ok(req)
    });
    println!("*** read block ***");
    let mut blocks = vec![];
    blocks.push(ReadRequest {
        block_id: Some(100),
        chunk_size: Some(1000),
        length: Some(50),
        offset: Some(5),
        offset_received: None,
        open_ufs_block_options: None,
        position_short: None,
        promote: None
    });
    let request = Request::new(stream::iter(blocks));
    let response = client
        .read_block(request)
        .await?;
    println!("Response {:?}", response);
    let mut stream = response.into_inner();
    while let Ok(Some(r)) = stream.message().await {
        println!("RESPONSE streaming = {:?}", r);
    }

    Ok(())
}
