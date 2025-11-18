//! Text widget - displays text on screen

use kobalt_core::types::{Color, Point, Rect, Size};
use kobalt_core::widget::Widget;

/// Style configuration for Text widget
#[derive(Clone, Debug)]
pub struct TextStyle {
    pub color: Color,
    pub size: f32,
    pub position: Option<Point>,
    pub font_family: Option<String>,
}

impl TextStyle {
    /// Creates a new TextStyle with default values
    pub fn new() -> Self {
        Self {
            color: Color::WHITE,
            size: 16.0,
            position: None,
            font_family: None,
        }
    }

    /// Sets the text color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the font size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Sets the position
    pub fn position(mut self, position: Point) -> Self {
        self.position = Some(position);
        self
    }

    /// Sets the font family
    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = Some(font_family.into());
        self
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Text widget for displaying text on screen
///
/// # Example
///
/// ```
/// use kobalt_widgets::{Text, Color};
///
/// let text = Text::new("Hello, World!")
///     .color(Color::WHITE)
///     .size(24.0);
/// ```
#[derive(Clone)]
pub struct Text {
    /// The text content to display
    pub content: String,
    /// Text color
    pub color: Color,
    /// Font size in logical pixels
    pub font_size: f32,
    /// Position on screen
    pub position: Point,
    /// Calculated bounds after layout
    bounds: Option<Rect>,
}

impl Text {
    /// Creates a new Text widget with the given content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            color: Color::WHITE,
            font_size: 16.0,
            position: Point::zero(),
            bounds: None,
        }
    }

    /// Creates a new Text widget with the given content and style
    pub fn with_style(content: impl Into<String>, style: TextStyle) -> Self {
        Self {
            content: content.into(),
            color: style.color,
            font_size: style.size,
            position: style.position.unwrap_or(Point::zero()),
            bounds: None,
        }
    }

    /// Sets the text color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the font size
    pub fn size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Sets the position
    pub fn position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    /// Returns the text content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Estimates the text size based on font size and character count
    /// This is a simple approximation - real text rendering will be more accurate
    fn estimate_size(&self) -> Size {
        // Rough estimation: average character width is ~0.6 * font_size
        let char_width = self.font_size * 0.6;
        let width = self.content.len() as f32 * char_width;
        let height = self.font_size * 1.2; // Include line height

        Size::new(width, height)
    }
}

impl Widget for Text {
    fn widget_type(&self) -> &'static str {
        "Text"
    }

    fn layout(&self, constraints: Size) -> Size {
        let estimated = self.estimate_size();

        // Constrain to the available space
        Size::new(
            estimated.width.min(constraints.width),
            estimated.height.min(constraints.height),
        )
    }

    fn bounds(&self) -> Option<Rect> {
        self.bounds
    }
}

impl std::fmt::Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Text")
            .field("content", &self.content)
            .field("color", &self.color)
            .field("font_size", &self.font_size)
            .field("position", &self.position)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_creation() {
        let text = Text::new("Hello");
        assert_eq!(text.content(), "Hello");
        assert_eq!(text.font_size, 16.0);
        assert_eq!(text.color, Color::WHITE);
    }

    #[test]
    fn test_text_builder() {
        let text = Text::new("Test")
            .color(Color::RED)
            .size(24.0)
            .position(Point::new(10.0, 20.0));

        assert_eq!(text.content(), "Test");
        assert_eq!(text.color, Color::RED);
        assert_eq!(text.font_size, 24.0);
        assert_eq!(text.position, Point::new(10.0, 20.0));
    }

    #[test]
    fn test_text_widget_type() {
        let text = Text::new("Test");
        assert_eq!(text.widget_type(), "Text");
    }

    #[test]
    fn test_text_layout() {
        let text = Text::new("Hello").size(20.0);
        let constraints = Size::new(1000.0, 1000.0);

        let size = text.layout(constraints);

        // Should have some width and height based on text
        assert!(size.width > 0.0);
        assert!(size.height > 0.0);
    }
}
