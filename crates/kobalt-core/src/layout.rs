//! Layout primitives and alignment enums

/// Main axis alignment (vertical for Column, horizontal for Row)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainAxisAlignment {
    /// Place children at the start of the axis
    Start,
    /// Place children at the end of the axis
    End,
    /// Place children at the center of the axis
    Center,
    /// Place children with equal spacing between them
    SpaceBetween,
    /// Place children with equal spacing around them
    SpaceAround,
    /// Place children with equal spacing evenly distributed
    SpaceEvenly,
}

impl Default for MainAxisAlignment {
    fn default() -> Self {
        Self::Start
    }
}

/// Cross axis alignment (horizontal for Column, vertical for Row)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrossAxisAlignment {
    /// Align children to the start of the cross axis
    Start,
    /// Align children to the end of the cross axis
    End,
    /// Align children to the center of the cross axis
    Center,
    /// Stretch children to fill the cross axis
    Stretch,
}

impl Default for CrossAxisAlignment {
    fn default() -> Self {
        Self::Start
    }
}

/// Edge insets for padding/margin
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeInsets {
    /// Creates edge insets with all sides equal
    pub fn all(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Creates edge insets with symmetric values
    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Creates edge insets with individual values
    pub fn only(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Creates zero edge insets
    pub fn zero() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }

    /// Returns the total horizontal insets
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    /// Returns the total vertical insets
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl Default for EdgeInsets {
    fn default() -> Self {
        Self::zero()
    }
}
