use std::sync::Arc;

use pollster::FutureExt;
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::rendering::rendering::Rendering;

/// Keeps track of all main components of the game such as the window and all subsystems.
pub struct App {
    #[allow(dead_code)]
    window: Arc<Window>,
    rendering: Rendering,
}

impl App {
    /// Initialize the game, create a window and create subsystems.
    pub fn initialize(event_loop: &ActiveEventLoop) -> Self {
        let window_attributes = Window::default_attributes().with_title("Evenfade");

        let window = event_loop.create_window(window_attributes).unwrap();
        let window = Arc::new(window);
        window.request_redraw();

        let rendering = Rendering::initialize(window.clone()).block_on();

        Self { window, rendering }
    }

    /// Handle a window event from winit and delegate the events to the appropriate subsystems.
    pub fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                self.rendering.window_resized(physical_size);
            }
            WindowEvent::RedrawRequested => {
                self.rendering.render();
            }
            _ => (),
        }
    }
}
