//! Widget tree and composition
//!
//! This module provides the core widget system inspired by Flutter and Compose.
//! All UI elements in Kobalt are widgets that implement the Widget trait.

use crate::types::{Rect, Size};

/// Base trait for all widgets in Kobalt
///
/// Widgets are the building blocks of Kobalt UIs. They describe what the UI should
/// look like given the current configuration and state.
///
/// Similar to Flutter's Widget and Compose's Composable functions.
pub trait Widget {
    /// Returns a unique identifier for this widget type
    fn widget_type(&self) -> &'static str;

    /// Calculates the desired size for this widget given constraints
    ///
    /// # Arguments
    /// * `constraints` - The size constraints from the parent
    ///
    /// # Returns
    /// The desired size for this widget
    fn layout(&self, constraints: Size) -> Size;

    /// Returns the layout bounds of this widget
    fn bounds(&self) -> Option<Rect> {
        None
    }
}

/// Layout constraints for widgets
#[derive(Debug, Clone, Copy)]
pub struct Constraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Constraints {
    /// Creates constraints with exact dimensions
    pub fn tight(width: f32, height: f32) -> Self {
        Self {
            min_width: width,
            max_width: width,
            min_height: height,
            max_height: height,
        }
    }

    /// Creates constraints with maximum dimensions only
    pub fn loose(max_width: f32, max_height: f32) -> Self {
        Self {
            min_width: 0.0,
            max_width,
            min_height: 0.0,
            max_height,
        }
    }

    /// Creates unbounded constraints
    pub fn unbounded() -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }

    /// Constrains a size to fit within these constraints
    pub fn constrain(&self, size: Size) -> Size {
        Size::new(
            size.width.max(self.min_width).min(self.max_width),
            size.height.max(self.min_height).min(self.max_height),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraints_tight() {
        let c = Constraints::tight(100.0, 200.0);
        assert_eq!(c.min_width, 100.0);
        assert_eq!(c.max_width, 100.0);
        assert_eq!(c.min_height, 200.0);
        assert_eq!(c.max_height, 200.0);
    }

    #[test]
    fn test_constraints_constrain() {
        let c = Constraints::loose(100.0, 200.0);

        let size1 = Size::new(50.0, 50.0);
        let constrained1 = c.constrain(size1);
        assert_eq!(constrained1, Size::new(50.0, 50.0));

        let size2 = Size::new(150.0, 250.0);
        let constrained2 = c.constrain(size2);
        assert_eq!(constrained2, Size::new(100.0, 200.0));
    }
}
