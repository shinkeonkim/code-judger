fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/proto/judger.proto")?;
    tonic_build::compile_protos("src/proto/executor.proto")?;
    Ok(())
} 