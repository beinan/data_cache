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
