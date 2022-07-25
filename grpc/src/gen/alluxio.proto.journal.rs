// Journal entry messages for the file master.

/// A pair of strings, useful for maps.
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringPairEntry {
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveSyncTxIdEntry {
    #[prost(int64, optional, tag="1")]
    pub mount_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub tx_id: ::core::option::Option<i64>,
}
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSyncPointEntry {
    #[prost(string, optional, tag="1")]
    pub syncpoint_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="2")]
    pub mount_id: ::core::option::Option<i64>,
}
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSyncPointEntry {
    #[prost(string, optional, tag="1")]
    pub syncpoint_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="2")]
    pub mount_id: ::core::option::Option<i64>,
}
/// next available id: 7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMountPointEntry {
    #[prost(string, optional, tag="1")]
    pub alluxio_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="3")]
    pub read_only: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="4")]
    pub properties: ::prost::alloc::vec::Vec<StringPairEntry>,
    #[prost(bool, optional, tag="5")]
    pub shared: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="6")]
    pub mount_id: ::core::option::Option<i64>,
}
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncPersistRequestEntry {
    #[prost(int64, optional, tag="1")]
    pub file_id: ::core::option::Option<i64>,
}
/// next available id: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteFileEntry {
    #[prost(int64, repeated, packed="false", tag="1")]
    pub block_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, optional, tag="2")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub length: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub op_time_ms: ::core::option::Option<i64>,
    #[prost(string, optional, tag="5")]
    pub ufs_fingerprint: ::core::option::Option<::prost::alloc::string::String>,
}
/// next available id: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFileEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    /// Deprecated, we now write one journal entry per inode removed
    #[prost(bool, optional, tag="2")]
    pub recursive: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="3")]
    pub op_time_ms: ::core::option::Option<i64>,
    /// Deprecated, this field is about whether to delete in the UFS. We don't journal UFS changes.
    #[prost(bool, optional, tag="4")]
    pub alluxio_only: ::core::option::Option<bool>,
    #[prost(string, optional, tag="5")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMountPointEntry {
    #[prost(string, optional, tag="1")]
    pub alluxio_path: ::core::option::Option<::prost::alloc::string::String>,
}
/// Creates a new block for a file inode.
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlockEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
}
/// next available id: 21
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInodeEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub parent_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub persistence_state: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="5")]
    pub pinned: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="6")]
    pub creation_time_ms: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="7")]
    pub last_modification_time_ms: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="8")]
    pub overwrite_modification_time: ::core::option::Option<bool>,
    #[prost(string, optional, tag="9")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="11")]
    pub mode: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="12")]
    pub ttl: ::core::option::Option<i64>,
    #[prost(enumeration="PTtlAction", optional, tag="13", default="Delete")]
    pub ttl_action: ::core::option::Option<i32>,
    #[prost(message, optional, tag="14")]
    pub acl: ::core::option::Option<super::shared::AccessControlList>,
    #[prost(string, optional, tag="15")]
    pub ufs_fingerprint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="16")]
    pub medium_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, bytes", tag="17")]
    pub x_attr: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag="18")]
    pub last_access_time_ms: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="19")]
    pub overwrite_access_time: ::core::option::Option<bool>,
    #[prost(enumeration="XAttrUpdateStrategy", optional, tag="20", default="Truncate")]
    pub x_attr_update_strategy: ::core::option::Option<i32>,
}
/// next available id: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInodeDirectoryEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="2")]
    pub mount_point: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub direct_children_loaded: ::core::option::Option<bool>,
    #[prost(message, optional, tag="4")]
    pub default_acl: ::core::option::Option<super::shared::AccessControlList>,
}
/// next available id: 13
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInodeFileEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub block_size_bytes: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub length: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="4")]
    pub completed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="5")]
    pub cacheable: ::core::option::Option<bool>,
    /// Overwrite the blocks list
    #[prost(int64, repeated, packed="false", tag="7")]
    pub set_blocks: ::prost::alloc::vec::Vec<i64>,
    #[prost(int32, optional, tag="8")]
    pub replication_max: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub replication_min: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="10")]
    pub persist_job_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag="11")]
    pub temp_ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// next available id: 21
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InodeDirectoryEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub parent_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub persistence_state: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="5")]
    pub pinned: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="6")]
    pub creation_time_ms: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="7")]
    pub last_modification_time_ms: ::core::option::Option<i64>,
    #[prost(string, optional, tag="8")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="10")]
    pub mode: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="11")]
    pub mount_point: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="12")]
    pub direct_children_loaded: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="13")]
    pub ttl: ::core::option::Option<i64>,
    #[prost(enumeration="PTtlAction", optional, tag="14", default="Delete")]
    pub ttl_action: ::core::option::Option<i32>,
    #[prost(message, optional, tag="15")]
    pub acl: ::core::option::Option<super::shared::AccessControlList>,
    #[prost(message, optional, tag="16")]
    pub default_acl: ::core::option::Option<super::shared::AccessControlList>,
    #[prost(string, optional, tag="17")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="18")]
    pub medium_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, bytes", tag="19")]
    pub x_attr: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag="20")]
    pub last_access_time_ms: ::core::option::Option<i64>,
}
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InodeDirectoryIdGeneratorEntry {
    #[prost(int64, optional, tag="1")]
    pub container_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub sequence_number: ::core::option::Option<i64>,
}
/// next available id: 30
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InodeFileEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub parent_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub persistence_state: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="5")]
    pub pinned: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="6")]
    pub creation_time_ms: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="7")]
    pub last_modification_time_ms: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="8")]
    pub block_size_bytes: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="9")]
    pub length: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="10")]
    pub completed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="11")]
    pub cacheable: ::core::option::Option<bool>,
    #[prost(int64, repeated, packed="false", tag="12")]
    pub blocks: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, optional, tag="13")]
    pub ttl: ::core::option::Option<i64>,
    #[prost(string, optional, tag="14")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="16")]
    pub mode: ::core::option::Option<i32>,
    #[prost(enumeration="PTtlAction", optional, tag="17", default="Delete")]
    pub ttl_action: ::core::option::Option<i32>,
    #[prost(string, optional, tag="18")]
    pub ufs_fingerprint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="19")]
    pub acl: ::core::option::Option<super::shared::AccessControlList>,
    #[prost(int32, optional, tag="20")]
    pub replication_max: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="21")]
    pub replication_min: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="22")]
    pub persist_job_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag="23")]
    pub temp_ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="24")]
    pub replication_durable: ::core::option::Option<i32>,
    #[prost(string, optional, tag="25")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="26")]
    pub medium_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="27")]
    pub should_persist_time: ::core::option::Option<i64>,
    #[prost(map="string, bytes", tag="28")]
    pub x_attr: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag="29")]
    pub last_access_time_ms: ::core::option::Option<i64>,
}
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InodeLastModificationTimeEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub last_modification_time_ms: ::core::option::Option<i64>,
}
/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistDirectoryEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
}
/// next available id: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistFileEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub length: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub op_time_ms: ::core::option::Option<i64>,
}
/// next available id: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    /// Deprecated, use new_parent_id/new_name instead
    #[prost(string, optional, tag="2")]
    pub dst_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="3")]
    pub op_time_ms: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub new_parent_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag="5")]
    pub new_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub new_path: ::core::option::Option<::prost::alloc::string::String>,
}
/// next available id: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAclEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub op_time_ms: ::core::option::Option<i64>,
    #[prost(enumeration="PSetAclAction", optional, tag="3")]
    pub action: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="4")]
    pub entries: ::prost::alloc::vec::Vec<super::shared::AclEntry>,
    #[prost(bool, optional, tag="5")]
    pub recursive: ::core::option::Option<bool>,
}
/// next available id: 15
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttributeEntry {
    #[prost(int64, optional, tag="1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub op_time_ms: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="3")]
    pub pinned: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="4")]
    pub ttl: ::core::option::Option<i64>,
    #[prost(bool, optional, tag="5")]
    pub persisted: ::core::option::Option<bool>,
    #[prost(string, optional, tag="6")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="8")]
    pub permission: ::core::option::Option<i32>,
    #[prost(enumeration="PTtlAction", optional, tag="9", default="Delete")]
    pub ttl_action: ::core::option::Option<i32>,
    #[prost(string, optional, tag="10")]
    pub ufs_fingerprint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="11")]
    pub persist_job_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag="12")]
    pub temp_ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="13")]
    pub replication_max: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="14")]
    pub replication_min: ::core::option::Option<i32>,
}
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUfsModeEntry {
    #[prost(string, optional, tag="1")]
    pub ufs_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="UfsMode", optional, tag="2", default="ReadWrite")]
    pub ufs_mode: ::core::option::Option<i32>,
}
/// XAttrUpdateStrategy controls the behaviour for
/// handling updates to the xAttr map
/// - TRUNCATE:       Replaces the existing xAttr map with the provided xAttr
/// - UNION_REPLACE:  Inserts all keys from xAttr and overwrites any existing keys
/// - UNION_PRESERVE: Inserts all new keys from xAttr (i.e: preserves any existing keys)
/// - DELETE_KEYS:    Deletes the specified xAttr keys from the existing xAttr map
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum XAttrUpdateStrategy {
    Truncate = 1,
    UnionReplace = 2,
    UnionPreserve = 3,
    DeleteKeys = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PTtlAction {
    Delete = 0,
    Free = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PSetAclAction {
    Replace = 0,
    Modify = 1,
    Remove = 2,
    RemoveAll = 3,
    RemoveDefault = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UfsMode {
    NoAccess = 0,
    ReadOnly = 1,
    ReadWrite = 2,
}
