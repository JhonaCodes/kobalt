//! # Kobalt
//!
//! A modern, declarative UI framework for Rust inspired by Flutter and Jetpack Compose.
//!
//! ## Quick Start
//!
//! Kobalt uses **declarative macros** for building UIs, providing a clean Flutter/Compose-style syntax:
//!
//! ```no_run
//! use kobalt::prelude::*;
//! use kobalt_macros::column; // Disambiguation for column! macro
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     app! {
//!         title: text!("Hello Kobalt"),
//!         size: (800, 600),
//!         background: Color::from_rgb8(20, 20, 30),
//!         home: column! {
//!             main_axis_alignment: MainAxisAlignment::Center,
//!             cross_axis_alignment: CrossAxisAlignment::Center,
//!             padding: EdgeInsets::all(20.0),
//!             children: [
//!                 text!("Hello, World!", size: 32.0, color: Color::WHITE),
//!                 text!("Welcome to Kobalt!", size: 24.0)
//!             ]
//!         }
//!     }
//! }
//! ```
//!
//! ## Macro-Based UI (Recommended)
//!
//! Kobalt's macro system provides:
//! - `app!` - Declarative app configuration
//! - `column!` - Vertical layout with alignment and padding
//! - `text!` - Text widgets with styling
//! - `text_style!` - Reusable text styles
//!
//! ## Architecture
//!
//! Kobalt is organized into several crates:
//!
//! - **kobalt-core**: Platform-agnostic core types and traits
//! - **kobalt-widgets**: Standard widget library (Text, Column, etc.)
//! - **kobalt-render**: WGPU-based rendering engine
//! - **kobalt-runtime**: Cross-platform runtime (Desktop, Mobile, Web)
//! - **kobalt-macros**: Declarative macros for ergonomic APIs

// Re-export all public APIs from sub-crates
pub use kobalt_core as core;
pub use kobalt_render as render;
pub use kobalt_runtime as runtime;
pub use kobalt_widgets as widgets;
pub use kobalt_macros as macros;

// Re-export commonly used items for convenience
pub mod prelude {
    // Runtime
    pub use kobalt_runtime::{KobaltApp, DesktopApp, DesktopWindow};
    pub use kobalt_runtime::{Event, WindowEvent};

    // Widget types (available but prefer using macros)
    pub use kobalt_widgets::{Text, TextStyle};
    // Note: Column type is available via kobalt::widgets::Column if needed directly

    // Core types
    pub use kobalt_core::types::{Color, Point, Rect, Size};
    pub use kobalt_core::widget::{Widget, Constraints};
    pub use kobalt_core::layout::{MainAxisAlignment, CrossAxisAlignment, EdgeInsets};
    
    // Macros (preferred way to create UIs - mandatory for layout widgets)
    pub use kobalt_macros::{text, column, app, text_style};
}

// Direct re-exports for top-level convenience
pub use prelude::*;
