/// The read request.
/// next available id: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub offset: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub length: ::core::option::Option<i64>,
    /// If set, this request is to cancel the reading request for the id.
    #[prost(bool, optional, tag="4")]
    pub cancel: ::core::option::Option<bool>,
    /// Whether the block should be promoted before reading
    #[prost(bool, optional, tag="7")]
    pub promote: ::core::option::Option<bool>,
    /// If set, the server should send packets in the specified packet size.
    #[prost(int64, optional, tag="5")]
    pub chunk_size: ::core::option::Option<i64>,
    /// This is only set for UFS block read.
    #[prost(message, optional, tag="6")]
    pub open_ufs_block_options: ::core::option::Option<OpenUfsBlockOptions>,
}
/// Options for caching a block asynchronously
/// next available id: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncCacheRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    /// TODO(calvin): source host and port should be replace with WorkerNetAddress
    #[prost(string, optional, tag="2")]
    pub source_host: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub source_port: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub open_ufs_block_options: ::core::option::Option<OpenUfsBlockOptions>,
    #[prost(int64, optional, tag="5")]
    pub length: ::core::option::Option<i64>,
}
/// Options to open a UFS block.
/// next available id: 7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenUfsBlockOptions {
    #[prost(string, optional, tag="1")]
    pub ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    /// The offset of the block in within the file.
    #[prost(int64, optional, tag="2")]
    pub offset_in_file: ::core::option::Option<i64>,
    /// The block size.
    #[prost(int64, optional, tag="3")]
    pub block_size: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="4")]
    pub max_ufs_read_concurrency: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="5")]
    pub mount_id: ::core::option::Option<i64>,
    /// If set, do not try to cache the block locally when reading the data from the UFS.
    #[prost(bool, optional, tag="6")]
    pub no_cache: ::core::option::Option<bool>,
    /// The client does not need to set this. This is set by the worker.
    #[prost(string, optional, tag="7")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    /// If set to true, the block is possibly stored as a UFS block.
    #[prost(bool, optional, tag="8")]
    pub block_in_ufs_tier: ::core::option::Option<bool>,
}
/// The write request.
/// next available id: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(enumeration="RequestType", optional, tag="1")]
    pub r#type: ::core::option::Option<i32>,
    /// The block ID or UFS file ID.
    #[prost(int64, optional, tag="2")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub offset: ::core::option::Option<i64>,
    /// This is only applicable for block write.
    #[prost(int32, optional, tag="4")]
    pub tier: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="5")]
    pub eof: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="6")]
    pub cancel: ::core::option::Option<bool>,
    /// This is only applicable for ufs writes.
    #[prost(message, optional, tag="7")]
    pub create_ufs_file_options: ::core::option::Option<CreateUfsFileOptions>,
}
/// Options to create a UFS file.
/// next available: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUfsFileOptions {
    #[prost(string, optional, tag="1")]
    pub ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="4")]
    pub mode: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="5")]
    pub mount_id: ::core::option::Option<i64>,
    #[prost(message, optional, tag="6")]
    pub acl: ::core::option::Option<super::shared::AccessControlList>,
}
/// Options to create a block file in UFS.
/// next available: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUfsBlockOptions {
    /// the number of bytes previously written to block store
    /// zero if no previous temp block created
    #[prost(int64, optional, tag="1")]
    pub bytes_in_block_store: ::core::option::Option<i64>,
    /// mount ID of the UFS to write to
    #[prost(int64, optional, tag="2")]
    pub mount_id: ::core::option::Option<i64>,
    /// true if the write request is already a fallback from
    /// a failed short-circuit write.
    #[prost(bool, optional, tag="3")]
    pub fallback: ::core::option::Option<bool>,
}
/// The read response.
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    #[prost(enumeration="read_response::Type", optional, tag="1")]
    pub r#type: ::core::option::Option<i32>,
}
/// Nested message and enum types in `ReadResponse`.
pub mod read_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// A heatbeat message indicates that the server is still actively acquiring access to a UFS file.
        /// This is to avoid timing out in the client.
        UfsReadHeartbeat = 1,
    }
}
/// A heartbeat
///
/// Empty message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {
}
/// The response.
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(enumeration="super::status::PStatus", optional, tag="1")]
    pub status: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
// Netty RPCs. Every RPC needs to define a request type and optionally a response type (default to Response).

/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalBlockOpenRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="2")]
    pub promote: ::core::option::Option<bool>,
}
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalBlockOpenResponse {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalBlockCloseRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
}
/// next available id: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalBlockCreateRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="3")]
    pub tier: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="4")]
    pub space_to_reserve: ::core::option::Option<i64>,
    /// If set, only reserve space for the block.
    #[prost(bool, optional, tag="5")]
    pub only_reserve_space: ::core::option::Option<bool>,
}
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalBlockCreateResponse {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalBlockCompleteRequest {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="2")]
    pub cancel: ::core::option::Option<bool>,
}
/// The read/write request type. It can either be an Alluxio block operation or a UFS file operation.
/// next available id: 2
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RequestType {
    AlluxioBlock = 0,
    UfsFile = 1,
}
