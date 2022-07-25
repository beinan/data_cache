///*
/// Unique operation id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsOpPId {
    #[prost(int64, optional, tag="1")]
    pub most_significant_bits: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub least_significant_bits: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemMasterCommonPOptions {
    #[prost(int64, optional, tag="1")]
    pub sync_interval_ms: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub ttl: ::core::option::Option<i64>,
    #[prost(enumeration="super::TtlAction", optional, tag="3")]
    pub ttl_action: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub operation_id: ::core::option::Option<FsOpPId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckAccessPRequest {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<CheckAccessPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckAccessPResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckAccessPOptions {
    #[prost(enumeration="super::Bits", optional, tag="1")]
    pub bits: ::core::option::Option<i32>,
    #[prost(message, optional, tag="2")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckConsistencyPResponse {
    #[prost(string, repeated, tag="1")]
    pub inconsistent_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckConsistencyPOptions {
    #[prost(message, optional, tag="1")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckConsistencyPRequest {
    ///* the root of the subtree to check 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<CheckConsistencyPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExistsPRequest {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<ExistsPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExistsPResponse {
    #[prost(bool, optional, tag="1")]
    pub exists: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteFilePResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteFilePOptions {
    #[prost(int64, optional, tag="1")]
    pub ufs_length: ::core::option::Option<i64>,
    #[prost(message, optional, tag="2")]
    pub async_persist_options: ::core::option::Option<ScheduleAsyncPersistencePOptions>,
    #[prost(message, optional, tag="3")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteFilePRequest {
    ///* the path of the file 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<CompleteFilePOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenFilePOptions {
    #[prost(enumeration="ReadPType", optional, tag="1")]
    pub read_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub max_ufs_read_concurrency: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(bool, optional, tag="4", default="true")]
    pub update_last_access_time: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDirectoryPResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDirectoryPOptions {
    #[prost(bool, optional, tag="1")]
    pub recursive: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub allow_exists: ::core::option::Option<bool>,
    #[prost(message, optional, tag="3")]
    pub mode: ::core::option::Option<super::PMode>,
    #[prost(enumeration="WritePType", optional, tag="4")]
    pub write_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="5")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(map="string, bytes", tag="6")]
    pub xattr: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration="XAttrPropagationStrategy", optional, tag="7", default="NewPaths")]
    pub xattr_prop_strat: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDirectoryPRequest {
    ///* the path of the directory 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<CreateDirectoryPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFilePResponse {
    #[prost(message, optional, tag="1")]
    pub file_info: ::core::option::Option<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFilePOptions {
    #[prost(int64, optional, tag="1")]
    pub block_size_bytes: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="2")]
    pub recursive: ::core::option::Option<bool>,
    #[prost(message, optional, tag="3")]
    pub mode: ::core::option::Option<super::PMode>,
    #[prost(int32, optional, tag="4")]
    pub replication_max: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub replication_min: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub replication_durable: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub write_tier: ::core::option::Option<i32>,
    #[prost(enumeration="WritePType", optional, tag="8")]
    pub write_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="9")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(int64, optional, tag="10")]
    pub persistence_wait_time: ::core::option::Option<i64>,
    #[prost(map="string, bytes", tag="11")]
    pub xattr: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration="XAttrPropagationStrategy", optional, tag="12", default="NewPaths")]
    pub xattr_prop_strat: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFilePRequest {
    ///* the path of the file 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<CreateFilePOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePOptions {
    #[prost(bool, optional, tag="1")]
    pub recursive: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub alluxio_only: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub unchecked: ::core::option::Option<bool>,
    #[prost(message, optional, tag="4")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePRequest {
    ///* the path of the file or directory 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<DeletePOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreePResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreePOptions {
    #[prost(bool, optional, tag="1")]
    pub recursive: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub forced: ::core::option::Option<bool>,
    #[prost(message, optional, tag="3")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreePRequest {
    ///* the path of the file or directory 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<FreePOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNewBlockIdForFilePResponse {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNewBlockIdForFilePOptions {
    #[prost(message, optional, tag="1")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNewBlockIdForFilePRequest {
    ///* the path of the file 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<GetNewBlockIdForFilePOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusPResponse {
    #[prost(message, optional, tag="1")]
    pub file_info: ::core::option::Option<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusPOptions {
    #[prost(enumeration="LoadMetadataPType", optional, tag="1")]
    pub load_metadata_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="2")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(enumeration="super::Bits", optional, tag="3")]
    pub access_mode: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="4", default="true")]
    pub update_timestamps: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusPRequest {
    ///* the path of the file or directory 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<GetStatusPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExistsPOptions {
    #[prost(enumeration="LoadMetadataPType", optional, tag="1")]
    pub load_metadata_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="2")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncPointInfo {
    #[prost(string, optional, tag="1")]
    pub sync_point_uri: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="SyncPointStatus", optional, tag="2")]
    pub sync_status: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSyncPathListPResponse {
    #[prost(message, repeated, tag="1")]
    pub sync_paths: ::prost::alloc::vec::Vec<SyncPointInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSyncPathListPRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStatusPResponse {
    #[prost(message, repeated, tag="1")]
    pub file_infos: ::prost::alloc::vec::Vec<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStatusPOptions {
    /// This is deprecated since 1.1.1 and will be removed in 2.0. Use loadMetadataType.
    #[prost(bool, optional, tag="1")]
    pub load_direct_children: ::core::option::Option<bool>,
    #[prost(enumeration="LoadMetadataPType", optional, tag="2")]
    pub load_metadata_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(bool, optional, tag="4")]
    pub recursive: ::core::option::Option<bool>,
    /// No data will be transferred.
    #[prost(bool, optional, tag="5")]
    pub load_metadata_only: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStatusPRequest {
    ///* the path of the file or directory 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<ListStatusPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadMetadataPOptions {
    ///* whether to load metadata recursively 
    #[prost(bool, optional, tag="1")]
    pub recursive: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub create_ancestors: ::core::option::Option<bool>,
    #[prost(enumeration="super::fscommon::LoadDescendantPType", optional, tag="3")]
    pub load_descendant_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(enumeration="LoadMetadataPType", optional, tag="5")]
    pub load_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PAclEntry {
    #[prost(enumeration="PAclEntryType", optional, tag="1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub subject: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="PAclAction", repeated, packed="false", tag="3")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag="4")]
    pub is_default: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PAcl {
    #[prost(string, optional, tag="1")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub owning_group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub entries: ::prost::alloc::vec::Vec<PAclEntry>,
    #[prost(int32, optional, tag="4")]
    pub mode: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="5")]
    pub is_default: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="6")]
    pub is_default_empty: ::core::option::Option<bool>,
}
///*
/// Contains the information of a block in a file. In addition to the BlockInfo, it includes the
/// offset in the file, and the under file system locations of the block replicas.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileBlockInfo {
    #[prost(message, optional, tag="1")]
    pub block_info: ::core::option::Option<super::BlockInfo>,
    #[prost(int64, optional, tag="2")]
    pub offset: ::core::option::Option<i64>,
    /// deprecated since 1.1 will be removed in 2.0 (replaced by ufsStringLocations)
    #[prost(message, repeated, tag="3")]
    pub ufs_locations: ::prost::alloc::vec::Vec<super::WorkerNetAddress>,
    #[prost(string, repeated, tag="4")]
    pub ufs_string_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    #[prost(int64, optional, tag="1")]
    pub file_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="5")]
    pub length: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="6")]
    pub block_size_bytes: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="7")]
    pub creation_time_ms: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="8")]
    pub completed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="9")]
    pub folder: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="10")]
    pub pinned: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="11")]
    pub cacheable: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="12")]
    pub persisted: ::core::option::Option<bool>,
    #[prost(int64, repeated, packed="false", tag="13")]
    pub block_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, optional, tag="14")]
    pub last_modification_time_ms: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="15")]
    pub ttl: ::core::option::Option<i64>,
    #[prost(string, optional, tag="16")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="18")]
    pub mode: ::core::option::Option<i32>,
    #[prost(string, optional, tag="19")]
    pub persistence_state: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="20")]
    pub mount_point: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="21")]
    pub file_block_infos: ::prost::alloc::vec::Vec<FileBlockInfo>,
    #[prost(enumeration="super::TtlAction", optional, tag="22")]
    pub ttl_action: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="23")]
    pub mount_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="24")]
    pub in_alluxio_percentage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="25")]
    pub in_memory_percentage: ::core::option::Option<i32>,
    #[prost(string, optional, tag="26")]
    pub ufs_fingerprint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="27")]
    pub acl: ::core::option::Option<PAcl>,
    #[prost(message, optional, tag="28")]
    pub default_acl: ::core::option::Option<PAcl>,
    #[prost(int32, optional, tag="29")]
    pub replication_max: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="30")]
    pub replication_min: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="31")]
    pub last_access_time_ms: ::core::option::Option<i64>,
    #[prost(map="string, bytes", tag="32")]
    pub xattr: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag="33")]
    pub medium_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFilePathPResponse {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFilePathPRequest {
    #[prost(int64, optional, tag="1")]
    pub file_id: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountPResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountPOptions {
    #[prost(bool, optional, tag="1")]
    pub read_only: ::core::option::Option<bool>,
    #[prost(map="string, string", tag="2")]
    pub properties: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(bool, optional, tag="3")]
    pub shared: ::core::option::Option<bool>,
    #[prost(message, optional, tag="4")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountPRequest {
    ///* the path of alluxio mount point 
    #[prost(string, optional, tag="1")]
    pub alluxio_path: ::core::option::Option<::prost::alloc::string::String>,
    ///* the path of the under file system 
    #[prost(string, optional, tag="2")]
    pub ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<MountPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMountTablePResponse {
    #[prost(map="string, message", tag="1")]
    pub mount_points: ::std::collections::HashMap<::prost::alloc::string::String, MountPointInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMountTablePRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountPointInfo {
    #[prost(string, optional, tag="1")]
    pub ufs_uri: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub ufs_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="3", default="-1")]
    pub ufs_capacity_bytes: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4", default="-1")]
    pub ufs_used_bytes: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="5")]
    pub read_only: ::core::option::Option<bool>,
    #[prost(map="string, string", tag="6")]
    pub properties: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(bool, optional, tag="7")]
    pub shared: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="8")]
    pub mount_id: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemCommandOptions {
    #[prost(message, optional, tag="1")]
    pub persist_options: ::core::option::Option<PersistCommandOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistCommandOptions {
    #[prost(message, repeated, tag="1")]
    pub persist_files: ::prost::alloc::vec::Vec<PersistFile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistFile {
    #[prost(int64, optional, tag="1")]
    pub file_id: ::core::option::Option<i64>,
    #[prost(int64, repeated, packed="false", tag="2")]
    pub block_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemCommand {
    #[prost(enumeration="super::CommandType", optional, tag="1")]
    pub command_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="2")]
    pub command_options: ::core::option::Option<FileSystemCommandOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenamePResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenamePOptions {
    #[prost(message, optional, tag="1")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(bool, optional, tag="2")]
    pub persist: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenamePRequest {
    ///* the source path of the file or directory 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    ///* the destination path 
    #[prost(string, optional, tag="2")]
    pub dst_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<RenamePOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReverseResolvePRequest {
    #[prost(string, optional, tag="1")]
    pub ufs_uri: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReverseResolvePResponse {
    #[prost(string, optional, tag="1")]
    pub alluxio_path: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttributePResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttributePOptions {
    #[prost(bool, optional, tag="1")]
    pub pinned: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub persisted: ::core::option::Option<bool>,
    #[prost(string, optional, tag="3")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub mode: ::core::option::Option<super::PMode>,
    #[prost(bool, optional, tag="6")]
    pub recursive: ::core::option::Option<bool>,
    #[prost(int32, optional, tag="7")]
    pub replication_max: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub replication_min: ::core::option::Option<i32>,
    #[prost(message, optional, tag="9")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(string, repeated, tag="10")]
    pub pinned_media: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, bytes", tag="11")]
    pub xattr: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration="super::super::proto::journal::XAttrUpdateStrategy", optional, tag="12")]
    pub xattr_update_strategy: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttributePRequest {
    ///* the path of the file 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<SetAttributePOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAclPResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAclPOptions {
    #[prost(message, optional, tag="1")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(bool, optional, tag="2")]
    pub recursive: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAclPRequest {
    ///* the path of the file or directory 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    ///* the set action to perform 
    #[prost(enumeration="SetAclAction", optional, tag="2")]
    pub action: ::core::option::Option<i32>,
    ///* the list of ACL entries 
    #[prost(message, repeated, tag="3")]
    pub entries: ::prost::alloc::vec::Vec<PAclEntry>,
    ///* the method options 
    #[prost(message, optional, tag="4")]
    pub options: ::core::option::Option<SetAclPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleAsyncPersistencePResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleAsyncPersistencePOptions {
    #[prost(message, optional, tag="1")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
    #[prost(int64, optional, tag="2")]
    pub persistence_wait_time: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleAsyncPersistencePRequest {
    ///* the path of the file 
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<ScheduleAsyncPersistencePOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartSyncPResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartSyncPOptions {
    #[prost(message, optional, tag="1")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartSyncPRequest {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<StartSyncPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopSyncPResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopSyncPOptions {
    #[prost(message, optional, tag="1")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopSyncPRequest {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<StopSyncPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnmountPResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnmountPOptions {
    #[prost(message, optional, tag="1")]
    pub common_options: ::core::option::Option<FileSystemMasterCommonPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnmountPRequest {
    ///* the path of the alluxio mount point 
    #[prost(string, optional, tag="1")]
    pub alluxio_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<UnmountPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UfsInfo {
    #[prost(string, optional, tag="1")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub properties: ::core::option::Option<MountPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMountPRequest {
    ///* the path of alluxio mount point 
    #[prost(string, optional, tag="1")]
    pub alluxio_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<MountPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMountPResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUfsModePResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUfsModePOptions {
    #[prost(enumeration="UfsPMode", optional, tag="1")]
    pub ufs_mode: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUfsModePRequest {
    ///* the ufs path 
    #[prost(string, optional, tag="1")]
    pub ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<UpdateUfsModePOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStateLockHoldersPResponse {
    #[prost(string, repeated, tag="1")]
    pub threads: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStateLockHoldersPOptions {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStateLockHoldersPRequest {
    #[prost(message, optional, tag="1")]
    pub options: ::core::option::Option<GetStateLockHoldersPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemHeartbeatPResponse {
    #[prost(message, optional, tag="1")]
    pub command: ::core::option::Option<FileSystemCommand>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemHeartbeatPOptions {
    #[prost(string, repeated, tag="1")]
    pub persisted_file_fingerprints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemHeartbeatPRequest {
    ///* the id of the worker 
    #[prost(int64, optional, tag="1")]
    pub worker_id: ::core::option::Option<i64>,
    ///* the list of persisted files 
    #[prost(int64, repeated, packed="false", tag="2")]
    pub persisted_files: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<FileSystemHeartbeatPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileInfoPResponse {
    #[prost(message, optional, tag="1")]
    pub file_info: ::core::option::Option<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileInfoPOptions {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileInfoPRequest {
    ///* the id of the file 
    #[prost(int64, optional, tag="1")]
    pub file_id: ::core::option::Option<i64>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<GetFileInfoPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPinnedFileIdsPResponse {
    /// TODO(adit): set replacement?
    #[prost(int64, repeated, packed="false", tag="1")]
    pub pinned_file_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPinnedFileIdsPOptions {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPinnedFileIdsPRequest {
    #[prost(message, optional, tag="1")]
    pub options: ::core::option::Option<GetPinnedFileIdsPOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUfsInfoPResponse {
    #[prost(message, optional, tag="1")]
    pub ufs_info: ::core::option::Option<UfsInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUfsInfoPOptions {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUfsInfoPRequest {
    ///* the id of the ufs 
    #[prost(int64, optional, tag="1")]
    pub mount_id: ::core::option::Option<i64>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<GetUfsInfoPOptions>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WritePType {
    MustCache = 1,
    TryCache = 2,
    CacheThrough = 3,
    Through = 4,
    AsyncThrough = 5,
    None = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReadPType {
    NoCache = 1,
    Cache = 2,
    CachePromote = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoadMetadataPType {
    /// Never load metadata.
    Never = 0,
    /// Load metadata only once.
    Once = 1,
    /// Always load metadata.
    Always = 2,
}
/// XAttrPropagationStrategy controls the behaviour for assigning xAttr
/// on Inodes within nested directories
/// - NEW_PATHS: Assign xAttr for any missing nodes along the filepath
/// - LEAF_NODE: Only assign xAttr on the leaf node of the filepath
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum XAttrPropagationStrategy {
    NewPaths = 1,
    LeafNode = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SyncPointStatus {
    NotInitiallySynced = 0,
    Syncing = 1,
    InitiallySynced = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PAclEntryType {
    Owner = 0,
    NamedUser = 1,
    OwningGroup = 2,
    NamedGroup = 3,
    Mask = 4,
    Other = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PAclAction {
    Read = 0,
    Write = 1,
    Execute = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetAclAction {
    Replace = 0,
    Modify = 1,
    Remove = 2,
    RemoveAll = 3,
    RemoveDefault = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UfsPMode {
    NoAccess = 1,
    ReadOnly = 2,
    ReadWrite = 3,
}
/// Generated client implementations.
pub mod file_system_master_client_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///*
    /// This interface contains file system master service endpoints for Alluxio clients.
    #[derive(Debug, Clone)]
    pub struct FileSystemMasterClientServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FileSystemMasterClientServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FileSystemMasterClientServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FileSystemMasterClientServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            FileSystemMasterClientServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        ///*
        /// Checks access to path.
        pub async fn check_access(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckAccessPRequest>,
        ) -> Result<tonic::Response<super::CheckAccessPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/CheckAccess",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Checks the consistency of the files and directores with the path as the root of the subtree
        pub async fn check_consistency(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckConsistencyPRequest>,
        ) -> Result<tonic::Response<super::CheckConsistencyPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/CheckConsistency",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Checks the existence of the file or directory.
        pub async fn exists(
            &mut self,
            request: impl tonic::IntoRequest<super::ExistsPRequest>,
        ) -> Result<tonic::Response<super::ExistsPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/Exists",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Marks a file as completed.
        pub async fn complete_file(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteFilePRequest>,
        ) -> Result<tonic::Response<super::CompleteFilePResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/CompleteFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Creates a directory.
        pub async fn create_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDirectoryPRequest>,
        ) -> Result<tonic::Response<super::CreateDirectoryPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/CreateDirectory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Creates a file.
        pub async fn create_file(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFilePRequest>,
        ) -> Result<tonic::Response<super::CreateFilePResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/CreateFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Frees the given file or directory from Alluxio.
        pub async fn free(
            &mut self,
            request: impl tonic::IntoRequest<super::FreePRequest>,
        ) -> Result<tonic::Response<super::FreePResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/Free",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Returns the file path of a file id
        pub async fn get_file_path(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFilePathPRequest>,
        ) -> Result<tonic::Response<super::GetFilePathPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/GetFilePath",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Returns a map from each Alluxio path to information of corresponding mount point
        pub async fn get_mount_table(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMountTablePRequest>,
        ) -> Result<tonic::Response<super::GetMountTablePResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/GetMountTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Returns a list of paths that are being actively synced by Alluxio
        pub async fn get_sync_path_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSyncPathListPRequest>,
        ) -> Result<tonic::Response<super::GetSyncPathListPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/GetSyncPathList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Generates a new block id for the given file.
        pub async fn get_new_block_id_for_file(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNewBlockIdForFilePRequest>,
        ) -> Result<
            tonic::Response<super::GetNewBlockIdForFilePResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/GetNewBlockIdForFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Returns the status of the file or directory.
        pub async fn get_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStatusPRequest>,
        ) -> Result<tonic::Response<super::GetStatusPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/GetStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// If the path points to a file, the method returns a singleton with its file information.
        /// If the path points to a directory, the method returns a list with file information for the
        /// directory contents.
        pub async fn list_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStatusPRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListStatusPResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/ListStatus",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        ///*
        /// Creates a new "mount point", mounts the given UFS path in the Alluxio namespace at the given
        /// path. The path should not exist and should not be nested under any existing mount point.
        pub async fn mount(
            &mut self,
            request: impl tonic::IntoRequest<super::MountPRequest>,
        ) -> Result<tonic::Response<super::MountPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/Mount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Deletes a file or a directory and returns whether the remove operation succeeded.
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePRequest>,
        ) -> Result<tonic::Response<super::DeletePResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/Remove",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Renames a file or a directory.
        pub async fn rename(
            &mut self,
            request: impl tonic::IntoRequest<super::RenamePRequest>,
        ) -> Result<tonic::Response<super::RenamePResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/Rename",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Reverse resolve a ufs path.
        pub async fn reverse_resolve(
            &mut self,
            request: impl tonic::IntoRequest<super::ReverseResolvePRequest>,
        ) -> Result<tonic::Response<super::ReverseResolvePResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/ReverseResolve",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Schedules async persistence.
        pub async fn schedule_async_persistence(
            &mut self,
            request: impl tonic::IntoRequest<super::ScheduleAsyncPersistencePRequest>,
        ) -> Result<
            tonic::Response<super::ScheduleAsyncPersistencePResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/ScheduleAsyncPersistence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Sets ACL for the path.
        pub async fn set_acl(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAclPRequest>,
        ) -> Result<tonic::Response<super::SetAclPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/SetAcl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Sets file or directory attributes.
        pub async fn set_attribute(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAttributePRequest>,
        ) -> Result<tonic::Response<super::SetAttributePResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/SetAttribute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Start the active syncing of the directory or file
        pub async fn start_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::StartSyncPRequest>,
        ) -> Result<tonic::Response<super::StartSyncPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/StartSync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Start the active syncing of the directory or file
        pub async fn stop_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::StopSyncPRequest>,
        ) -> Result<tonic::Response<super::StopSyncPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/StopSync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Deletes an existing "mount point", voiding the Alluxio namespace at the given path. The path
        /// should correspond to an existing mount point. Any files in its subtree that are backed by UFS
        /// will be persisted before they are removed from the Alluxio namespace.
        pub async fn unmount(
            &mut self,
            request: impl tonic::IntoRequest<super::UnmountPRequest>,
        ) -> Result<tonic::Response<super::UnmountPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/Unmount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Updates an existing "mount point", changing its mount properties
        pub async fn update_mount(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMountPRequest>,
        ) -> Result<tonic::Response<super::UpdateMountPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/UpdateMount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Updates the ufs mode for a ufs path under one or more mount points.
        pub async fn update_ufs_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUfsModePRequest>,
        ) -> Result<tonic::Response<super::UpdateUfsModePResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/UpdateUfsMode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_state_lock_holders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStateLockHoldersPRequest>,
        ) -> Result<
            tonic::Response<super::GetStateLockHoldersPResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterClientService/GetStateLockHolders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod file_system_master_worker_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///*
    /// This interface contains file system master service endpoints for Alluxio workers.
    #[derive(Debug, Clone)]
    pub struct FileSystemMasterWorkerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FileSystemMasterWorkerServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FileSystemMasterWorkerServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FileSystemMasterWorkerServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            FileSystemMasterWorkerServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        ///*
        /// Periodic file system worker heartbeat. Returns the command for persisting
        /// the blocks of a file.
        pub async fn file_system_heartbeat(
            &mut self,
            request: impl tonic::IntoRequest<super::FileSystemHeartbeatPRequest>,
        ) -> Result<
            tonic::Response<super::FileSystemHeartbeatPResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterWorkerService/FileSystemHeartbeat",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Returns the file information for a file or directory identified by the given file id.
        pub async fn get_file_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFileInfoPRequest>,
        ) -> Result<tonic::Response<super::GetFileInfoPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterWorkerService/GetFileInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Returns the set of pinned file ids.
        pub async fn get_pinned_file_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPinnedFileIdsPRequest>,
        ) -> Result<tonic::Response<super::GetPinnedFileIdsPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterWorkerService/GetPinnedFileIds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Returns the UFS information for the given mount point identified by its id.
        pub async fn get_ufs_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUfsInfoPRequest>,
        ) -> Result<tonic::Response<super::GetUfsInfoPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterWorkerService/GetUfsInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod file_system_master_job_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///*
    /// This interface contains file system master service endpoints for Alluxio workers.
    #[derive(Debug, Clone)]
    pub struct FileSystemMasterJobServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FileSystemMasterJobServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FileSystemMasterJobServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FileSystemMasterJobServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            FileSystemMasterJobServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        ///
        /// Returns the file information for a file or directory identified by the given file id.
        pub async fn get_file_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFileInfoPRequest>,
        ) -> Result<tonic::Response<super::GetFileInfoPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterJobService/GetFileInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///*
        /// Returns the UFS information for the given mount point identified by its id.
        pub async fn get_ufs_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUfsInfoPRequest>,
        ) -> Result<tonic::Response<super::GetUfsInfoPResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/alluxio.grpc.file.FileSystemMasterJobService/GetUfsInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
