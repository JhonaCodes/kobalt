//! Simple text rendering
//!
//! This is a basic text renderer for displaying simple text.
//! For now, it renders text as colored rectangles to demonstrate the widget system.
//! A proper font rendering system (using glyphon or similar) will be added later.

use kobalt_core::types::{Color, Point, Rect};
use crate::shape::RectRenderer;

/// Simple text renderer
///
/// This is a placeholder that renders text as a colored rectangle.
/// Real text rendering will be implemented later with proper font support.
pub struct KobaltTextRenderer {
    rect_renderer: RectRenderer,
}

impl KobaltTextRenderer {
    /// Creates a new text renderer
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            rect_renderer: RectRenderer::new(device, config),
        }
    }

    /// Clears all prepared text
    pub fn clear(&mut self) {
        self.rect_renderer.clear();
    }

    /// Adds text to be rendered (as a colored rectangle placeholder)
    ///
    /// This is a temporary implementation. It shows where text would be rendered.
    pub fn add_text(
        &mut self,
        text: &str,
        position: Point,
        font_size: f32,
        color: Color,
    ) {
        // Make a visible rectangle
        let char_width = font_size * 0.6;
        let width = text.len() as f32 * char_width;
        let height = font_size * 1.2;

        let rect = Rect::new(position.x, position.y, width, height);

        println!("Adding text placeholder at ({}, {}) with size {}x{}, color: {:?}",
                 position.x, position.y, width, height, [color.r, color.g, color.b, color.a]);

        self.rect_renderer.add_rect(rect, color);
    }

    /// Builds all text for rendering
    pub fn build(&mut self, device: &wgpu::Device, screen_size: kobalt_core::types::Size) {
        self.rect_renderer.build_buffers(device, screen_size);
    }

    /// Legacy method for single text rendering
    pub fn prepare_text(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        text: &str,
        position: Point,
        font_size: f32,
        color: Color,
        screen_size: kobalt_core::types::Size,
    ) {
        self.clear();
        self.add_text(text, position, font_size, color);
        self.build(device, screen_size);
    }

    /// Renders the prepared text
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.rect_renderer.render(render_pass);
    }
}
