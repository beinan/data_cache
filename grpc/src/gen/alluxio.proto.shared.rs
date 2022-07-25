/// next available id: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AclActions {
    #[prost(enumeration="AclAction", repeated, packed="false", tag="1")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
}
/// next available id: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AclEntry {
    #[prost(enumeration="AclEntryType", optional, tag="1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub subject: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="AclAction", repeated, packed="false", tag="3")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag="4")]
    pub is_default: ::core::option::Option<bool>,
}
/// AclActions for a String name.
/// next available id: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedAclActions {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub actions: ::core::option::Option<AclActions>,
}
/// next available id: 9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessControlList {
    #[prost(string, optional, tag="1")]
    pub owning_user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub owning_group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub user_actions: ::prost::alloc::vec::Vec<NamedAclActions>,
    #[prost(message, repeated, tag="4")]
    pub group_actions: ::prost::alloc::vec::Vec<NamedAclActions>,
    #[prost(message, optional, tag="5")]
    pub mask_actions: ::core::option::Option<AclActions>,
    #[prost(message, optional, tag="6")]
    pub other_actions: ::core::option::Option<AclActions>,
    #[prost(bool, optional, tag="7")]
    pub is_default: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="8")]
    pub is_empty: ::core::option::Option<bool>,
}
/// next available id: 3
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AclAction {
    Read = 0,
    Write = 1,
    Execute = 2,
}
/// next available id: 6
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AclEntryType {
    Owner = 0,
    NamedUser = 1,
    OwningGroup = 2,
    NamedGroup = 3,
    Mask = 4,
    Other = 5,
}
