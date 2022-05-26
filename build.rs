fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/block_worker.proto")?;
    tonic_build::compile_protos("proto/protocol.proto")?;
    tonic_build::compile_protos("proto/sasl_server.proto")?;
    Ok(())
}