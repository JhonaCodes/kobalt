//! # Kobalt Render
//!
//! WGPU-based rendering engine for the Kobalt UI framework.
//!
//! This crate provides:
//! - WGPU initialization and management
//! - Surface and swapchain handling
//! - Basic shape rendering (rectangles, circles)
//! - Text rendering
//! - Shader management

mod renderer;
mod shape;
mod text;
mod text_real;

pub use renderer::Renderer;
pub use shape::{RectRenderer, Vertex};
pub use text::KobaltTextRenderer;
pub use text_real::RealTextRenderer;

pub use kobalt_core::types::{Color, Point, Rect, Size};

// Re-export common WGPU types
pub use wgpu::SurfaceError;
