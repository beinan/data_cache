use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;

use alluxio_grpc::alluxio::grpc::file::{
    file_system_master_client_service_client::FileSystemMasterClientServiceClient, FileInfo,
    GetStatusPOptions, GetStatusPRequest, ListStatusPOptions, ListStatusPRequest, OpenFilePOptions,
};
use alluxio_grpc::auth::AuthInterceptor;
use alluxio_grpc::grpc_client::Client;

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

    pub async fn open_file(
        &mut self,
        fileInfo: FileInfo,
        options: OpenFilePOptions,
    ) -> Result<Vec<FileInfo>, Box<dyn std::error::Error>> {
        if fileInfo.folder() {
            // throw new OpenDirectoryException(path);
        }
        if !fileInfo.completed() {
            // throw new FileIncompleteException(path);
        }
        // AlluxioConfiguration conf = mFsContext.getPathConf(path);
        // OpenFilePOptions mergedOptions = FileSystemOptions.openFileDefaults(conf)
        //     .toBuilder().mergeFrom(options).build();
        // InStreamOptions inStreamOptions = new InStreamOptions(status, mergedOptions, conf);
        // return new AlluxioFileInStream(status, inStreamOptions, mFsContext);
        Ok(Vec::new())
    }
}

pub fn create(client: Client) -> Result<AlluxioFileSystem, &'static str> {
    return Ok(AlluxioFileSystem::create(client).unwrap());
}
