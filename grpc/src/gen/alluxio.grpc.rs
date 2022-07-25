#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PMode {
    #[prost(enumeration="Bits", required, tag="1")]
    pub owner_bits: i32,
    #[prost(enumeration="Bits", required, tag="2")]
    pub group_bits: i32,
    #[prost(enumeration="Bits", required, tag="3")]
    pub other_bits: i32,
}
///*
/// Contains the information of a block in Alluxio. It maintains the worker nodes where the replicas
/// of the blocks are stored.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfo {
    #[prost(int64, optional, tag="1")]
    pub block_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub length: ::core::option::Option<i64>,
    #[prost(message, repeated, tag="3")]
    pub locations: ::prost::alloc::vec::Vec<BlockLocation>,
}
///*
/// Information about blocks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockLocation {
    #[prost(int64, optional, tag="1")]
    pub worker_id: ::core::option::Option<i64>,
    #[prost(message, optional, tag="2")]
    pub worker_address: ::core::option::Option<WorkerNetAddress>,
    #[prost(string, optional, tag="3")]
    pub tier_alias: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub medium_type: ::core::option::Option<::prost::alloc::string::String>,
}
///*
/// Information about metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    #[prost(string, optional, tag="1")]
    pub instance: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag="4")]
    pub value: ::core::option::Option<f64>,
    #[prost(enumeration="MetricType", required, tag="5")]
    pub metric_type: i32,
    #[prost(map="string, string", tag="6")]
    pub tags: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigProperty {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    #[prost(enumeration="CommandType", optional, tag="1")]
    pub command_type: ::core::option::Option<i32>,
    #[prost(int64, repeated, packed="false", tag="2")]
    pub data: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalityTier {
    #[prost(string, optional, tag="1")]
    pub tier_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TieredIdentity {
    #[prost(message, repeated, tag="1")]
    pub tiers: ::prost::alloc::vec::Vec<LocalityTier>,
}
///*
/// Address information about masters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetAddress {
    #[prost(string, optional, tag="1")]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="2")]
    pub rpc_port: ::core::option::Option<i32>,
}
///*
/// Address information about workers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerNetAddress {
    #[prost(string, optional, tag="1")]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="2")]
    pub rpc_port: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub data_port: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub web_port: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub domain_socket_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub tiered_identity: ::core::option::Option<TieredIdentity>,
    #[prost(string, optional, tag="7")]
    pub container_host: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockStoreLocationProto {
    #[prost(string, optional, tag="1")]
    pub tier_alias: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub medium_type: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Bits {
    None = 1,
    Execute = 2,
    Write = 3,
    WriteExecute = 4,
    Read = 5,
    ReadExecute = 6,
    ReadWrite = 7,
    All = 8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricType {
    /// GAUGE is the simplest type of metric. If you're not sure which to use, gauge is a safe choice. It is represents a
    /// general K-V pair.
    Gauge = 0,
    /// COUNTER represents values which can be incremented or decremented over time by certain operations. It does not rely
    /// on timing to determine the value.
    Counter = 1,
    /// METER represents a metric value at a _rate_. The value of the metric varies with the time over which events are
    /// recorded
    Meter = 2,
    /// TIMER represents a histogram of the rate of the specified events.
    Timer = 3,
    /// EXECUTOR_SERVICE represents an executor service.
    ExecutorService = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandType {
    Unknown = 0,
    Nothing = 1,
    /// Ask the worker to re-register.
    Register = 2,
    /// Ask the worker to free files.
    Free = 3,
    /// Ask the worker to delete files.
    Delete = 4,
    /// Ask the worker to persist a file for lineage
    Persist = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TtlAction {
    /// Delete the file after TTL expires.
    Delete = 0,
    /// Free the file after TTL expires.
    Free = 1,
}
