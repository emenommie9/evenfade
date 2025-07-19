use env_logger::{Builder, Env};
use log::info;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::app_state::AppState;

mod app;
mod app_state;
mod rendering;

fn main() {
    let env = Env::new().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();

    info!(
        "Launching {} (version {})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = AppState::Uninitialized;
    event_loop.run_app(&mut app).unwrap();
}
