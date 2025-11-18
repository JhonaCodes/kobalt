//! # Kobalt Core
//!
//! Core functionality for the Kobalt UI framework.
//!
//! This crate provides:
//! - Widget tree and composition
//! - Layout system (Column, Row, Flexbox)
//! - State management (LiveData, ValueNotifier, ChangeNotifier)
//! - MVVM infrastructure
//! - Common types (Size, Rect, Color)
//!
//! ## Architecture
//!
//! Kobalt Core is platform-agnostic and does not depend on WGPU or
//! any specific runtime. It can be tested with `cargo test` and is
//! portable to any platform.

// Re-export modules
pub mod types;
pub mod state;
pub mod widget;
pub mod layout;

// Re-export common layout types for convenience
pub use layout::{MainAxisAlignment, CrossAxisAlignment, EdgeInsets};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
