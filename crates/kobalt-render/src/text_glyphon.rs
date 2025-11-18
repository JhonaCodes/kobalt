//! Professional text rendering using glyphon
//!
//! This module provides high-quality text rendering using the glyphon library,
//! which is the standard solution for text in WGPU applications.

use glyphon::{
    Attrs, Buffer, Color as GlyphonColor, Family, FontSystem, Metrics, Resolution, Shaping,
    SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer,
};
use kobalt_core::types::{Color, Point};

/// Text renderer using glyphon
pub struct GlyphonTextRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: TextAtlas,
    text_renderer: TextRenderer,
    buffers: Vec<(Buffer, Point, Color)>,
    viewport_width: u32,
    viewport_height: u32,
}

impl GlyphonTextRenderer {
    /// Creates a new glyphon text renderer
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = glyphon::Cache::new(device);
        let mut atlas = TextAtlas::new(device, queue, &cache, config.format);
        let text_renderer = TextRenderer::new(
            &mut atlas,
            device,
            wgpu::MultisampleState::default(),
            None,
        );

        Self {
            font_system,
            swash_cache,
            atlas,
            text_renderer,
            buffers: Vec::new(),
            viewport_width: config.width,
            viewport_height: config.height,
        }
    }

    /// Adds text to be rendered
    pub fn add_text(
        &mut self,
        text: &str,
        position: Point,
        font_size: f32,
        color: Color,
    ) {
        let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(font_size, font_size * 1.2));

        buffer.set_size(
            &mut self.font_system,
            Some(self.viewport_width as f32),
            Some(self.viewport_height as f32),
        );

        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );

        self.buffers.push((buffer, position, color));
    }

    /// Prepares all text for rendering
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), String> {
        self.viewport_width = screen_width;
        self.viewport_height = screen_height;

        let mut text_areas = Vec::new();

        for (buffer, position, color) in &self.buffers {
            text_areas.push(TextArea {
                buffer,
                left: position.x,
                top: position.y,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: screen_width as i32,
                    bottom: screen_height as i32,
                },
                default_color: GlyphonColor::rgb(
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                ),
            });
        }

        self.text_renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                Resolution {
                    width: screen_width,
                    height: screen_height,
                },
                text_areas,
                &mut self.swash_cache,
            )
            .map_err(|e| format!("Failed to prepare text: {:?}", e))
    }

    /// Clears all text
    pub fn clear(&mut self) {
        self.buffers.clear();
    }

    /// Renders the text
    pub fn render<'rpass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'rpass>) -> Result<(), String> {
        self.text_renderer
            .render(&self.atlas, render_pass)
            .map_err(|e| format!("Failed to render text: {:?}", e))
    }

    /// Trims the atlas
    pub fn trim_atlas(&mut self) {
        self.atlas.trim();
    }
}
