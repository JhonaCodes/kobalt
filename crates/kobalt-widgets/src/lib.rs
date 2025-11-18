//! # Kobalt Widgets
//!
//! Official widget library for the Kobalt UI framework.
//!
//! This crate provides the standard widgets like Text, Button, Container, etc.

mod text;
mod column;

pub use text::{Text, TextStyle};
pub use column::Column;

// Re-export core types for convenience
pub use kobalt_core::types::{Color, Point, Rect, Size};
pub use kobalt_core::widget::{Constraints, Widget};
pub use kobalt_core::layout::{MainAxisAlignment, CrossAxisAlignment, EdgeInsets};
