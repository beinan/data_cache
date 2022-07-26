use tonic::metadata::AsciiMetadataValue;
use tonic::{Request, transport::Channel};
use tonic::service::Interceptor;
use tonic::service::interceptor::InterceptedService;
use crate::auth::AuthInterceptor;

use crate::grpc::file::file_system_master_client_service_client::FileSystemMasterClientServiceClient;
use crate::grpc::file::{ListStatusPRequest, ListStatusPOptions, FileInfo, GetStatusPRequest, GetStatusPOptions};

#[derive(Debug)]
pub struct MasterClient {
    client: FileSystemMasterClientServiceClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl MasterClient {
    pub fn create(channel : Channel, interceptor: AuthInterceptor) -> Result<MasterClient, &'static str>
    {
        return Ok(MasterClient {client : FileSystemMasterClientServiceClient::with_interceptor(
            channel, interceptor) });
    }

    pub async fn getStatus(&mut self, path : String) -> Result<Option<FileInfo>, Box<dyn std::error::Error>> {
        let request = GetStatusPRequest {
            path: Some(path),
            options: Some(GetStatusPOptions {
                load_metadata_type: Some(1), //load once
                common_options: None,
                access_mode: None,
                update_timestamps: Some(false)
            })
        };
        let response = self.client.get_status(request).await?;
        //println!("get status {:?}", response.into_inner().file_info);
        // let mut stream = response.into_inner();
        // while let Ok(Some(getStatusResponse)) = stream.message().await {
        //     //println!("Ls RESPONSE streaming = {:?}", listStatusResponse);
        //     return Ok(getStatusResponse.file_info)
        // }
        Ok(response.into_inner().file_info)
    }
    pub async fn listStatus(&mut self, path : String) -> Result<Vec<FileInfo>, Box<dyn std::error::Error>> {
        let request = ListStatusPRequest {
            path: Some(String::from("/")),
            options: Some(ListStatusPOptions {
                load_direct_children: Some(true),
                load_metadata_type: None,
                common_options: None,
                recursive: Some(false),
                load_metadata_only: Some(false)
            })
        };
        let response = self.client.list_status(request).await?;
        // println!("list status {:?}", response);
        let mut stream = response.into_inner();
        while let Ok(Some(listStatusResponse)) = stream.message().await {
            //println!("Ls RESPONSE streaming = {:?}", listStatusResponse);
            return Ok(listStatusResponse.file_infos)
        }
        Ok(Vec::new())
    }
}
