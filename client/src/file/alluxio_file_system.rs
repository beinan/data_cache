use alluxio_grpc::alluxio::grpc::block::ReadRequest;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;

use alluxio_grpc::alluxio::grpc::file::{
    file_system_master_client_service_client::FileSystemMasterClientServiceClient, FileInfo,
    GetStatusPOptions, GetStatusPRequest, ListStatusPOptions, ListStatusPRequest, OpenFilePOptions,
};
use alluxio_grpc::auth::AuthInterceptor;
use alluxio_grpc::grpc_client::Client;

use crate::file::common::{
    AlluxioFileInStream, DataReader, DefaultDataBuffer, FileSystemContext, GrpcDataReader,
    InStreamOptions, LocalFirstPolicy, URIStatus, WorkerNetAddress,
};

use super::common::DataBuffer;

#[derive(Debug)]
pub struct AlluxioFileSystem {
    client: FileSystemMasterClientServiceClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl AlluxioFileSystem {
    pub fn create(client: Client) -> Result<AlluxioFileSystem, &'static str> {
        return Ok(AlluxioFileSystem {
            client: FileSystemMasterClientServiceClient::with_interceptor(
                client.channel,
                client.interceptor,
            ),
        });
    }

    pub async fn get_status(
        &mut self,
        path: String,
    ) -> Result<Option<FileInfo>, Box<dyn std::error::Error>> {
        let request = GetStatusPRequest {
            path: Some(path),
            options: Some(GetStatusPOptions {
                load_metadata_type: Some(1), //load once
                common_options: None,
                access_mode: None,
                update_timestamps: Some(false),
            }),
        };
        // TODO remove .await and let get_status full async
        let response = self.client.get_status(request).await?;
        //println!("get status {:?}", response.into_inner().file_info);
        let stream = response.into_inner();
        while let Some(file_info) = stream.file_info {
            //println!("Ls RESPONSE streaming = {:?}", listStatusResponse);
            return Ok(Some(file_info));
        }
        Ok(None)
    }

    pub async fn list_status(
        &mut self,
        path: String,
    ) -> Result<Vec<FileInfo>, Box<dyn std::error::Error>> {
        let request = ListStatusPRequest {
            path: Some(path),
            options: Some(ListStatusPOptions {
                load_direct_children: Some(true),
                load_metadata_type: None,
                common_options: None,
                recursive: Some(false),
                load_metadata_only: Some(false),
            }),
        };
        let response = self.client.list_status(request).await?;
        // println!("list status {:?}", response);
        let mut stream = response.into_inner();
        while let Ok(Some(list_status_response)) = stream.message().await {
            //println!("Ls RESPONSE streaming = {:?}", listStatusResponse);
            return Ok(list_status_response.file_infos);
        }
        Ok(Vec::new())
    }

    // TODO Rewrite the code to avoid using multiple nested matches
    pub async fn open_file(&mut self, path: String) -> Result<Vec<u8>, String> {
        match self.get_status(path).await {
            Ok(status) => {
                match status {
                    Some(file_info) => {
                        // println!("{:?}", file_info);
                        if file_info.folder() {
                            return Err("OpenDirectoryException".to_string());
                        }
                        if !file_info.completed() {
                            return Err("FileIncompleteException".to_string());
                        }
                        // AlluxioConfiguration conf = mFsContext.getPathConf(path);
                        // OpenFilePOptions mergedOptions = FileSystemOptions.openFileDefaults(conf)
                        //     .toBuilder().mergeFrom(options).build();
                        // InStreamOptions inStreamOptions = new InStreamOptions(status, mergedOptions, conf);
                        // return new AlluxioFileInStream(status, inStreamOptions, mFsContext);
                        let status = URIStatus { file_info };
                        let options = InStreamOptions {
                            status,
                            options: OpenFilePOptions::default(),
                            ufs_read_location_policy: LocalFirstPolicy {},
                            position_short: false, // Set position_short to false for minimum available
                        };
                        let context = FileSystemContext {};
                        // let in_stream: AlluxioFileInStream<GrpcDataReader, LocalFirstPolicy> =
                        //     AlluxioFileInStream::new(options, context);
                        // TODO FileInStream <- BlockInStream <- DataReader
                        let read_request = ReadRequest {
                            block_id: Some(83902857216),
                            chunk_size: Some(1000),
                            length: Some(50),
                            offset: Some(5),
                            offset_received: None,
                            open_ufs_block_options: None,
                            position_short: None,
                            promote: None,
                        };
                        let address = WorkerNetAddress {
                            host: "localhost".to_string(),
                        };
                        let data_reader = GrpcDataReader::create(read_request, address);
                        match data_reader.read_chunk().await {
                            Ok(mut data_buffer) => {
                                let mut result = Vec::new();
                                match data_buffer.read_bytes(&mut result) {
                                    Ok(()) => Ok(result),
                                    Err(err) => Err(err.to_string()),
                                }
                            }
                            Err(err) => Err(err.to_string()),
                        }
                    }
                    None => Ok(Vec::new()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}

pub fn create(client: Client) -> Result<AlluxioFileSystem, &'static str> {
    return Ok(AlluxioFileSystem::create(client).unwrap());
}
