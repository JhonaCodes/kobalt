//! # Kobalt Macros
//!
//! Declarative macros for building UIs with Kobalt, inspired by Jetpack Compose and Flutter.
//!
//! # Example
//!
//! ```ignore
//! column! {
//!     text!("Hello, World!")
//!     button!("Click me", on_click = || { println!("Clicked!") })
//! }
//! ```

/// Macro for creating Text widgets with Flutter-style properties
///
/// # Examples
///
/// Simple text:
/// ```ignore
/// text!("Hello, World!")
/// ```
///
/// With properties:
/// ```ignore
/// text! {
///     content: "Hello",
///     font_size: 32.0,
///     color: Color::from_rgb8(255, 255, 255),
///     position: Point::new(100.0, 100.0)
/// }
/// ```
///
/// Short form:
/// ```ignore
/// text!("Hello", color: Color::RED, font_size: 24.0)
/// ```
#[macro_export]
macro_rules! text {
    // Simple form: just content
    ($content:expr) => {
        kobalt_widgets::Text::new($content)
    };

    // Full form with all properties
    (
        content: $content:expr
        $(, font_size: $font_size:expr)?
        $(, color: $color:expr)?
        $(, position: $position:expr)?
        $(,)?
    ) => {{
        let mut text = kobalt_widgets::Text::new($content);
        $(
            text = text.size($font_size);
        )?
        $(
            text = text.color($color);
        )?
        $(
            text = text.position($position);
        )?
        text
    }};

    // Short form: content with optional properties
    ($content:expr $(, $key:ident: $value:expr)* $(,)?) => {{
        let mut text = kobalt_widgets::Text::new($content);
        $(
            text = text.$key($value);
        )*
        text
    }};
}

/// Macro for creating Column layouts with full Flutter-style properties
///
/// # Examples
///
/// Simple column:
/// ```ignore
/// column! {
///     text!("Item 1"),
///     text!("Item 2")
/// }
/// ```
///
/// With properties:
/// ```ignore
/// column! {
///     main_axis_alignment: MainAxisAlignment::Center,
///     cross_axis_alignment: CrossAxisAlignment::Start,
///     padding: EdgeInsets::all(10.0),
///     children: [
///         text!("Item 1"),
///         text!("Item 2")
///     ]
/// }
/// ```
#[macro_export]
macro_rules! column {
    // Simple form: just children
    ($($child:expr),+ $(,)?) => {{
        let mut col = kobalt_widgets::Column::new();
        $(
            col = col.add($child);
        )*
        col
    }};

    // Full form with properties
    (
        $(main_axis_alignment: $main_align:expr,)?
        $(cross_axis_alignment: $cross_align:expr,)?
        $(padding: $padding:expr,)?
        children: [$($child:expr),* $(,)?]
    ) => {{
        let mut col = kobalt_widgets::Column::new();
        $(
            col = col.main_axis_alignment($main_align);
        )?
        $(
            col = col.cross_axis_alignment($cross_align);
        )?
        $(
            col = col.padding($padding);
        )?
        $(
            col = col.add($child);
        )*
        col
    }};
}

/// Macro for creating Row layouts (to be implemented)
#[macro_export]
macro_rules! row {
    ($($child:expr),* $(,)?) => {{
        vec![$($child),*]
    }};
}

/// Macro for creating TextStyle
///
/// # Example
///
/// ```ignore
/// text_style!(
///     font_family: "Arial",
///     color: Color::RED,
///     size: 24.0,
///     position: Point::new(10.0, 20.0)
/// )
/// ```
#[macro_export]
macro_rules! text_style {
    ($($key:ident: $value:expr),* $(,)?) => {{
        let mut style = kobalt_widgets::TextStyle::new();
        $(
            style = style.$key($value);
        )*
        style
    }};
}

/// Main app macro for declarative app creation
///
/// # Example
///
/// ```ignore
/// app! {
///     title: text!("My App"),
///     size: (800, 600),
///     background: Color::BLACK,
///     home: column! {
///         text!("Hello")
///     }
/// }
/// ```
#[macro_export]
macro_rules! app {
    (
        title: $title:expr
        $(, size: ($width:expr, $height:expr))?
        $(, background: $bg:expr)?
        , home: $home:expr
        $(,)?
    ) => {{
        let mut app = kobalt_runtime::KobaltApp::build()
            .title($title);

        $(
            app = app.size($width, $height);
        )?

        $(
            app = app.background($bg);
        )?

        app.home($home).run()
    }};
}
