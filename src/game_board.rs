use crate::drawable::Drawable;
//use crate::game_state;
use crate::objects::{Arrow, GameSquare};
//use std::collections::HashMap;
//use winit::window::Window;
//use tiny_skia::{Color, Pixmap};
//use std::sync::Arc;
use tiny_skia::Pixmap;

// Define the enum for different board types
#[derive(Debug, Clone)]
//pub enum GameBoard<'a> {
pub enum GameBoard {
    SquareBoard {
        squares: Vec<GameSquare>,
        //arrows: Vec<(u32, u32)>,
        arrows: Vec<Arrow>,
    }, // A square board with a given size (e.g., 8x8)
    EinsteinTileBoard {
        //tiles: Vec<EinsteinTile>,
        tiles: Vec<GameSquare>,
        arrows: Vec<(u32, u32)>,
    },
}

#[allow(unused_variables)]
#[allow(unreachable_patterns)]
impl GameBoard {
    pub fn init(
        &mut self,
        board_padding: i32,
        board_size: i32,
        grid_count: i32,
        spacing: i32,
        colors: Vec<u32>,
        added_arrows: &mut Vec<(u32, u32)>,
    ) {
        match self {
            GameBoard::SquareBoard { squares, arrows } => {
                if !squares.is_empty() {
                    return;
                }

                println!(
                    "Initializing SquareBoard with board size: {}, grid_size: {}, spacing: {}, arrows: {}",
                    board_size, grid_count, spacing, added_arrows.len()
                );

                let sq_size = (board_size / grid_count) - (spacing * 2);

                for col in 0..grid_count {
                    for row in 0..grid_count {
                        let sq_id = row + (col * grid_count) + 1;
                        let sq_y = board_padding + (grid_count - col) * (sq_size + spacing)
                            - (sq_size + spacing);

                        let sq_x = (sq_size * row)
                            + (spacing * row)
                            + spacing
                            + board_padding
                            + (col % 2)
                                * ((grid_count - 1 - row) * (sq_size + spacing)
                                    - (row * (sq_size + spacing)));

                        squares.push(GameSquare::new(
                            sq_id as u32,
                            sq_x as f32,
                            sq_y as f32,
                            sq_size as f32,
                            colors[(sq_id as usize + 1) % colors.len()],
                            sq_id.to_string(),
                        ));
                    }
                }

                for (arrow_x, arrow_y) in added_arrows {
                    let thickness = 7.0;
                    let color = 0x5F505FF0; // Red color for the arrow

                    let (start_x, start_y) =
                        squares[usize::try_from(*arrow_x - 1).unwrap()].center();
                    let (end_x, end_y) = squares[usize::try_from(*arrow_y - 1).unwrap()].center();

                    arrows.push(Arrow::new(start_x, start_y, end_x, end_y, thickness, color));
                }
            }
            GameBoard::EinsteinTileBoard { tiles, arrows } => {}
            _ => panic!("Unknown board type"),
        }
    }

    pub fn reset(&mut self) {
        match self {
            GameBoard::SquareBoard { squares, arrows } => {
                squares.clear();
                arrows.clear();
            }
            GameBoard::EinsteinTileBoard { tiles, arrows } => {
                tiles.clear();
                arrows.clear();
            }
        }
    }

    pub fn onclick(&self, x: f64, y: f64) -> Option<u32> {
        match self {
            GameBoard::SquareBoard { squares, arrows } => {
                for square in squares {
                    if square.contains_point(x, y) {
                        //println!("🎯 Clicked inside game square ID: {}", square.id);
                        return Some(square.id);
                    }
                }
                None
            }
            GameBoard::EinsteinTileBoard { tiles, arrows } => None,
        }
    }
}

// Implement methods for the GameBoard enum
impl Drawable for GameBoard {
    // Method to draw the board (console-based for simplicity)
    fn draw(&self, pixmap: &mut Pixmap) {
        match self {
            GameBoard::SquareBoard { squares, arrows } => {
                for square in squares {
                    square.draw(pixmap);
                }

                for arrow in arrows {
                    arrow.draw(pixmap);
                }
            }
            GameBoard::EinsteinTileBoard { tiles, arrows } => {
                println!(
                    "Drawing a square board of size {}x{}",
                    tiles.len(),
                    arrows.len()
                );
            }
        }
    }
}
