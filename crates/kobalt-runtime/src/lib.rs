//! # Kobalt Runtime
//!
//! Cross-platform runtime for the Kobalt UI framework.
//!
//! This crate provides platform integration for:
//! - Desktop (via winit)
//! - Mobile (Android/iOS)
//! - Web (WASM)

mod desktop;
mod app;

pub use desktop::{DesktopApp, DesktopWindow};
pub use app::KobaltApp;
pub use winit::event::{Event, WindowEvent};
pub use winit::event_loop::EventLoop;
