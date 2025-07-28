use ab_glyph::{Font, FontArc, Glyph, PxScale};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowAttributes, WindowId};

fn draw_square(
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

    // Create a path for the square fill
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

    // Set up paint for the square fill
    let mut fill_paint = Paint::default();
    fill_paint.set_color(Color::from_rgba(r, g, b, a).unwrap());
    fill_paint.anti_alias = true;

    // Draw the filled square
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

    // Render the square number using ab_glyph and tiny_skia
    let text = number.to_string();
    let text_size = scale * 0.45; // Adjust text size relative to square scale

    // Calculate text position (center it in the square)
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

fn draw_arrow(
    pixmap: &mut Pixmap,
    start: (usize, usize),
    end: (usize, usize),
    scale: f32,
    offset_x_start: f32,
    offset_y_start: f32,
    spacing: f32,
) {
    // Calculate center of start and end squares
    let start_x = offset_x_start + start.1 as f32 * spacing + scale;
    let start_y = offset_y_start + start.0 as f32 * spacing + scale;
    let end_x = offset_x_start + end.1 as f32 * spacing + scale;
    let end_y = offset_y_start + end.0 as f32 * spacing + scale;

    // Create path for the arrow line
    let mut pb = PathBuilder::new();
    pb.move_to(start_x, start_y);
    pb.line_to(end_x, end_y);
    let path = pb.finish().unwrap();

    // Set up paint for the arrow
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(0, 0, 0, 255));
    paint.anti_alias = true;

    // Draw the arrow line
    pixmap.stroke_path(
        &path,
        &paint,
        &Stroke {
            width: scale * 0.05,
            line_cap: tiny_skia::LineCap::Round,
            line_join: tiny_skia::LineJoin::Round,
            ..Default::default()
        },
        Transform::identity(),
        None,
    );

    // Calculate arrowhead
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 {
        return;
    }
    let ux = dx / len;
    let uy = dy / len;

    // Arrowhead parameters
    let arrow_size = scale * 0.5; // Increased size for fatter arrowhead
    let angle: f32 = 0.4; // Increased angle for wider arrowhead

    // Points for arrowhead triangle
    let p1 = (end_x, end_y);
    let p2 = (
        end_x - arrow_size * (ux * angle.cos() + uy * angle.sin()),
        end_y - arrow_size * (uy * angle.cos() - ux * angle.sin()),
    );
    let p3 = (
        end_x - arrow_size * (ux * angle.cos() - uy * angle.sin()),
        end_y - arrow_size * (uy * angle.cos() + ux * angle.sin()),
    );

    // Create path for arrowhead
    let mut pb = PathBuilder::new();
    pb.move_to(p1.0, p1.1);
    pb.line_to(p2.0, p2.1);
    pb.line_to(p3.0, p3.1);
    pb.close();
    let arrowhead = pb.finish().unwrap();

    // Draw filled arrowhead
    pixmap.fill_path(
        &arrowhead,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
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

            // Draw a 10x10 grid of squares
            let grid_size = 10;
            // Set grid height to 80% of window height
            let grid_height = height as f32 * 0.8;
            let square_scale = grid_height / (grid_size as f32 * 2.5); // Scale squares to fit
            let spacing = grid_height / grid_size as f32; // Spacing based on grid height

            // Calculate offsets to center the grid
            let grid_width = spacing * grid_size as f32;
            let offset_x_start = (width as f32 - grid_width) / 2.0;
            let offset_y_start = (height as f32 - grid_height) / 2.0;

            for row in 0..grid_size {
                for col in 0..grid_size {
                    let square_number = 101 - (row * grid_size + col + 1); // Numbers 1 to 100
                    let color = self.colors[(square_number - 1) % self.colors.len()];
                    let offset_x = offset_x_start + col as f32 * spacing;
                    let offset_y = offset_y_start + row as f32 * spacing;

                    draw_square(
                        &mut pixmap,
                        color,
                        square_scale,
                        offset_x,
                        offset_y,
                        square_number as u32,
                        &self.font,
                    );
                }
            }

            // Example: Draw an arrow from (1,1) to (3,3)
            draw_arrow(
                &mut pixmap,
                (1, 1),
                (3, 3),
                square_scale,
                offset_x_start,
                offset_y_start,
                spacing,
            );

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
            .with_title("The dynamic of life game")
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
