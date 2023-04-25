pub mod auth;
pub mod grpc_client;

pub mod alluxio {
    pub mod grpc {
        include!("gen/alluxio.grpc.rs");
        pub mod block {
            include!("gen/alluxio.grpc.block.rs");
        }
        pub mod sasl {
            include!("gen/alluxio.grpc.sasl.rs");
        }
        pub mod file {
            include!("gen/alluxio.grpc.file.rs");
        }
        pub mod fscommon {
            include!("gen/alluxio.grpc.fscommon.rs");
        }
    }
    pub mod proto {
        pub mod dataserver {
            include!("gen/alluxio.proto.dataserver.rs");
        }
        pub mod journal {
            include!("gen/alluxio.proto.journal.rs");
        }
        pub mod shared {
            include!("gen/alluxio.proto.shared.rs");
        }
        pub mod status {
            include!("gen/alluxio.proto.status.rs");
        }
    }
}
