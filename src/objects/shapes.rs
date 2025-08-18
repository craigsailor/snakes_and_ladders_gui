use ab_glyph::{Font, FontArc, Glyph, PxScale};
use image::DynamicImage;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

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

    // Function to draw a square with rounded corners
    /*
    pub fn draw_rounded(
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

#[derive(Debug, Clone)]
pub struct Png {
    pub id: i32,
    //pub png_path: &str,
    //pub png_path: String,
    pub img: DynamicImage,
    pub png_width: u32,
    pub png_height: u32,
    pub png_pixels: Vec<u32>, // Store pixels in ARGB format
}

impl Png {
    pub fn new(id: i32) -> Self {
        let png_bytes = include_bytes!("../player.png");
        let img = image::load_from_memory(png_bytes).unwrap();

        let rgba = img.to_rgba8();
        let png_width = rgba.width();
        let png_height = rgba.height();

        let png_pixels: Vec<u32> = rgba
            .chunks_exact(4)
            .map(|pixel| {
                let [r, g, b, a] = [pixel[0], pixel[1], pixel[2], pixel[3]];
                ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            })
            .collect();

        Png {
            id,
            img,
            png_width,
            png_height,
            png_pixels,
        }
    }

    pub fn draw_png_scaled_height(
        &self,
        buffer: &mut [u32],
        buffer_width: u32,
        x: i32,
        y: i32,
        target_height: u32,
        flip_horizontal: bool,
    ) {
        if target_height == 0 {
            return;
        }

        // Calculate scale factor based on target height
        let scale = target_height as f32 / self.png_height as f32;
        let scaled_width = (self.png_width as f32 * scale) as u32;
        let scaled_height = target_height;

        self.draw_png_scaled(
            buffer,
            buffer_width,
            x,
            y,
            scaled_width,
            scaled_height,
            flip_horizontal,
        );
    }
    fn draw_png_scaled(
        &self,
        buffer: &mut [u32],
        buffer_width: u32,
        x: i32,
        y: i32,
        target_width: u32,
        target_height: u32,
        flip_horizontal: bool,
    ) {
        let buffer_width = buffer_width as i32;
        let target_width = target_width as i32;
        let target_height = target_height as i32;
        let png_width = self.png_width as f32;
        let png_height = self.png_height as f32;

        // Calculate clipping bounds for the scaled image
        let start_x = 0.max(-x);
        let start_y = 0.max(-y);
        let end_x = target_width.min(buffer_width - x);
        let end_y = target_height.min((buffer.len() as i32) / buffer_width - y);

        // Draw scaled pixels
        for dst_y in start_y..end_y {
            for dst_x in start_x..end_x {
                // Map destination pixel back to source pixel
                let src_x = if flip_horizontal {
                    // Flip horizontally: rightmost dst_x maps to leftmost src pixel
                    ((target_width - 1 - dst_x) as f32 / target_width as f32) * png_width
                } else {
                    (dst_x as f32 / target_width as f32) * png_width
                };
                let src_y = (dst_y as f32 / target_height as f32) * png_height;

                // Use nearest neighbor sampling (you could implement bilinear for better quality)
                let src_x = src_x as u32;
                let src_y = src_y as u32;

                if src_x < self.png_width && src_y < self.png_height {
                    let src_index = (src_y * self.png_width + src_x) as usize;
                    let dst_index = ((y + dst_y) * buffer_width + (x + dst_x)) as usize;

                    if src_index < self.png_pixels.len() && dst_index < buffer.len() {
                        let src_pixel = self.png_pixels[src_index];
                        let alpha = (src_pixel >> 24) & 0xFF;

                        if alpha == 0 {
                            // Fully transparent - don't draw
                            continue;
                        } else if alpha == 255 {
                            // Fully opaque - direct copy
                            buffer[dst_index] = src_pixel;
                        } else {
                            // Semi-transparent - blend
                            let bg = buffer[dst_index];
                            let bg_r = (bg >> 16) & 0xFF;
                            let bg_g = (bg >> 8) & 0xFF;
                            let bg_b = bg & 0xFF;

                            let fg_r = (src_pixel >> 16) & 0xFF;
                            let fg_g = (src_pixel >> 8) & 0xFF;
                            let fg_b = src_pixel & 0xFF;

                            let inv_alpha = 255 - alpha;
                            let r = (fg_r * alpha + bg_r * inv_alpha) / 255;
                            let g = (fg_g * alpha + bg_g * inv_alpha) / 255;
                            let b = (fg_b * alpha + bg_b * inv_alpha) / 255;

                            buffer[dst_index] = 0xFF000000 | (r << 16) | (g << 8) | b;
                        }
                    }
                }
            }
        }
    }

    pub fn draw(
        &self,
        buffer: &mut [u32],
        buffer_width: u32,
        buffer_height: u32,
        x: i32,
        y: i32,
        flip_horizontal: bool,
    ) {
        let buffer_width: i32 = buffer_width as i32;
        let buffer_height: i32 = buffer_height as i32;
        let png_width: i32 = self.png_width as i32;
        let png_height: i32 = self.png_height as i32;

        // Calculate clipping bounds
        let start_x: i32 = 0.max(-x);
        let start_y: i32 = 0.max(-y);
        let end_x: i32 = png_width.min(buffer_width - x);
        let end_y: i32 = png_height.min(buffer_height - y);

        // Copy pixels row by row
        for src_y in start_y..end_y {
            let dst_y: i32 = y + src_y;
            if dst_y >= 0 && dst_y < buffer_height {
                // Handle transparency - only draw non-transparent pixels
                for dst_x_offset in 0..(end_x - start_x) {
                    let dst_x = x + start_x + dst_x_offset;

                    let src_x = if flip_horizontal {
                        // When flipping, map destination x to flipped source x
                        png_width - 1 - (start_x + dst_x_offset)
                    } else {
                        start_x + dst_x_offset
                    };

                    // Bounds checking
                    if src_x >= 0 && src_x < png_width && dst_x >= 0 && dst_x < buffer_width {
                        let src_idx = (src_y * png_width + src_x) as usize;
                        let dst_idx = (dst_y * buffer_width + dst_x) as usize;

                        if src_idx < self.png_pixels.len() && dst_idx < buffer.len() {
                            let src_pixel: u32 = self.png_pixels[src_idx];
                            let alpha: u32 = (src_pixel >> 24) & 0xFF;

                            if alpha == 0 {
                                // Fully transparent - don't draw anything
                                continue;
                            } else if alpha == 255 {
                                // Fully opaque - direct copy
                                buffer[dst_idx] = src_pixel;
                            } else {
                                // Semi-transparent - blend with existing pixel
                                let bg: u32 = buffer[dst_idx];
                                let bg_r: u32 = (bg >> 16) & 0xFF;
                                let bg_g: u32 = (bg >> 8) & 0xFF;
                                let bg_b: u32 = bg & 0xFF;
                                let fg_r: u32 = (src_pixel >> 16) & 0xFF;
                                let fg_g: u32 = (src_pixel >> 8) & 0xFF;
                                let fg_b: u32 = src_pixel & 0xFF;
                                let inv_alpha: u32 = 255 - alpha;
                                let r: u32 = (fg_r * alpha + bg_r * inv_alpha) / 255;
                                let g: u32 = (fg_g * alpha + bg_g * inv_alpha) / 255;
                                let b: u32 = (fg_b * alpha + bg_b * inv_alpha) / 255;
                                buffer[dst_idx] = 0xFF000000 | (r << 16) | (g << 8) | b;
                            }
                        }
                    }
                }
            }
        }
    }
}
