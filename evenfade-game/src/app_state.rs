use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::app::App;

pub enum AppState {
    Uninitialized,
    Initialized { app: App },
}

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
