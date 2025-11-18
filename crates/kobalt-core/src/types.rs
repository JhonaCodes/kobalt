//! Common types used throughout Kobalt
//!
//! This module provides fundamental types for UI layout and rendering:
//! - `Size`: Represents width and height dimensions
//! - `Rect`: Represents a rectangle with position and size
//! - `Color`: Represents RGBA color values
//! - `Point`: Represents a 2D point

use std::ops::{Add, Sub, Mul, Div};

/// Represents a 2D point with x and y coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Creates a new point at (0, 0)
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Creates a new point with the given coordinates
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Calculates the distance to another point
    pub fn distance_to(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Represents width and height dimensions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    /// Creates a new size with zero dimensions
    pub const fn zero() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }

    /// Creates a new size with the given dimensions
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Creates a square size with equal width and height
    pub const fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    /// Returns true if either width or height is zero
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    /// Returns the area of this size
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Returns the aspect ratio (width / height)
    pub fn aspect_ratio(&self) -> f32 {
        if self.height != 0.0 {
            self.width / self.height
        } else {
            0.0
        }
    }

    /// Scales this size by a factor
    pub fn scale(&self, factor: f32) -> Self {
        Self {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

impl Mul<f32> for Size {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        self.scale(factor)
    }
}

impl Div<f32> for Size {
    type Output = Self;

    fn div(self, factor: f32) -> Self {
        Self {
            width: self.width / factor,
            height: self.height / factor,
        }
    }
}

/// Represents a rectangle with position and size
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Creates a new rectangle with zero position and size
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }

    /// Creates a new rectangle with the given position and size
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a rectangle from a point and size
    pub const fn from_point_size(point: Point, size: Size) -> Self {
        Self {
            x: point.x,
            y: point.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Returns the origin point (top-left corner)
    pub fn origin(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Returns the size of this rectangle
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Returns the center point of this rectangle
    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Returns the right edge x-coordinate
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Returns the bottom edge y-coordinate
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Returns true if this rectangle contains the given point
    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.right()
            && point.y >= self.y
            && point.y <= self.bottom()
    }

    /// Returns true if this rectangle intersects with another rectangle
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Returns the intersection of this rectangle with another, or None if they don't intersect
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }

        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        Some(Rect::new(x, y, right - x, bottom - y))
    }

    /// Returns true if either width or height is zero or negative
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    /// Insets this rectangle by the given amount on all sides
    pub fn inset(&self, amount: f32) -> Self {
        Self {
            x: self.x + amount,
            y: self.y + amount,
            width: (self.width - 2.0 * amount).max(0.0),
            height: (self.height - 2.0 * amount).max(0.0),
        }
    }
}

/// Represents an RGBA color with values from 0.0 to 1.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Creates a new color with the given RGBA values (0.0 to 1.0)
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new opaque color with the given RGB values (0.0 to 1.0)
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a color from 8-bit RGBA values (0-255)
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Creates a color from 8-bit RGB values (0-255) with full opacity
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }

    /// Creates a color from a hex string (e.g., "#FF5733" or "#FF5733FF")
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 && hex.len() != 8 {
            return Err(format!("Invalid hex color: must be 6 or 8 characters"));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| format!("Invalid hex color"))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| format!("Invalid hex color"))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| format!("Invalid hex color"))?;
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16)
                .map_err(|_| format!("Invalid hex color"))?
        } else {
            255
        };

        Ok(Self::from_rgba8(r, g, b, a))
    }

    /// Returns this color with a different alpha value
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    /// Converts to an array of f32 values [r, g, b, a]
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    // Common colors
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_operations() {
        let p1 = Point::new(10.0, 20.0);
        let p2 = Point::new(5.0, 10.0);

        let sum = p1 + p2;
        assert_eq!(sum, Point::new(15.0, 30.0));

        let diff = p1 - p2;
        assert_eq!(diff, Point::new(5.0, 10.0));
    }

    #[test]
    fn test_size_operations() {
        let size = Size::new(100.0, 200.0);

        assert_eq!(size.area(), 20000.0);
        assert_eq!(size.aspect_ratio(), 0.5);
        assert!(!size.is_empty());

        let scaled = size.scale(2.0);
        assert_eq!(scaled, Size::new(200.0, 400.0));
    }

    #[test]
    fn test_rect_operations() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);

        assert_eq!(rect.center(), Point::new(60.0, 120.0));
        assert_eq!(rect.right(), 110.0);
        assert_eq!(rect.bottom(), 220.0);

        assert!(rect.contains_point(Point::new(50.0, 100.0)));
        assert!(!rect.contains_point(Point::new(5.0, 100.0)));
    }

    #[test]
    fn test_rect_intersection() {
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);

        assert!(rect1.intersects(&rect2));

        let intersection = rect1.intersection(&rect2).unwrap();
        assert_eq!(intersection, Rect::new(50.0, 50.0, 50.0, 50.0));
    }

    #[test]
    fn test_color_from_rgb8() {
        let color = Color::from_rgb8(255, 128, 64);
        assert_eq!(color.r, 1.0);
        assert!((color.g - 0.502).abs() < 0.01);
        assert!((color.b - 0.251).abs() < 0.01);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF8040").unwrap();
        assert_eq!(color.r, 1.0);
        assert!((color.g - 0.502).abs() < 0.01);
        assert!((color.b - 0.251).abs() < 0.01);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(Color::WHITE, Color::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(Color::BLACK, Color::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::RED, Color::new(1.0, 0.0, 0.0, 1.0));
    }
}
