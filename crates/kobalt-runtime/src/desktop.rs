//! Desktop platform implementation using winit

use kobalt_core::types::Size;
use kobalt_render::Renderer;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

/// Desktop window wrapper
pub struct DesktopWindow {
    window: Arc<Window>,
    renderer: Option<Renderer>,
}

impl DesktopWindow {
    /// Creates a new desktop window
    async fn new(event_loop: &ActiveEventLoop, title: &str, width: u32, height: u32) -> Self {
        let attributes = WindowAttributes::default()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));

        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        // Create WGPU instance and surface
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let size = Size::new(width as f32, height as f32);
        let renderer = Renderer::new_with_instance(instance, surface, size).await;

        Self {
            window,
            renderer: Some(renderer),
        }
    }

    /// Returns the window
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Returns the renderer
    pub fn renderer(&self) -> &Renderer {
        self.renderer.as_ref().unwrap()
    }

    /// Returns a mutable reference to the renderer
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        self.renderer.as_mut().unwrap()
    }

    /// Resizes the window
    pub fn resize(&mut self, width: u32, height: u32) {
        if let Some(renderer) = &mut self.renderer {
            renderer.resize(Size::new(width as f32, height as f32));
        }
    }
}

/// Desktop application
pub struct DesktopApp<F>
where
    F: FnMut(&mut DesktopWindow, &Event<()>) + 'static,
{
    window: Option<DesktopWindow>,
    on_event: F,
    window_config: WindowConfig,
}

struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
}

impl<F> DesktopApp<F>
where
    F: FnMut(&mut DesktopWindow, &Event<()>) + 'static,
{
    /// Creates a new desktop application
    pub fn new(title: &str, width: u32, height: u32, on_event: F) -> Self {
        Self {
            window: None,
            on_event,
            window_config: WindowConfig {
                title: title.to_string(),
                width,
                height,
            },
        }
    }

    /// Runs the application
    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl<F> ApplicationHandler for DesktopApp<F>
where
    F: FnMut(&mut DesktopWindow, &Event<()>) + 'static,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = pollster::block_on(DesktopWindow::new(
                event_loop,
                &self.window_config.title,
                self.window_config.width,
                self.window_config.height,
            ));
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = &mut self.window {
            if window.window.id() == window_id {
                match event {
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        window.resize(physical_size.width, physical_size.height);
                    }
                    WindowEvent::RedrawRequested => {
                        // Trigger custom event handler
                        (self.on_event)(
                            window,
                            &Event::WindowEvent {
                                window_id,
                                event: WindowEvent::RedrawRequested,
                            },
                        );
                        window.window.request_redraw();
                    }
                    _ => {}
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.window.request_redraw();
        }
    }
}
