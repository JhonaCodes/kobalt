//! Full example demonstrating Kobalt's macro-based UI syntax
//! This shows the Flutter/Compose-style declarative API

use kobalt::prelude::*;
use kobalt_macros::column;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using the app! macro with all features
    app! {
        title: text!("Kobalt - Full Macro UI"),
        size: (900, 700),
        background: Color::from_rgb8(20, 20, 30),
        home: column! {
            main_axis_alignment: MainAxisAlignment::Center,
            cross_axis_alignment: CrossAxisAlignment::Center,
            padding: EdgeInsets::all(20.0),
            children: [
                // Title
                text! {
                    content: "Welcome to Kobalt UI Framework",
                    font_size: 36.0,
                    color: Color::from_rgb8(255, 255, 255),
                    position: Point::new(100.0, 50.0)
                },

                // Subtitle with short syntax
                text!(
                    "Built with Rust + WGPU",
                    size: 24.0,
                    color: Color::from_rgb8(100, 200, 255)
                ),

                // Simple text
                text!("Declarative Macros for UI"),

                // Another styled text
                text! {
                    content: "Flutter-inspired syntax",
                    font_size: 20.0,
                    color: Color::from_rgb8(150, 255, 150)
                }
            ]
        }
    }
}
