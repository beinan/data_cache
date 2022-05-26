use tonic::{transport::Server, Request, Response, Status};

use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;

use alluxio::grpc::block::block_worker_server::{BlockWorker, BlockWorkerServer};
use alluxio::grpc::block::{ReadRequest, ReadResponse, Chunk};

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

#[derive(Debug, Default)]
pub struct DefaultBlockWorker {}

#[tonic::async_trait]
impl BlockWorker for DefaultBlockWorker {
    type ReadBlockStream = ReceiverStream<Result<ReadResponse, Status>>;
    async fn read_block(
        &self,
        request: Request<tonic::Streaming<ReadRequest>>, // Accept request of type HelloRequest
    ) -> Result<Response<Self::ReadBlockStream>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            let mut buffer : Vec<u8> = Vec::new();
            buffer.push(b'a');
            let reply = ReadResponse {
                chunk: Some(Chunk {
                    data: Some(buffer)
                })
            };
            tx.send(Ok(reply)).await.unwrap();
            println!(" /// done sending");
        });


        Ok(Response::new(ReceiverStream::new(rx))) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let worker = DefaultBlockWorker::default();

    Server::builder()
        .add_service(BlockWorkerServer::new(worker))
        .serve(addr)
        .await?;

    Ok(())
}