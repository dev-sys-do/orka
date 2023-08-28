fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["src/node-agent/agent.proto"], &["src/node-agent"])?;

    tonic_build::configure().compile(
        &[
            "src/scheduler/agent.proto",
            "src/scheduler/controller.proto",
        ],
        &["src/scheduler"],
    )?;

    Ok(())
}
