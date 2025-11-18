//! High-level Kobalt application API
//!
//! Provides a declarative, Flutter-style API for building Kobalt apps

use crate::desktop::DesktopApp;
use kobalt_core::types::Color;
use kobalt_core::widget::Widget;
use kobalt_render::RealTextRenderer;
use winit::event::{Event, WindowEvent};

/// Kobalt application builder
///
/// Provides a declarative API for building apps, similar to Flutter's MaterialApp
///
/// # Example
///
/// ```no_run
/// use kobalt_runtime::KobaltApp;
/// use kobalt_widgets::Text;
///
/// KobaltApp::build()
///     .title("My App")
///     .size(800, 600)
///     .home(Text::new("Hello, Kobalt!"))
///     .run()
///     .unwrap();
/// ```
pub struct KobaltApp {
    title: Box<dyn Widget>,
    width: u32,
    height: u32,
    background_color: Color,
    home: Option<Box<dyn Widget>>,
}

impl KobaltApp {
    /// Creates a new Kobalt application builder
    pub fn build() -> Self {
        // Default title
        let default_title = kobalt_widgets::Text::new("Kobalt App");

        Self {
            title: Box::new(default_title),
            width: 800,
            height: 600,
            background_color: Color::from_rgb8(20, 20, 30),
            home: None,
        }
    }

    /// Sets the window title (as a Widget)
    pub fn title<W: Widget + 'static>(mut self, title: W) -> Self {
        self.title = Box::new(title);
        self
    }

    /// Sets the window size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the background color
    pub fn background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Sets the home widget (main content)
    pub fn home<W: Widget + 'static>(mut self, widget: W) -> Self {
        self.home = Some(Box::new(widget));
        self
    }

    /// Runs the application
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let title_widget = self.title;
        let width = self.width;
        let height = self.height;
        let bg_color = self.background_color;
        let home_widget = self.home.expect("Home widget not set. Call .home() before .run()");

        // Extract title text for window (temporary hack)
        let window_title = if title_widget.widget_type() == "Text" {
            let text_ptr = &*title_widget as *const dyn Widget as *const kobalt_widgets::Text;
            let text = unsafe { &*text_ptr };
            text.content().to_string()
        } else {
            "Kobalt App".to_string()
        };

        // Text renderer state
        let mut text_renderer: Option<RealTextRenderer> = None;

        let app = DesktopApp::new(&window_title, width, height, move |window, event| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Initialize text renderer on first frame
                    if text_renderer.is_none() {
                        let renderer = window.renderer();
                        text_renderer = Some(RealTextRenderer::new(
                            renderer.device(),
                            renderer.config(),
                        ));
                    }

                    let screen_size = window.renderer().size();

                    // Collect all widgets to render
                    let mut widgets_to_render = Vec::new();
                    collect_widgets(&*home_widget, &mut widgets_to_render);

                    // Prepare text for each widget
                    if let Some(ref mut text_renderer) = text_renderer {
                        for widget in &widgets_to_render {
                            if widget.widget_type() == "Text" {
                                add_text_widget(
                                    *widget,
                                    text_renderer,
                                    window.renderer().device(),
                                    window.renderer().queue(),
                                    screen_size.width,
                                    screen_size.height,
                                );
                            }
                        }
                    }

                    // Render frame
                    match window.renderer().begin_frame() {
                        Ok(frame) => {
                            let mut encoder = frame.create_encoder();
                            {
                                let mut render_pass = frame.begin_render_pass(&mut encoder, bg_color);

                                // Render text
                                if let Some(ref text_renderer) = text_renderer {
                                    text_renderer.render(&mut render_pass);
                                }
                            }
                            frame.present(encoder);
                        }
                        Err(kobalt_render::SurfaceError::Lost) => {
                            window.renderer_mut().resize(screen_size);
                        }
                        Err(kobalt_render::SurfaceError::OutOfMemory) => {
                            eprintln!("Out of memory!");
                        }
                        Err(e) => {
                            eprintln!("Surface error: {:?}", e);
                        }
                    }
                }
                _ => {}
            }
        });

        app.run()
    }
}

/// Recursively collect all widgets from the widget tree
fn collect_widgets<'a>(widget: &'a dyn Widget, widgets: &mut Vec<&'a dyn Widget>) {
    match widget.widget_type() {
        "Text" => {
            widgets.push(widget);
        }
        "Column" => {
            // Downcast to Column to access children
            let column_ptr = widget as *const dyn Widget as *const kobalt_widgets::Column;
            let column = unsafe { &*column_ptr };

            for child in column.children() {
                collect_widgets(&**child, widgets);
            }
        }
        _ => {
            eprintln!("Warning: Unknown widget type: {}", widget.widget_type());
        }
    }
}

/// Add a text widget to the text renderer
fn add_text_widget(
    widget: &dyn Widget,
    text_renderer: &mut RealTextRenderer,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    screen_width: f32,
    screen_height: f32,
) {
    use kobalt_widgets::Text;

    let text_ptr = widget as *const dyn Widget as *const Text;
    let text = unsafe { &*text_ptr };

    text_renderer.prepare_text(
        device,
        queue,
        text.content(),
        text.position,
        text.font_size,
        text.color,
        screen_width,
        screen_height,
    );
}
