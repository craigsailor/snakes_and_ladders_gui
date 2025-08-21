use ab_glyph::{Font, FontArc, Glyph, PxScale};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

use crate::drawable::Drawable;
use crate::font_list;

// Drawable objects
#[derive(Debug, Clone)]
pub struct GameControls {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub bg_color: u32, // ARGB format
    pub title: String,
    pub button_height: f32,
    pub buttons: Vec<Button>,
}

impl GameControls {
    pub fn new() -> Self {
        GameControls {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            bg_color: 0xFFFFFFFF, // Default to white background
            title: String::from("Game Controls"),
            button_height: 0.0,
            buttons: Vec::new(),
        }
    }

    pub fn configure(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        bg_color: u32, // ARGB format
        title: String,
        button_height: f32,
        buttons: Vec<Button>,
    ) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
        self.bg_color = bg_color;
        self.title = title;
        self.button_height = button_height;
        self.buttons = buttons;
    }

    pub fn add_button(&mut self, button: Button) {
        self.buttons.push(button);
    }

    pub fn contains_point(&self, px: f64, py: f64) -> bool {
        let px = px as f32;
        let py = py as f32;

        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }

    pub fn onclick(&self, x: f64, y: f64) -> Option<String> {
        for button in &self.buttons {
            //if px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
            if button.contains_point(x, y) {
                return Some(button.label.clone());
            }
        }
        None
    }
}

impl GameControls {
    pub fn draw(&mut self, pixmap: &mut Pixmap) {
        // The font for the numbering of the squares
        //let font_data = include_bytes!("./DejaVuSans-Bold.ttf");
        let font_data = font_list!();
        let font = FontArc::try_from_slice(font_data[1]).expect("Failed to load font");

        let path = PathBuilder::from_rect(
            Rect::from_ltrb(self.x, self.y, self.x + self.width, self.y + self.height).unwrap(),
        );

        // Extract RGBA components from u32 color (format: 0xRRGGBBAA)
        let r = ((self.bg_color >> 24) & 0xFF) as f32 / 255.0;
        let g = ((self.bg_color >> 16) & 0xFF) as f32 / 255.0;
        let b = ((self.bg_color >> 8) & 0xFF) as f32 / 255.0;
        let a = (self.bg_color & 0xFF) as f32 / 255.0;

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
        let stroke_width = 2.5; // Adjust stroke width relative to size
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
        let text = &self.title;
        let text_size = self.button_height * 0.8; // Adjust text size relative to square size

        // Calculate text position (center it in the square)
        let text_x = self.x + text_size * 0.2;
        let text_y = self.y + text_size;

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

        // Draw each button
        //for button in &mut self.buttons {
        for indx in 0..self.buttons.len() {
            let y_offset = (self.button_height + (self.button_height * 0.2)) * indx as f32
                + (2.0 * self.button_height);

            self.buttons[indx].set_start(self.x + 10.0, self.y + y_offset);
            self.buttons[indx].set_end(
                self.x + self.width - 10.0,
                self.y + y_offset + self.button_height,
            );
            // Draw the button
            self.buttons[indx].draw(pixmap);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Button {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub label: String,
    pub color: u32,
}

impl Button {
    pub fn new(label: String, color: u32) -> Self {
        Button {
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
            label,
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

    pub fn contains_point(&self, px: f64, py: f64) -> bool {
        let px = px as f32;
        let py = py as f32;

        px >= self.start_x && px < self.end_x && py >= self.start_y && py < self.end_y
    }
}

impl Drawable for Button {
    fn draw(&self, pixmap: &mut Pixmap) {
        let path = PathBuilder::from_rect(
            Rect::from_ltrb(self.start_x, self.start_y, self.end_x, self.end_y).unwrap(),
        );
        let thickness = 4.0;

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
                width: thickness,
                line_cap: tiny_skia::LineCap::Round,
                line_join: tiny_skia::LineJoin::Round,
                ..Default::default()
            },
            Transform::identity(),
            None,
        );

        let font_data = font_list!();
        let font = FontArc::try_from_slice(font_data[1]).expect("Failed to load font");

        // Render the square label using ab_glyph and tiny_skia
        let text = &self.label;
        let text_size = (self.end_y - self.start_y) * 0.75; // Adjust text size relative to square size

        // Calculate text position (center it in the square)
        let text_x = self.start_x + ((self.end_x - self.start_x) / 2.0)
            - ((self.label.len() / 2) as f32 * (text_size));
        let text_y = self.start_y + (self.end_y - self.start_y) * 0.1 + text_size * 0.8;
        //let text_y = self.y + self.size * 0.9;

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
            // Draw the text
            pixmap.fill_path(
                &text_path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
    }
}
