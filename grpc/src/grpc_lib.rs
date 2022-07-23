pub mod alluxio {
    pub mod grpc {
        tonic::include_proto!("alluxio.grpc");
        pub mod block {
            tonic::include_proto!("alluxio.grpc.block");
        }
        pub mod sasl {
            tonic::include_proto!("alluxio.grpc.sasl");
        }
        pub mod file {
            tonic::include_proto!("alluxio.grpc.file");
        }
        pub mod fscommon {
            tonic::include_proto!("alluxio.grpc.fscommon");
        }
    }
    pub mod proto {
        pub mod dataserver {
            tonic::include_proto!("alluxio.proto.dataserver");
        }
        pub mod journal {
            tonic::include_proto!("alluxio.proto.journal");
        }
        pub mod shared {
            tonic::include_proto!("alluxio.proto.shared");
        }
    }
}