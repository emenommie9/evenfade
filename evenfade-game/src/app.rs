use std::sync::Arc;

use pollster::FutureExt;
use wgpu::{
    Backends, Color, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits,
    LoadOp, Operations, PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration, TextureUsages,
    TextureViewDescriptor, Trace, wgt::CommandEncoderDescriptor,
};
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, window::Window,
};

pub enum AppState {
    Uninitialized,
    Initialized { app: App },
}

pub struct App {
    #[allow(dead_code)]
    window: Arc<Window>,
    rendering: Rendering,
}

struct Rendering {
    window: Arc<Window>,
    surface: Surface<'static>,
    is_surface_configured: bool,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let app = App::initialize(event_loop);
        *self = AppState::Initialized { app };
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let app = match self {
            AppState::Initialized { app } => app,
            AppState::Uninitialized => return,
        };

        app.window_event(event_loop, window_id, event);
    }
}

impl App {
    fn initialize(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let window_attributes = Window::default_attributes().with_title("Evenfade");

        let window = event_loop.create_window(window_attributes).unwrap();
        let window = Arc::new(window);
        window.request_redraw();

        let rendering = Rendering::initialize(window.clone()).block_on();

        Self { window, rendering }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
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

impl Rendering {
    async fn initialize(window: Arc<Window>) -> Self {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: Default::default(),
                trace: Trace::Off,
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            window: window.clone(),
            surface,
            is_surface_configured: false,
            device,
            queue,
            config,
        }
    }

    fn window_resized(&mut self, physical_size: PhysicalSize<u32>) {
        self.config.width = physical_size.width;
        self.config.height = physical_size.height;

        self.surface.configure(&self.device, &self.config);

        self.is_surface_configured = true;
    }

    fn render(&mut self) {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return;
        }

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
