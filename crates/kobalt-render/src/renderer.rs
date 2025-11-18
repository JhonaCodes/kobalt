//! Core renderer implementation

use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use kobalt_core::types::{Color, Size};

/// Main renderer struct that manages WGPU resources
pub struct Renderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: Size,
}

impl Renderer {
    /// Creates a new renderer with the given surface and size
    pub async fn new(surface: Surface<'static>, size: Size) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        Self::new_with_instance(instance, surface, size).await
    }

    /// Creates a new renderer with a provided WGPU instance
    pub async fn new_with_instance(
        instance: wgpu::Instance,
        surface: Surface<'static>,
        size: Size,
    ) -> Self {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Kobalt Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    experimental_features: Default::default(),
                    trace: Default::default(),
                },
            )
            .await
            .expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    /// Returns a reference to the WGPU device
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Returns a reference to the WGPU queue
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Returns the current surface configuration
    pub fn config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    /// Returns the current size
    pub fn size(&self) -> Size {
        self.size
    }

    /// Resizes the renderer
    pub fn resize(&mut self, new_size: Size) {
        if new_size.width > 0.0 && new_size.height > 0.0 {
            self.size = new_size;
            self.config.width = new_size.width as u32;
            self.config.height = new_size.height as u32;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Begins a new frame and returns the render target
    pub fn begin_frame(&self) -> Result<FrameContext<'_>, wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Ok(FrameContext {
            output,
            view,
            device: &self.device,
            queue: &self.queue,
            config: &self.config,
        })
    }
}

/// Context for rendering a single frame
pub struct FrameContext<'a> {
    output: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
    device: &'a Device,
    queue: &'a Queue,
    config: &'a SurfaceConfiguration,
}

impl<'a> FrameContext<'a> {
    /// Returns the texture view for this frame
    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    /// Returns the device
    pub fn device(&self) -> &Device {
        self.device
    }

    /// Returns the queue
    pub fn queue(&self) -> &Queue {
        self.queue
    }

    /// Returns the surface configuration
    pub fn config(&self) -> &SurfaceConfiguration {
        self.config
    }

    /// Creates a command encoder for this frame
    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Frame Command Encoder"),
            })
    }

    /// Begins a render pass with a clear color
    pub fn begin_render_pass(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        clear_color: Color,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: clear_color.r as f64,
                        g: clear_color.g as f64,
                        b: clear_color.b as f64,
                        a: clear_color.a as f64,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        })
    }

    /// Submits the command encoder and presents the frame
    pub fn present(self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
        self.output.present();
    }
}
