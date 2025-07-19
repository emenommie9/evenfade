use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::app::App;

/// Keeps track of when the window has been resumed by winit.
pub enum AppState {
    /// The window is not yet shown (resumed has not been called by winit yet).
    Uninitialized,
    /// The window is shown (resumed has been called by winit). The application has been initialized.
    Initialized { app: App },
}

/// Handles lifecycle events from winit.
impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let app = App::initialize(event_loop);
        *self = AppState::Initialized { app };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let app = match self {
            AppState::Initialized { app } => app,
            AppState::Uninitialized => return,
        };

        app.window_event(event_loop, window_id, event);
    }
}
