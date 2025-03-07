use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub max_players: usize,
    pub world_seed: u64,
    pub view_distance: u8,
    pub game_mode: GameMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:25565".to_string(),
            max_players: 20,
            world_seed: 12345,
            view_distance: 10,
            game_mode: GameMode::Survival,
        }
    }
}

impl ServerConfig {
    pub fn load_or_default() -> Self {
        match File::open("server_config.json") {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    if let Ok(config) = serde_json::from_str(&contents) {
                        return config;
                    }
                }
                let config = Self::default();
                let _ = Self::save(&config);
                config
            }
            Err(_) => {
                let config = Self::default();
                let _ = Self::save(&config);
                config
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create("server_config.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
} 