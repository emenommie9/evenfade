use env_logger::{Builder, Env};
use log::info;

fn main() {
    let env = Env::new().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();

    info!(
        "Launching {} (version {})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
