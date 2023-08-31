fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &["src/workload_manager/container/metrics/metrics.proto"],
        &["src/workload_manager/container/metrics"],
    )?;
    Ok(())
}
