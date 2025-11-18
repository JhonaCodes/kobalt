//! Column layout widget - arranges children vertically

use kobalt_core::layout::{CrossAxisAlignment, EdgeInsets, MainAxisAlignment};
use kobalt_core::types::{Point, Rect, Size};
use kobalt_core::widget::Widget;

/// A layout widget that arranges its children vertically
pub struct Column {
    children: Vec<Box<dyn Widget>>,
    position: Point,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
    padding: EdgeInsets,
}

impl Column {
    /// Creates a new empty Column
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            position: Point::new(0.0, 0.0),
            main_axis_alignment: MainAxisAlignment::default(),
            cross_axis_alignment: CrossAxisAlignment::default(),
            padding: EdgeInsets::default(),
        }
    }

    /// Creates a new Column with the given children
    pub fn with_children(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            children,
            position: Point::new(0.0, 0.0),
            main_axis_alignment: MainAxisAlignment::default(),
            cross_axis_alignment: CrossAxisAlignment::default(),
            padding: EdgeInsets::default(),
        }
    }

    /// Adds a child widget to the column
    pub fn add<W: Widget + 'static>(mut self, widget: W) -> Self {
        self.children.push(Box::new(widget));
        self
    }

    /// Sets the position of the column
    pub fn position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    /// Sets the main axis alignment (vertical alignment)
    pub fn main_axis_alignment(mut self, alignment: MainAxisAlignment) -> Self {
        self.main_axis_alignment = alignment;
        self
    }

    /// Sets the cross axis alignment (horizontal alignment)
    pub fn cross_axis_alignment(mut self, alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = alignment;
        self
    }

    /// Sets the padding
    pub fn padding(mut self, padding: EdgeInsets) -> Self {
        self.padding = padding;
        self
    }

    /// Returns the children
    pub fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }

    /// Returns the main axis alignment
    pub fn get_main_axis_alignment(&self) -> MainAxisAlignment {
        self.main_axis_alignment
    }

    /// Returns the cross axis alignment
    pub fn get_cross_axis_alignment(&self) -> CrossAxisAlignment {
        self.cross_axis_alignment
    }

    /// Returns the padding
    pub fn get_padding(&self) -> EdgeInsets {
        self.padding
    }
}

impl Widget for Column {
    fn widget_type(&self) -> &'static str {
        "Column"
    }

    fn layout(&self, constraints: Size) -> Size {
        // Sum up the heights of all children
        let total_height: f32 = self.children
            .iter()
            .map(|child| child.layout(constraints).height)
            .sum();

        // Width is the maximum width of all children
        let max_width: f32 = self.children
            .iter()
            .map(|child| child.layout(constraints).width)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        Size::new(max_width, total_height)
    }

    fn bounds(&self) -> Option<Rect> {
        Some(Rect::new(
            self.position.x,
            self.position.y,
            0.0,  // Will be calculated during layout
            0.0,
        ))
    }
}
