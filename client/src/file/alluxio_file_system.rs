use alluxio_grpc::alluxio::grpc::block::ReadRequest;
use futures::stream;
use std::io::Cursor;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;

use alluxio_common::exception::AlluxioException;
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

// #[derive(Debug)]
pub struct AlluxioFileSystem {
    client: FileSystemMasterClientServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    options: Result<InStreamOptions<LocalFirstPolicy>, AlluxioException>,
    context: Result<FileSystemContext, AlluxioException>,
}

impl AlluxioFileSystem {
    pub fn create(client: Client) -> Result<AlluxioFileSystem, &'static str> {
        return Ok(AlluxioFileSystem {
            client: FileSystemMasterClientServiceClient::with_interceptor(
                client.channel,
                client.interceptor,
            ),
            options: Err(AlluxioException::UnexpectedAlluxioException {
                message: "InStreamOptions not set for AlluxioFileSystem".to_string(),
            }),
            context: Err(AlluxioException::UnexpectedAlluxioException {
                message: "FileSystemContext not set for AlluxioFileSystem".to_string(),
            }),
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
        let mut stream = response.into_inner();
        while let Ok(Some(list_status_response)) = stream.message().await {
            return Ok(list_status_response.file_infos);
        }
        Ok(Vec::new())
    }

    // TODO Rewrite the code to avoid using multiple nested matches
    pub async fn open_file<'a>(
        &mut self,
        path: String,
    ) -> Result<AlluxioFileInStream<LocalFirstPolicy>, AlluxioException> {
        // just for write log
        let path_clone = path.clone();
        match self.get_status(path).await {
            Ok(status) => {
                match status {
                    Some(file_info) => {
                        if file_info.folder() {
                            return Err(AlluxioException::InvalidPathException {
                                message: "OpenDirectoryException".to_string(),
                            });
                        }
                        if !file_info.completed() {
                            return Err(AlluxioException::FileIncompleteException {
                                message: path_clone,
                            });
                        }

                        let status = URIStatus { file_info };
                        let options = InStreamOptions {
                            status,
                            options: OpenFilePOptions::default(),
                            ufs_read_location_policy: LocalFirstPolicy {},
                            position_short: false, // Set position_short to false for minimum available
                        };
                        let context = FileSystemContext {};
                        self.options = Ok(options);
                        self.context = Ok(context);
                        let in_stream = AlluxioFileInStream::new(
                            self.options.as_ref().unwrap(),
                            self.context.as_ref().unwrap(),
                        );
                        in_stream
                    }
                    None => {
                        return Err(AlluxioException::FileDoesNotExistException {
                            message: path_clone,
                        })
                    }
                }
            }
            Err(err) => Err(AlluxioException::UnexpectedAlluxioException {
                message: path_clone,
            }),
        }
    }
}

pub fn create(client: Client) -> Result<AlluxioFileSystem, &'static str> {
    return Ok(AlluxioFileSystem::create(client).unwrap());
}
