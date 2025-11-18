//! Simple shape rendering (rectangles, etc.)

use bytemuck::{Pod, Zeroable};
use kobalt_core::types::{Color, Rect, Size};
use wgpu::util::DeviceExt;

/// Vertex data for rendering shapes
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

/// Renderer for rectangle shapes
pub struct RectRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    /// All rectangles to render
    rectangles: Vec<(Rect, Color)>,
}

impl RectRenderer {
    /// Creates a new rectangle renderer
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Rect Shader"),
            source: wgpu::ShaderSource::Wgsl(RECT_SHADER.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Rect Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rect Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,  // ✅ Disabled culling so rect is visible from both sides
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Create initial empty buffers
        let vertices: Vec<Vertex> = vec![];
        let indices: Vec<u16> = vec![];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices: 0,
            rectangles: Vec::new(),
        }
    }

    /// Converts screen coordinates to NDC (Normalized Device Coordinates)
    fn to_ndc(x: f32, y: f32, screen_size: Size) -> [f32; 2] {
        [
            (x / screen_size.width) * 2.0 - 1.0,
            1.0 - (y / screen_size.height) * 2.0,
        ]
    }

    /// Clears all rectangles
    pub fn clear(&mut self) {
        self.rectangles.clear();
    }

    /// Adds a rectangle to be rendered
    pub fn add_rect(&mut self, rect: Rect, color: Color) {
        self.rectangles.push((rect, color));
    }

    /// Updates the renderer with new rectangle data (legacy method for single rect)
    pub fn update(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue, rect: Rect, color: Color, screen_size: Size) {
        let color_array = [color.r, color.g, color.b, color.a];

        println!("Rect: ({},{}) {}x{}, Color: {:?}, Screen: {}x{}",
                 rect.x, rect.y, rect.width, rect.height, color_array, screen_size.width, screen_size.height);

        let vertices = vec![
            Vertex {
                position: Self::to_ndc(rect.x, rect.y, screen_size),
                color: color_array,
            },
            Vertex {
                position: Self::to_ndc(rect.x + rect.width, rect.y, screen_size),
                color: color_array,
            },
            Vertex {
                position: Self::to_ndc(rect.x + rect.width, rect.y + rect.height, screen_size),
                color: color_array,
            },
            Vertex {
                position: Self::to_ndc(rect.x, rect.y + rect.height, screen_size),
                color: color_array,
            },
        ];

        let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

        // Recreate buffers with new data
        self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.num_indices = indices.len() as u32;
    }

    /// Builds buffers with all accumulated rectangles
    pub fn build_buffers(&mut self, device: &wgpu::Device, screen_size: Size) {
        if self.rectangles.is_empty() {
            self.num_indices = 0;
            return;
        }

        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();

        for (rect, color) in &self.rectangles {
            let color_array = [color.r, color.g, color.b, color.a];
            let base_index = all_vertices.len() as u16;

            // Add vertices for this rectangle
            all_vertices.push(Vertex {
                position: Self::to_ndc(rect.x, rect.y, screen_size),
                color: color_array,
            });
            all_vertices.push(Vertex {
                position: Self::to_ndc(rect.x + rect.width, rect.y, screen_size),
                color: color_array,
            });
            all_vertices.push(Vertex {
                position: Self::to_ndc(rect.x + rect.width, rect.y + rect.height, screen_size),
                color: color_array,
            });
            all_vertices.push(Vertex {
                position: Self::to_ndc(rect.x, rect.y + rect.height, screen_size),
                color: color_array,
            });

            // Add indices for this rectangle (two triangles)
            all_indices.push(base_index);
            all_indices.push(base_index + 1);
            all_indices.push(base_index + 2);
            all_indices.push(base_index);
            all_indices.push(base_index + 2);
            all_indices.push(base_index + 3);
        }

        // Create buffers with all rectangles
        self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Vertex Buffer"),
            contents: bytemuck::cast_slice(&all_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Index Buffer"),
            contents: bytemuck::cast_slice(&all_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.num_indices = all_indices.len() as u32;
    }

    /// Renders the rectangle
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.num_indices > 0 {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
    }
}

const RECT_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 0.0, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
"#;
