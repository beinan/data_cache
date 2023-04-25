fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = &[
        "grpc/block_worker.proto",
        "grpc/file_system_master.proto",
        "grpc/common.proto",
        "grpc/fscommon.proto",
//        "grpc/journal_file.proto",
//        "grpc/protocol.proto",
        "grpc/sasl_server.proto",
//        "grpc/shared_acl.proto",
    ];
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/gen")
        .compile(proto_files, &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
    // recompile protobufs only if any of the proto files changes.
    for file in proto_files {
        println!("cargo:rerun-if-changed={}", file);
    }
    Ok(())
}
