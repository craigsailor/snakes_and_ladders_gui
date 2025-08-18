// File: src/game_data.rs
//use crate::game_board::GameBoard::SquareBoard;
use crate::objects::{GameSettings, User};
//use bincode;
use curv::arithmetic::Converter;
use curv::BigInt;
use rand::Rng;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
use std::fs;
use std::io::Write;

// Main game data container
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    // Drawable objects stored as trait objects
    //pub drawable_objects: Vec<Box<dyn Drawable>>,
    pub arrows: Vec<(u32, u32)>, // Pairs of arrows

    // Non-drawable objects
    pub users: Vec<User>,
    pub settings: GameSettings,
    pub colors: Vec<u32>,
    //pub board_type: GameBoard,
    pub grid_size: u32,
    //pub user_position: u32,
    pub new_game: bool,
}

impl GameState {
    pub fn new() -> Self {
        let colors: Vec<u32> = vec![
            0x0066FF6F, // Blue
            0x00AA006F, // Green
            0xFF00006F, // Red
            0xAA00AA6F, // Purple
            0x00AAAA6F, // Cyan
            0x0066FF6F, // Blue
            0x00AA006F, // Green
            0xFF00006F, // Red
            0xAA00AA6F, // Purple
            0x00AAAA6F, // Cyan
        ];

        let random_seed = Self::generate_random_seed();
        let arrows = Self::generate_arrow_pairs(random_seed.clone());

        Self {
            arrows: arrows, // Initialize with an empty vector
            users: vec![
                User::new(
                    0,                      // Default user ID
                    "Player 1".to_string(), // Default user name
                    1,                      // Starting position
                ),
                User::new(
                    1,                      // Default user ID
                    "Player 2".to_string(), // Default user name
                    1,                      // Starting position
                ),
            ],
            settings: GameSettings {
                game_id: Self::generate_random_seed(), // Randomly generated or assigned
                difficulty: "Normal".to_string(),
                sound_enabled: true,
                max_players: 4,
            },
            colors: colors,
            //board_type: GameBoard::SquareBoard,
            grid_size: 10, // Default board size
            //user_position: 1, // Default user position (starting square)
            new_game: true, // Default user position (starting square)
        }
    }

    pub fn move_player(&mut self, new_square: u32, player_id: i32) {
        // Placeholder for player movement logic
        println!("Moving user to square {}", new_square);
        self.users[player_id as usize].position = new_square;
    }

    pub fn advance_player(&mut self, count: u32, player_id: i32) {
        if self.new_game {
            self.users[player_id as usize].position = 0;
            self.new_game = false; // Set to false after the first move
        }

        if self.users[player_id as usize].position + count <= (self.grid_size * self.grid_size) {
            self.users[player_id as usize].position =
                self.users[player_id as usize].position.wrapping_add(count);
        }

        for arrow in &self.arrows {
            if &self.users[player_id as usize].position == &arrow.0 {
                println!(
                    "Landed at bottom of arrow on square {}. Moving to {}.",
                    arrow.0, arrow.1
                );
                self.users[player_id as usize].position = arrow.1;
            }
        }
    }

    pub fn spin(&mut self, player_id: i32) {
        self.advance_player(rand::rng().random_range(1..=5), player_id);
    }

    pub fn reset(&mut self) {
        // Reset the game state to initial values
        self.new_game = true; // Reset new game flag
        self.arrows.clear();
        let random_seed = Self::generate_random_seed();
        self.settings.game_id = random_seed.clone(); // Reset game ID to a new random seed
        let new_arrows = Self::generate_arrow_pairs(random_seed.clone());
        self.arrows = new_arrows;

        for user in &mut self.users {
            user.position = 1; // Reset each user's state
        }
        //self.user_position = 1; // Default user position (starting square)
    }

    // Temporary auxiliraty function to generate a seed
    fn generate_random_seed() -> BigInt {
        let mut rng = rand::rng();
        let mut digits = String::new();

        // First digit must be 1-9 to ensure 192 digits
        digits.push_str(&rng.random_range(1..=9).to_string());

        // Generate remaining 191 digits (0-9)
        for _ in 0..191 {
            digits.push_str(&rng.random_range(0..=9).to_string());
        }

        BigInt::from_str_radix(&digits, 10).expect("Invalid BigInt string")
    }

    // Save the game state to disk in JSON format
    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(filename)?;
        file.write_all(json_string.as_bytes())?;
        println!("Data saved to {}", filename);
        Ok(())
    }

    // Function to deserialize and load data from disk
    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_string = fs::read_to_string(filename)?;
        let state: Self = serde_json::from_str(&json_string)?;
        println!("Data loaded from {}", filename);
        Ok(state)
    }

    // Save GameState instance to a binary file
    /*
    fn save_to_file_binary(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = bincode::config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(self, config)?;
        fs::write(filename, encoded)?;
        println!("Binary data saved to {}", filename);
        Ok(())
    }

    // Load a GameState from a binary file
    fn load_from_file_binary(filename: &str) -> Result<GameState, Box<dyn std::error::Error>> {
        let bytes = fs::read(filename)?;
        let config = bincode::config::standard();
        let (state, _): (GameState, usize) = bincode::decode_from_slice(&bytes, config)?;
        println!("Binary data loaded from {}", filename);
        Ok(state)
    }
    */

    #[allow(dead_code)]
    //fn insert_with_auto_key(map: &mut HashMap<u32, String>, value: &str, counter: &mut u32) {
    /*
    fn insert_with_auto_key(
        map: &mut HashMap<u32, String>,
        //value: Box<dyn User>,
        value: String,
        counter: &mut u32,
    ) {
        let key = *counter;
        map.insert(key, value.to_string());
        *counter += 1;
    }

    // Add users
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.user_id, user);
    }

    // Draw all drawable objects
    pub fn draw_all(&self) {
        println!("Drawing all objects:");
        for drawable in &self.drawable_objects {
            drawable.draw();
        }
    }

    // Get user by ID
    pub fn get_user(&self, user_id: u32) -> Option<&User> {
        self.users.get(&user_id)
    }
    */

    // Update game settings
    pub fn update_settings(&mut self, settings: GameSettings) {
        self.settings = settings;
    }

    //pub fn generate_arrow_pairs(&mut self, seed: BigInt) {
    fn generate_arrow_pairs(seed: BigInt) -> Vec<(u32, u32)> {
        let mut arrows = Vec::new();
        let seed_str = seed.to_string();

        let num_pairs = ((&seed % BigInt::from(6)) + BigInt::from(5))
            .to_string()
            .parse::<u32>()
            .unwrap_or(5)
            .clamp(5, 10);

        //let mut pairs = Vec::new();

        for i in 1..num_pairs + 1 {
            let start = (i * 4) as usize;

            if start + 4 > seed_str.len() {
                break;
            }

            let chunk = &seed_str[start..start + 4];
            let first = chunk[0..2].parse::<u32>().unwrap_or(1).max(1).min(100);
            let second = chunk[2..4].parse::<u32>().unwrap_or(1).max(1).min(100);

            let first_decade = first / 10;
            let second_decade = second / 10;

            let final_second = if first_decade != second_decade {
                second
            } else if second >= first {
                (second + 10) % 100
            } else {
                (second.wrapping_sub(10)) % 100
            };

            arrows.push((first, final_second));
        }
        arrows
    }
}

// TODO: fix these tests for this object rather than the example Person object
/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_json_serialization() {
        let person = Person::new(
            "Test User".to_string(),
            25,
            "test@example.com".to_string(),
            false,
        );

        let filename = "test_person.json";

        // Save and load
        save_to_file(&person, filename).unwrap();
        let loaded = load_from_file(filename).unwrap();

        // Cleanup
        fs::remove_file(filename).ok();

        assert_eq!(person.name, loaded.name);
        assert_eq!(person.age, loaded.age);
        assert_eq!(person.email, loaded.email);
        assert_eq!(person.active, loaded.active);
    }

    #[test]
    fn test_binary_serialization() {
        let person = Person::new(
            "Binary Test".to_string(),
            35,
            "binary@example.com".to_string(),
            true,
        );

        let filename = "test_person.bin";

        // Save and load
        save_to_file_binary(&person, filename).unwrap();
        let loaded = load_from_file_binary(filename).unwrap();

        // Cleanup
        fs::remove_file(filename).ok();

        assert_eq!(person.name, loaded.name);
        assert_eq!(person.age, loaded.age);
        assert_eq!(person.email, loaded.email);
        assert_eq!(person.active, loaded.active);
    }
}
*/

// Example usage
/*
fn main() {
    let mut game_data = GameState::new();

    // Add some drawable objects
    let square = Square {
        x: 10.0,
        y: 20.0,
        size: 50.0,
        color: "red".to_string(),
    };

    let line = Line {
        x1: 0.0,
        y1: 0.0,
        x2: 100.0,
        y2: 100.0,
        thickness: 2.0,
        color: "blue".to_string(),
    };

    game_data.add_drawable(Box::new(square));
    game_data.add_drawable(Box::new(line));

    // Add some users
    let user1 = User {
        user_id: 1,
        name: "Alice".to_string(),
        score: 1500,
    };

    let user2 = User {
        user_id: 2,
        name: "Bob".to_string(),
        score: 1200,
    };

    game_data.add_user(user1);
    game_data.add_user(user2);

    // Draw all objects
    game_data.draw_all();

    // Access user data
    if let Some(user) = game_data.get_user(1) {
        println!("User {}: {} (Score: {})", user.user_id, user.name, user.score);
    }

    // Display current settings
    println!("Game Settings: Difficulty={}, Sound={}, Max Players={}",
             game_data.settings.difficulty,
             game_data.settings.sound_enabled,
             game_data.settings.max_players);
}
*/
