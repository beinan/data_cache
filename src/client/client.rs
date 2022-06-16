use futures::stream;
use tonic::{metadata::MetadataValue, transport::Channel, Request};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use alluxio::grpc::block::block_worker_client::BlockWorkerClient;
use alluxio::grpc::block::ReadRequest;

use alluxio::grpc::file::file_system_master_client_service_client::FileSystemMasterClientServiceClient;
use alluxio::grpc::file::{CreateFilePOptions, CreateFilePRequest};


use alluxio::grpc::sasl::sasl_authentication_service_client::SaslAuthenticationServiceClient;
use alluxio::grpc::sasl::{SaslMessage,ChannelAuthenticationScheme, SaslMessageType};

pub mod alluxio {
    pub mod grpc {
        tonic::include_proto!("alluxio.grpc");
        pub mod block {
            tonic::include_proto!("alluxio.grpc.block");
        }
        pub mod sasl {
            tonic::include_proto!("alluxio.grpc.sasl");
        }
        pub mod file {
            tonic::include_proto!("alluxio.grpc.file");
        }
        pub mod fscommon {
            tonic::include_proto!("alluxio.grpc.fscommon");
        }
    }
    pub mod proto {
        pub mod dataserver {
            tonic::include_proto!("alluxio.proto.dataserver");
        }
        pub mod journal {
            tonic::include_proto!("alluxio.proto.journal");
        }
        pub mod shared {
            tonic::include_proto!("alluxio.proto.shared");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_channel = Channel::from_static("http://[::1]:29999").connect().await?;

    let token: MetadataValue<_> = "af28ed5a-dca6-11ec-9d64-0242ac120002".parse()?;
    let mut auth_client = SaslAuthenticationServiceClient::with_interceptor(auth_channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("channel-id", token.clone());
        Ok(req)
    });

    let (tx, rx) = mpsc::channel(4);
    //let mut sasl_messages = vec![];
    tx.send(SaslMessage{
        authentication_scheme: Some(ChannelAuthenticationScheme::Simple as i32),
        channel_ref: None,
        client_id: Some(String::from("af28ed5a-dca6-11ec-9d64-0242ac120002")),
        message: Some(String::from("beinan\0beinan\0beinan").as_bytes().to_vec()),
        message_type: Some(SaslMessageType::Challenge as i32)
    }).await.unwrap();
    let auth_request = Request::new(ReceiverStream::new(rx));
    let response = auth_client
        .authenticate(auth_request)
        .await?;
    println!("Auth Response {:?}", response);
    let mut stream = response.into_inner();
    while let Ok(Some(r)) = stream.message().await {
        println!("auth response streaming = {:?}", r);

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

    }



    Ok(())
}
