// File: src/game_data.rs
use crate::drawable::Drawable;
use crate::objects::{GameSettings, User};
use std::collections::HashMap;

// Main game data container
pub struct GameData {
    // Drawable objects stored as trait objects
    pub drawable_objects: Vec<Box<dyn Drawable>>,

    // Non-drawable objects
    pub users: HashMap<u32, User>,
    pub settings: GameSettings,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            drawable_objects: Vec::new(),
            users: HashMap::new(),
            settings: GameSettings {
                difficulty: "Normal".to_string(),
                sound_enabled: true,
                max_players: 4,
            },
        }
    }

    // Add drawable objects
    pub fn add_drawable(&mut self, drawable: Box<dyn Drawable>) {
        self.drawable_objects.push(drawable);
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

    // Update game settings
    pub fn update_settings(&mut self, settings: GameSettings) {
        self.settings = settings;
    }
}

// Example usage
/*
fn main() {
    let mut game_data = GameData::new();

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
