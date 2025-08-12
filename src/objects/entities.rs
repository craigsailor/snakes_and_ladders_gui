use curv::BigInt;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;

// Non-drawable objects
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: u32,
    pub name: String,
    pub score: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameSettings {
    pub game_id: BigInt,
    pub difficulty: String,
    pub sound_enabled: bool,
    pub max_players: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            game_id: BigInt::from(0),
            difficulty: "Normal".to_string(),
            sound_enabled: true,
            max_players: 4,
        }
    }
}
