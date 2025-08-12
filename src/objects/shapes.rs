use ab_glyph::{Font, FontArc, Glyph, PxScale};
//use image::{DynamicImage, ImageReader, Rgba};
//use image::{ImageReader, Rgba};
//use std::sync::Arc;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};
//use winit::window::Window;
//use softbuffer::{Context, Surface};
//use std::num::NonZeroU32;
//use std::sync::Arc;
//use winit::application::ApplicationHandler;
//use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
//use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
//use winit::keyboard::{Key, NamedKey};
//use winit::window::{Window, WindowAttributes, WindowId};

use crate::drawable::Drawable;

// Drawable objects
#[derive(Debug, Clone)]
pub struct GameSquare {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: u32, // ARGB format
    pub label: String,
}

impl GameSquare {
    pub fn new(id: u32, x: f32, y: f32, size: f32, color: u32, label: String) -> Self {
        GameSquare {
            id,
            x,
            y,
            size,
            color,
            label,
        }
    }

    pub fn contains_point(&self, px: f64, py: f64) -> bool {
        let px = px as f32;
        let py = py as f32;

        px >= self.x && px < self.x + self.size && py >= self.y && py < self.y + self.size
    }

    pub fn center(&self) -> (f32, f32) {
        (
            self.x + (self.size as f32 * 0.5),
            self.y + (self.size as f32 * 0.5),
        )
    }

    // Function to draw a square with rounded corners
    /*
    fn draw_rounded_square(
        pixmap: &mut Pixmap,
        x: i32,
        y: i32,
        size: i32,
        corner_radius: i32,
        color: Color,
    ) {
        let size = size.max(2 * corner_radius); // Ensure size is at least twice the radius
        let half_size = size / 2;

        for py in y..(y + size) {
            for px in x..(x + size) {
                // Check if pixel is in one of the rounded corners
                let in_corner = if px < x + corner_radius && py < y + corner_radius {
                    // Top-left corner
                    let dx = x + corner_radius - px;
                    let dy = y + corner_radius - py;
                    dx * dx + dy * dy > corner_radius * corner_radius
                } else if px >= x + size - corner_radius && py < y + corner_radius {
                    // Top-right corner
                    let dx = px - (x + size - corner_radius - 1);
                    let dy = y + corner_radius - py;
                    dx * dx + dy * dy > corner_radius * corner_radius
                } else if px < x + corner_radius && py >= y + size - corner_radius {
                    // Bottom-left corner
                    let dx = x + corner_radius - px;
                    let dy = py - (y + size - corner_radius - 1);
                    dx * dx + dy * dy > corner_radius * corner_radius
                } else if px >= x + size - corner_radius && py >= y + size - corner_radius {
                    // Bottom-right corner
                    let dx = px - (x + size - corner_radius - 1);
                    let dy = py - (y + size - corner_radius - 1);
                    dx * dx + dy * dy > corner_radius * corner_radius
                } else {
                    false
                };

                // Draw pixel if it's inside the square and not in a rounded corner
                if !in_corner && px >= x && px < x + size && py >= y && py < y + size {
                    pixmap.set_pixel(px as u32, py as u32, color);
                }
            }
        }
    }
    */
}

impl Drawable for GameSquare {
    fn draw(&self, pixmap: &mut Pixmap) {
        // The font for the numbering of the squares
        let font_data = include_bytes!("../DejaVuSans-Bold.ttf");
        let font = FontArc::try_from_slice(font_data).expect("Failed to load font");

        // Simple square
        let points = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

        // Create a path for the square fill
        let mut pb = PathBuilder::new();
        pb.move_to(
            self.x + points[0].0 * self.size,
            self.y + points[0].1 * self.size,
        );
        for &(x, y) in points.iter().skip(1) {
            pb.line_to(self.x + x * self.size, self.y + y * self.size);
        }
        pb.close();
        let path = pb.finish().unwrap();

        // Alternatively, you can use Rect to create a square path
        //let path = PathBuilder::from_rect(
        //    Rect::from_ltrb(self.x, self.y, (self.x + self.size), (self.y + self.size)).unwrap(),
        //);

        // Extract RGBA components from u32 color (format: 0xRRGGBBAA)
        let r = ((self.color >> 24) & 0xFF) as f32 / 255.0;
        let g = ((self.color >> 16) & 0xFF) as f32 / 255.0;
        let b = ((self.color >> 8) & 0xFF) as f32 / 255.0;
        let a = (self.color & 0xFF) as f32 / 255.0;

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
        stroke_paint.set_color(Color::from_rgba8(0, 0, 0, 200));
        stroke_paint.anti_alias = true;

        // Draw the outline
        let stroke_width = self.size * 0.02; // Adjust stroke width relative to size
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

        // Render the square label using ab_glyph and tiny_skia
        let text = &self.label;
        let text_size = self.size * 0.25; // Adjust text size relative to square size

        // Calculate text position (center it in the square)
        let text_x = self.x + self.size * 0.1;
        let text_y = self.y + self.size * 0.9;

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
                current_x += font.h_advance_unscaled(glyph_id) * text_size
                    / font.units_per_em().unwrap() as f32;
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
}

#[derive(Debug, Clone)]
pub struct Arrow {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub thickness: f32,
    pub color: u32,
}

impl Arrow {
    pub fn new(
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: u32,
    ) -> Self {
        Arrow {
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color,
        }
    }

    pub fn set_start(&mut self, x: f32, y: f32) {
        self.start_x = x;
        self.start_y = y;
    }

    pub fn set_end(&mut self, x: f32, y: f32) {
        self.end_x = x;
        self.end_y = y;
    }
}

impl Drawable for Arrow {
    fn draw(&self, pixmap: &mut Pixmap) {
        //println!(
        //    "Drawing arrow from ({}, {}) to ({}, {}) with thickness {} and color {}",
        //    self.start_x, self.start_y, self.end_x, self.end_y, self.thickness, self.color
        //);

        // Create path for the arrow line
        let mut pb = PathBuilder::new();
        pb.move_to(self.start_x, self.start_y);
        pb.line_to(self.end_x, self.end_y);
        let path = pb.finish().unwrap();

        // Extract RGBA components from u32 color (format: 0xRRGGBBAA)
        let r = ((self.color >> 24) & 0xFF) as f32 / 255.0;
        let g = ((self.color >> 16) & 0xFF) as f32 / 255.0;
        let b = ((self.color >> 8) & 0xFF) as f32 / 255.0;
        let a = (self.color & 0xFF) as f32 / 255.0;

        // Set up paint for the arrow
        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba(r, g, b, a).unwrap());
        paint.anti_alias = true;

        // Draw the arrow line
        pixmap.stroke_path(
            &path,
            &paint,
            &Stroke {
                width: self.thickness,
                line_cap: tiny_skia::LineCap::Round,
                line_join: tiny_skia::LineJoin::Round,
                ..Default::default()
            },
            Transform::identity(),
            None,
        );

        // Calculate arrowhead
        let dx = self.end_x - self.start_x;
        let dy = self.end_y - self.start_y;
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 {
            return;
        }
        let ux = dx / len;
        let uy = dy / len;

        // Arrowhead parameters
        let arrow_size = self.thickness * 3.5; // Increased size for fatter arrowhead
        let angle: f32 = 0.4; // Increased angle for wider arrowhead

        // Points for arrowhead triangle
        let p1 = (self.end_x, self.end_y);
        let p2 = (
            self.end_x - arrow_size * (ux * angle.cos() + uy * angle.sin()),
            self.end_y - arrow_size * (uy * angle.cos() - ux * angle.sin()),
        );
        let p3 = (
            self.end_x - arrow_size * (ux * angle.cos() - uy * angle.sin()),
            self.end_y - arrow_size * (uy * angle.cos() + ux * angle.sin()),
        );

        // Create path for arrowhead
        let mut pb = PathBuilder::new();
        pb.move_to(p2.0, p2.1);
        pb.line_to(p1.0, p1.1);
        pb.line_to(p3.0, p3.1);

        let arrowhead = pb.finish().unwrap();

        // Draw filled arrowhead
        pixmap.stroke_path(
            &arrowhead,
            &paint,
            &Stroke {
                width: self.thickness,
                line_cap: tiny_skia::LineCap::Round,
                line_join: tiny_skia::LineJoin::Round,
                ..Default::default()
            },
            Transform::identity(),
            None,
        );
    }
}

/*
#[derive(Debug, Clone)]
pub struct Png {
    pub id: i32,
    //pub png_path: &str,
    pub png_path: String,
    pub x: i32,
    pub y: i32,
    pub window_width: u32,
    pub window_height: u32,
}

impl Png {
    pub fn draw(&self, buffer: &Arc<Window>) {
        println!(
            "Drawing png from ({}, {}) to ({}, {}) with thickness {} and color {}",
            self.x1, self.y1, self.x2, self.y2, self.thickness, self.color
        );

        /*
            buffer: &mut [u32],
            png_path: &str,
            x: i32,
            y: i32,
            window_width: u32,
            window_height: u32,
        ) -> Result<(), Box<dyn std::error::Error>> {
        */
        // Load the PNG image
        //let img = ImageReader::open(self.png_path)?.decode()?;
        let img = ImageReader::open(&self.png_path)?.decode()?;
        let rgba_img = img.to_rgba8();

        let (img_width, img_height) = rgba_img.dimensions();

        // Draw each pixel of the image
        for img_y in 0..img_height {
            for img_x in 0..img_width {
                let pixel = rgba_img.get_pixel(img_x, img_y);
                let Rgba([r, g, b, a]) = *pixel;

                // Skip transparent pixels
                if a == 0 {
                    continue;
                }

                // Calculate position in the window buffer
                let window_x = self.x + img_x as i32;
                let window_y = self.y + img_y as i32;

                // Check bounds
                if window_x >= 0
                    && window_y >= 0
                    && window_x < self.window_width as i32
                    && window_y < self.window_height as i32
                {
                    let buffer_index =
                        (window_y as u32 * self.window_width + window_x as u32) as usize;

                    if buffer_index < buffer.len() {
                        // Handle alpha blending (simple version)
                        if a == 255 {
                            // Fully opaque - just replace
                            buffer[buffer_index] = ((a as u32) << 24)
                                | ((r as u32) << 16)
                                | ((g as u32) << 8)
                                | (b as u32);
                        } else {
                            // Semi-transparent - blend with existing pixel
                            let existing = buffer[buffer_index];
                            let existing_r = ((existing >> 16) & 0xFF) as u8;
                            let existing_g = ((existing >> 8) & 0xFF) as u8;
                            let existing_b = (existing & 0xFF) as u8;

                            let alpha_f = a as f32 / 255.0;
                            let inv_alpha = 1.0 - alpha_f;

                            let new_r =
                                ((r as f32 * alpha_f) + (existing_r as f32 * inv_alpha)) as u8;
                            let new_g =
                                ((g as f32 * alpha_f) + (existing_g as f32 * inv_alpha)) as u8;
                            let new_b =
                                ((b as f32 * alpha_f) + (existing_b as f32 * inv_alpha)) as u8;

                            buffer[buffer_index] = (0xFF << 24)
                                | ((new_r as u32) << 16)
                                | ((new_g as u32) << 8)
                                | (new_b as u32);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
*/
