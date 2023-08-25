fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/src/scheduler/controller.proto")?;
    Ok(())
}
