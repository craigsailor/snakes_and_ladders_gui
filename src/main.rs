pub mod drawable;
pub mod game_board;
pub mod game_controls;
pub mod game_state;
pub mod objects;

// Re-export commonly used items for convenience
pub use crate::game_board::GameBoard;
pub use drawable::Drawable;
pub use game_controls::{Button, GameControls};
pub use game_state::GameState;
pub use objects::{Arrow, GameSettings, GameSquare, Png, User};
use std::cmp;

//use crate::{Arrow, GameSettings, GameSquare, GameState, User};
//use ab_glyph::{Font, FontArc, Glyph, PxScale};
///use ab_glyph::FontArc;
use softbuffer::{Context, Surface};
//use std::fs;
use std::num::NonZeroU32;
use std::sync::Arc;
//use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};
use tiny_skia::{Color, Pixmap};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowAttributes, WindowId};

struct App {
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    context: Option<Context<Arc<Window>>>,
    cursor_position: (f64, f64),
    game_state: GameState,
    game_board: GameBoard,
    game_controls: GameControls,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            surface: None,
            context: None,
            cursor_position: (0.0, 0.0),
            game_state: GameState::new(),
            game_board: GameBoard::SquareBoard {
                squares: vec![],
                arrows: vec![],
            },
            game_controls: GameControls::new(),
        }
    }

    #[allow(unused_variables)]
    fn get_sq_center(board: &GameBoard, sq_number: usize) -> Option<(f32, f32)> {
        if let GameBoard::SquareBoard { squares, arrows } = board {
            if let Some(square) = squares.get(sq_number) {
                let center = square.center();
                return Some(center);
            }
        }
        None
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
            let board_size = cmp::min(width, height) as f32 * 0.9; // 80% of the smaller dimension
            let board_padding = cmp::min(width, height) as f32 * 0.1; // 5%
                                                                      //let grid_count = 10.0;
            let grid_count = self.game_state.grid_size as f32;
            let spacing = (board_size / grid_count) as f32 * 0.1;
            let sq_size = (board_size / grid_count) - (spacing * 2.0);

            // Set grid height to 80% of window height
            //let grid_height = height as f32 * 0.8;
            //let square_scale = grid_height / (grid_size as f32 * 2.5); // Scale squares to fit
            //let spacing = grid_height / grid_size as f32; // Spacing based on grid height

            // Calculate offsets to center the grid
            //let grid_width = spacing * grid_size as f32;
            //let offset_x_start = (width as f32 - grid_width) / 2.0;
            //let offset_y_start = (height as f32 - grid_height) / 2.0;

            self.game_board.init(
                //let game_board =  self.game_board.new(
                board_padding as i32,
                board_size as i32,
                grid_count as i32,
                spacing as i32,
                self.game_state.colors.clone(),
                self.game_state.arrows.as_mut(),
            );

            let button_list = vec![
                Button::new("Spin".to_string(), 0x00CC00FF),
                Button::new("Mine".to_string(), 0xCC0000FF),
                Button::new("Reset".to_string(), 0x0000CCFF),
            ];

            self.game_controls.configure(
                //cmp::max(width, height) as f32 + board_padding,
                board_size + board_padding,
                board_padding,
                width as f32 - board_size - (board_padding * 2.0),
                height as f32 - (board_padding * 2.0),
                0xCCCCCC0F,
                "The Game".to_string(),
                button_list,
            );

            // Draw the game board
            self.game_board.draw(&mut pixmap);
            self.game_controls.draw(&mut pixmap);

            // Copy pixmap to softbuffer
            for (i, pixel) in pixmap.pixels().iter().enumerate() {
                let r = pixel.red();
                let g = pixel.green();
                let b = pixel.blue();
                let a = pixel.alpha();
                buffer[i] =
                    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }

            //let player = Png::new();
            let player_position = Self::get_sq_center(
                &self.game_board,
                (self.game_state.user_position - 1) as usize,
            )
            .unwrap_or((0.0, 0.0));

            let player = Png::new(0);

            player.draw(
                &mut buffer,
                width as u32,
                height as u32,
                (player_position.0 - sq_size / 2.0) as i32,
                (player_position.1 - sq_size / 2.0) as i32,
                !get_range_flag(self.game_state.user_position, grid_count as u32),
            );

            // Present the buffer
            buffer.present().unwrap();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("The dynamic of life game")
            .with_inner_size(winit::dpi::LogicalSize::new(1024, 800));

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

            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = (position.x, position.y);
                //println!("Mouse moved to: x={:.2}, y={:.2}", position.x, position.y);
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                match self
                    .game_board
                    .onclick(self.cursor_position.0, self.cursor_position.1)
                {
                    Some(square_number) => {
                        println!("🎯 Clicked inside game square ID: {}", square_number);
                        //self.game_state.move_player(square_number);
                        //self.game_state.advance_player(rand::rng().random_range(1..=5));
                        //if let Some(window) = &self.window {
                        //    window.request_redraw();
                        // }
                    }
                    None => {}
                }

                match &self
                    .game_controls
                    .onclick(self.cursor_position.0, self.cursor_position.1)
                {
                    Some(button_name) => {
                        match button_name.as_str() {
                            "Spin" => {
                                self.game_state.spin();
                                for arrow in &self.game_state.arrows.clone() {
                                    if &self.game_state.user_position == &arrow.0 {
                                        println!(
                                            "Landed at bottom of arrow on square {}. Moving to {}.",
                                            arrow.0, arrow.1
                                        );
                                        self.game_state.move_player(arrow.1);
                                    }
                                }
                            }
                            "Mine" => {
                                //self.game_state.mine();
                            }
                            "Reset" => {
                                self.game_state.reset();
                                self.game_board.reset();
                            }
                            _ => {}
                        }
                        //println!("🎯 Clicked inside button: {}", button_name);

                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                    None => {}
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

// TODO: Make this generalized for any range not just a grid of 10
/*
fn get_range_flag(n: u32) -> bool {
    if n < 1 {
        return false;
    } // Handle invalid input (optional)
    let mod_value = (n - 1) % 20;
    mod_value < 10
}
*/

fn get_range_flag(n: u32, range_size: u32) -> bool {
    if n < 1 || range_size < 1 {
        return false;
    } // Handle invalid input
    let mod_value = (n - 1) % (2 * range_size);
    mod_value < range_size
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
