fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = &[
        "proto/block_worker.proto",
        "proto/file_system_master.proto",
        "proto/common.proto",
        "proto/fscommon.proto",
        "proto/journal_file.proto",
        "proto/protocol.proto",
        "proto/sasl_server.proto",
        "proto/shared_acl.proto"];
    tonic_build::configure()
        .build_server(false)
        .compile(
            proto_files,
            &["proto/."],
        ).unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
    // recompile protobufs only if any of the proto files changes.
    for file in proto_files {
        println!("cargo:rerun-if-changed={}", file);
    }
    Ok(())
}