use ab_glyph::{Font, FontArc, Glyph, PxScale};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};
//use winit::application::ApplicationHandler;
//use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
//use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
//use winit::keyboard::{Key, NamedKey};
//use winit::window::{Window, WindowAttributes, WindowId};

use crate::drawable::Drawable;

use std::collections::HashMap;

// Drawable objects
#[derive(Debug, Clone)]
struct GameSquare {
    id: u32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: u32, // ARGB format
}

impl GameSquare {
    fn new(id: u32, x: i32, y: i32, width: u32, height: u32, color: u32) -> Self {
        Self {
            id,
            x,
            y,
            width,
            height,
            color,
        }
    }

    fn contains_point(&self, px: f64, py: f64) -> bool {
        let px = px as i32;
        let py = py as i32;
        px >= self.x
            && px < self.x + self.width as i32
            && py >= self.y
            && py < self.y + self.height as i32
    }

    fn draw(&self, pixmap: &mut Pixmap) {
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
                            text_path
                                .push_rect(tiny_skia::Rect::from_xywh(px, py, 1.0, 1.0).unwrap());
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
        println!(
            "Drawing square at ({}, {}) with size {} and color {}",
            self.x, self.y, self.size, self.color
        );
    }
}

#[derive(Debug, Clone)]
pub struct Square {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: String,
}

impl Drawable for Square {
    fn draw(&self) {
        println!(
            "Drawing square at ({}, {}) with size {} and color {}",
            self.x, self.y, self.size, self.color
        );
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub thickness: f32,
    pub color: String,
}

impl Drawable for Line {
    fn draw(&self) {
        println!(
            "Drawing line from ({}, {}) to ({}, {}) with thickness {} and color {}",
            self.x1, self.y1, self.x2, self.y2, self.thickness, self.color
        );
    }
}

impl Drawable for Arrow {
    fn draw(
        &self,
        pixmap: &mut Pixmap, //start: (usize, usize),
                             //end: (usize, usize),
                             //scale: f32,
                             //offset_x_start: f32,
                             //offset_y_start: f32,
                             //spacing: f32,
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
}
