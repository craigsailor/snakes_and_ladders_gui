use std::collections::HashMap;

// Non-drawable objects
#[derive(Debug, Clone)]
pub struct User {
    pub user_id: u32,
    pub name: String,
    pub score: i32,
}

#[derive(Debug, Clone)]
pub struct GameSettings {
    pub difficulty: String,
    pub sound_enabled: bool,
    pub max_players: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            difficulty: "Normal".to_string(),
            sound_enabled: true,
            max_players: 4,
        }
    }
}
