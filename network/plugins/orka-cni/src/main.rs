use cni_plugin::logger;
use tokio::runtime::Runtime;

fn main() {
    logger::install(env!("CARGO_PKG_NAME"));

    if let Ok(runtime) = Runtime::new() {
        runtime.block_on(async move { orka_cni::run().await });
    };
}
