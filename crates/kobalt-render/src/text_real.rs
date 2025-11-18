//! Real text rendering using fontdue
//!
//! This module provides actual text rendering with fonts, replacing the placeholder rectangles.

use bytemuck::{Pod, Zeroable};
use fontdue::{Font, FontSettings};
use kobalt_core::types::{Color, Point};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

/// Vertex for text rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct TextVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 4],
}

impl TextVertex {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

/// A rasterized glyph in the atlas
struct GlyphInfo {
    atlas_x: u32,
    atlas_y: u32,
    width: u32,
    height: u32,
    bearing_x: i32,
    bearing_y: i32,
    advance: f32,
}

/// Real text renderer using fontdue
pub struct RealTextRenderer {
    font: Font,
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    atlas_texture: wgpu::Texture,
    atlas_view: wgpu::TextureView,
    atlas_sampler: wgpu::Sampler,
    bind_group: wgpu::BindGroup,
    glyph_cache: HashMap<char, GlyphInfo>,
    atlas_width: u32,
    atlas_height: u32,
    next_x: u32,
    next_y: u32,
    current_row_height: u32,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    num_indices: u32,
}

impl RealTextRenderer {
    /// Creates a new real text renderer with a default font
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        // Load default font (using a basic embedded font for now)
        // In a real app, you'd load from a file or embed a proper font
        let font_data = include_bytes!("../assets/fonts/Roboto-Regular.ttf");
        let font = Font::from_bytes(&font_data[..], FontSettings::default())
            .expect("Failed to load font");

        // Create texture atlas for glyphs (1024x1024 should be enough for most cases)
        let atlas_width = 1024;
        let atlas_height = 1024;

        let atlas_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Text Atlas"),
            size: wgpu::Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let atlas_view = atlas_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let atlas_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Text Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&atlas_sampler),
                },
            ],
        });

        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: wgpu::ShaderSource::Wgsl(TEXT_SHADER.into()),
        });

        // Create pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[TextVertex::desc()],
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
                cull_mode: None,
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

        Self {
            font,
            pipeline,
            bind_group_layout,
            atlas_texture,
            atlas_view,
            atlas_sampler,
            bind_group,
            glyph_cache: HashMap::new(),
            atlas_width,
            atlas_height,
            next_x: 0,
            next_y: 0,
            current_row_height: 0,
            vertex_buffer: None,
            index_buffer: None,
            num_indices: 0,
        }
    }

    /// Rasterizes a glyph and adds it to the atlas
    fn rasterize_glyph(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        ch: char,
        font_size: f32,
    ) -> &GlyphInfo {
        if !self.glyph_cache.contains_key(&ch) {
            let (metrics, bitmap) = self.font.rasterize(ch, font_size);

            // Check if we need to move to next row
            if self.next_x + metrics.width as u32 > self.atlas_width {
                self.next_x = 0;
                self.next_y += self.current_row_height;
                self.current_row_height = 0;
            }

            // Update current row height
            self.current_row_height = self.current_row_height.max(metrics.height as u32);

            // Copy glyph to atlas
            if !bitmap.is_empty() {
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &self.atlas_texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d {
                            x: self.next_x,
                            y: self.next_y,
                            z: 0,
                        },
                        aspect: wgpu::TextureAspect::All,
                    },
                    &bitmap,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(metrics.width as u32),
                        rows_per_image: Some(metrics.height as u32),
                    },
                    wgpu::Extent3d {
                        width: metrics.width as u32,
                        height: metrics.height as u32,
                        depth_or_array_layers: 1,
                    },
                );
            }

            let glyph_info = GlyphInfo {
                atlas_x: self.next_x,
                atlas_y: self.next_y,
                width: metrics.width as u32,
                height: metrics.height as u32,
                bearing_x: metrics.xmin,
                bearing_y: metrics.ymin,
                advance: metrics.advance_width,
            };

            self.next_x += metrics.width as u32;
            self.glyph_cache.insert(ch, glyph_info);
        }

        self.glyph_cache.get(&ch).unwrap()
    }

    /// Prepares text for rendering
    pub fn prepare_text(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        text: &str,
        position: Point,
        font_size: f32,
        color: Color,
        screen_width: f32,
        screen_height: f32,
    ) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut cursor_x = position.x;
        let cursor_y = position.y;
        let atlas_width = self.atlas_width;
        let atlas_height = self.atlas_height;

        for ch in text.chars() {
            let glyph = self.rasterize_glyph(device, queue, ch, font_size);

            if glyph.width > 0 && glyph.height > 0 {
                let x = cursor_x + glyph.bearing_x as f32;
                let y = cursor_y - glyph.bearing_y as f32;

                // Convert to NDC
                let x0 = (x / screen_width) * 2.0 - 1.0;
                let y0 = 1.0 - (y / screen_height) * 2.0;
                let x1 = ((x + glyph.width as f32) / screen_width) * 2.0 - 1.0;
                let y1 = 1.0 - ((y + glyph.height as f32) / screen_height) * 2.0;

                // Texture coordinates
                let u0 = glyph.atlas_x as f32 / atlas_width as f32;
                let v0 = glyph.atlas_y as f32 / atlas_height as f32;
                let u1 = (glyph.atlas_x + glyph.width) as f32 / atlas_width as f32;
                let v1 = (glyph.atlas_y + glyph.height) as f32 / atlas_height as f32;

                let color_array = [color.r, color.g, color.b, color.a];
                let base_index = vertices.len() as u16;

                // Add vertices
                vertices.push(TextVertex {
                    position: [x0, y0],
                    tex_coords: [u0, v0],
                    color: color_array,
                });
                vertices.push(TextVertex {
                    position: [x1, y0],
                    tex_coords: [u1, v0],
                    color: color_array,
                });
                vertices.push(TextVertex {
                    position: [x1, y1],
                    tex_coords: [u1, v1],
                    color: color_array,
                });
                vertices.push(TextVertex {
                    position: [x0, y1],
                    tex_coords: [u0, v1],
                    color: color_array,
                });

                // Add indices
                indices.extend_from_slice(&[
                    base_index,
                    base_index + 1,
                    base_index + 2,
                    base_index,
                    base_index + 2,
                    base_index + 3,
                ]);
            }

            cursor_x += glyph.advance;
        }

        if !vertices.is_empty() {
            self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Text Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }));

            self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Text Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }));

            self.num_indices = indices.len() as u32;
        }
    }

    /// Renders the prepared text
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.num_indices > 0 {
            if let (Some(vertex_buffer), Some(index_buffer)) =
                (&self.vertex_buffer, &self.index_buffer)
            {
                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            }
        }
    }
}

const TEXT_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 0.0, 1.0);
    output.tex_coords = input.tex_coords;
    output.color = input.color;
    return output;
}

@group(0) @binding(0)
var atlas_texture: texture_2d<f32>;
@group(0) @binding(1)
var atlas_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = textureSample(atlas_texture, atlas_sampler, input.tex_coords).r;
    return vec4<f32>(input.color.rgb, input.color.a * alpha);
}
"#;
