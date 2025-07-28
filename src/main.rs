use ab_glyph::{Font, FontArc, Glyph, PxScale};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowAttributes, WindowId};

fn draw_einstein_tile(
    pixmap: &mut Pixmap,
    color: u32,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    number: u32,
    font: &FontArc,
) {
    // Simple square
    let points = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];

    // This actually draws the Einstein tile shape
    // 340.2 247
    /*
    let points = [
        (89.3 / 150.2, 146.9 / 123.0),
        (8.5 / 150.2, 194.0 / 123.0),
        (35.3 / 150.2, 240.5 / 123.0),
        (143.3 / 150.2, 240.5 / 123.0),
        (170.2 / 150.2, 193.6 / 123.0),
        (250.8 / 150.2, 240.7 / 123.0),
        (331.7 / 150.2, 193.9 / 123.0),
        (305.0 / 150.2, 146.9 / 123.0),
        (251.0 / 150.2, 146.9 / 123.0),
        (251.7 / 150.2, 53.4 / 123.0),
        (170.1 / 150.2, 6.5 / 123.0),
        (143.3 / 150.2, 53.4 / 123.0),
        (89.3 / 150.2, 53.4 / 123.0),
    ];
    */

    // Create a path for the tile fill
    let mut pb = PathBuilder::new();
    pb.move_to(
        offset_x + points[0].0 * scale,
        offset_y + points[0].1 * scale,
    );
    for &(x, y) in points.iter().skip(1) {
        pb.line_to(offset_x + x * scale, offset_y + y * scale);
    }
    pb.close();
    let path = pb.finish().unwrap();

    // Extract RGBA components from u32 color (format: 0xRRGGBBAA)
    let r = ((color >> 24) & 0xFF) as f32 / 255.0;
    let g = ((color >> 16) & 0xFF) as f32 / 255.0;
    let b = ((color >> 8) & 0xFF) as f32 / 255.0;
    let a = (color & 0xFF) as f32 / 255.0;

    // Set up paint for the tile fill
    let mut fill_paint = Paint::default();
    fill_paint.set_color(Color::from_rgba(r, g, b, a).unwrap());
    fill_paint.anti_alias = true;

    // Draw the filled tile
    pixmap.fill_path(
        &path,
        &fill_paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );

    // Set up paint for the black outline
    let mut stroke_paint = Paint::default();
    stroke_paint.set_color(Color::from_rgba8(0, 0, 0, 255));
    stroke_paint.anti_alias = true;

    // Draw the outline
    let stroke_width = scale * 0.02; // Adjust stroke width relative to scale
    pixmap.stroke_path(
        &path,
        &stroke_paint,
        &tiny_skia::Stroke {
            width: stroke_width,
            line_cap: tiny_skia::LineCap::Round,
            line_join: tiny_skia::LineJoin::Round,
            ..Default::default()
        },
        Transform::identity(),
        None,
    );

    // Render the tile number using ab_glyph and tiny_skia
    let text = number.to_string();
    let text_size = scale * 0.45; // Adjust text size relative to tile scale

    // Calculate text position (center it in the tile)
    let text_x = offset_x + scale * 0.6;
    let text_y = offset_y + scale * 1.0;

    // Create a path for the text
    let mut text_path = PathBuilder::new();
    let mut current_x = text_x;

    for c in text.chars() {
        let glyph_id = font.glyph_id(c);
        let glyph = Glyph {
            id: glyph_id,
            scale: PxScale::from(text_size),
            position: ab_glyph::point(current_x, text_y + 1.0),
        };

        if let Some(outlined) = font.outline_glyph(glyph) {
            // Convert the glyph outline to a tiny_skia path
            outlined.draw(|x, y, coverage| {
                if coverage > 0.5 {
                    let px = x as f32 + outlined.px_bounds().min.x;
                    let py = y as f32 + outlined.px_bounds().min.y;

                    // Create a small rectangle for each pixel of the glyph
                    text_path.push_rect(tiny_skia::Rect::from_xywh(px, py, 1.0, 1.0).unwrap());
                }
            });

            //current_x += font.h_advance(glyph_id) * text_size / font.units_per_em();
            current_x +=
                font.h_advance_unscaled(glyph_id) * text_size / font.units_per_em().unwrap() as f32;
        }
    }

    if let Some(text_path) = text_path.finish() {
        // Set up paint for the text (black color)
        let mut text_paint = Paint::default();
        text_paint.set_color(Color::from_rgba8(0, 0, 0, 255));
        text_paint.anti_alias = true;

        // Draw the text
        pixmap.fill_path(
            &text_path,
            &text_paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
}

struct App {
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    context: Option<Context<Arc<Window>>>,
    colors: Vec<u32>,
    font: FontArc,
}

impl App {
    fn new() -> Self {
        // Define a list of u32 colors (RGBA format)
        //let colors = vec![0xFF0066FF, 0xFF00FF00, 0xFFFF0000, 0xFFFFFF00];
        let colors = vec![
            0xFF0066FF, // Blue
            0xFF00AA00, // Green
            0xFFFF0000, // Red
            0xFFAA00AA, // Purple
            0xFF00AAAA, // Cyan
            0xFF0066FF, // Blue
            0xFF00AA00, // Green
            0xFFFF0000, // Red
            0xFFAA00AA, // Purple
            0xFF00AAAA, // Cyan
        ];

        // Load a font from a project-local file
        // Ensure DejaVuSans.ttf is in your project directory
        let font_data = include_bytes!("./DejaVuSans-Bold.ttf");
        let font = FontArc::try_from_slice(font_data).expect("Failed to load font");

        Self {
            window: None,
            surface: None,
            context: None,
            colors,
            font,
        }
    }

    fn draw(&mut self) {
        if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
            // Get the surface buffer and create a pixmap
            let mut buffer = surface.buffer_mut().unwrap();
            let size = window.inner_size();
            let (width, height) = (size.width, size.height);
            let mut pixmap = Pixmap::new(width, height).unwrap();

            // Clear the pixmap with a white background
            pixmap.fill(Color::from_rgba8(255, 255, 255, 255));

            // Draw a 10x10 grid of Einstein tiles
            let grid_size = 10;
            let tile_scale = (width as f32) / (grid_size as f32 * 2.5); // Adjust scale to fit tiles
            let spacing = (width as f32) / grid_size as f32;

            for row in 0..grid_size {
                for col in 0..grid_size {
                    let tile_number = 101 - (row * grid_size + col + 1); // Numbers 1 to 100
                    let color = self.colors[(tile_number - 1) % self.colors.len()];
                    let offset_x = col as f32 * spacing + spacing / 2.0;
                    let offset_y = row as f32 * spacing + spacing / 2.0;

                    draw_einstein_tile(
                        &mut pixmap,
                        color,
                        tile_scale,
                        offset_x / 1.1,
                        offset_y / 1.1,
                        tile_number as u32,
                        &self.font,
                    );
                }
            }

            // Copy pixmap to softbuffer
            for (i, pixel) in pixmap.pixels().iter().enumerate() {
                let r = pixel.red();
                let g = pixel.green();
                let b = pixel.blue();
                let a = pixel.alpha();
                buffer[i] =
                    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }

            // Present the buffer
            buffer.present().unwrap();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Einstein Tile Grid")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 800));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // Set up softbuffer context and surface
        let context = Context::new(window.clone()).unwrap();
        let window_size = window.inner_size();
        let mut surface = Surface::new(&context, window.clone()).unwrap();
        surface
            .resize(
                NonZeroU32::new(window_size.width).unwrap(),
                NonZeroU32::new(window_size.height).unwrap(),
            )
            .unwrap();

        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);

        // Initial draw
        self.draw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.draw();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(surface) = &mut self.surface {
                    // Resize the surface when the window is resized
                    surface
                        .resize(
                            NonZeroU32::new(new_size.width).unwrap(),
                            NonZeroU32::new(new_size.height).unwrap(),
                        )
                        .unwrap();
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
