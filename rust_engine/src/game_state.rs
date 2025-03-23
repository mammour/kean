use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::character::Character;
use crate::npc::NPC;
use crate::entity_type::EntityType;
use crate::tag::TagCollection;

/// Represents the current state of the game world
#[derive(Serialize, Deserialize)]
pub struct GameState {
    /// Unique ID for this game instance
    pub id: String,
    /// Current game version
    pub version: String,
    /// Current tick/frame count
    pub tick: u64,
    /// Timestamp of the last update
    pub last_updated: u64,
    /// The main player character
    pub player: Character,
    /// All NPCs in the game world
    pub npcs: Vec<NPC>,
    /// Collection of all tags in the game
    pub tag_collection: TagCollection,
    /// All entity types defined in the game
    pub entity_types: HashMap<String, EntityType>,
    /// Current game time (may differ from real time)
    pub game_time: f32,
    /// Whether the game is currently running
    #[serde(skip)]
    pub running: bool,
    /// Custom game properties that can be set by the game logic
    pub properties: HashMap<String, String>,
}

impl GameState {
    /// Create a new game state with default values
    pub fn new() -> Self {
        println!("Initializing game state...");
        let state = GameState {
            id: format!("game_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()),
            version: env!("CARGO_PKG_VERSION").to_string(),
            tick: 0,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            player: Character::new(),
            npcs: Vec::new(),
            tag_collection: TagCollection::new(),
            entity_types: HashMap::new(),
            game_time: 0.0,
            running: true,
            properties: HashMap::new(),
        };
        
        println!("Game state initialized");
        state
    }
    
    /// Update the game state for the next frame
    pub fn update(&mut self, delta_time: f32) {
        // Update tick counter and game time
        self.tick += 1;
        self.game_time += delta_time;
        
        // Update last updated timestamp
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Print game state occasionally
        if self.tick % 10 == 0 {
            println!("Tick {}: Player at {}, {} NPCs", 
                self.tick, self.player.position, self.npcs.len());
        }
    }
    
    /// Process a command from the user or external tool
    pub fn process_command(&mut self, command: &str) -> String {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return "No command provided".to_string();
        }
        
        match parts[0].to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("Shutting down game service...");
                self.running = false;
                "Shutting down...".to_string()
            },
            "move" => {
                if parts.len() >= 3 {
                    if let (Ok(x), Ok(y)) = (parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                        self.player.position.set(0, x);
                        self.player.position.set(1, y);
                        format!("Player moved to {}", self.player.position)
                    } else {
                        "Invalid coordinates. Usage: move <x> <y>".to_string()
                    }
                } else {
                    "Not enough arguments. Usage: move <x> <y>".to_string()
                }
            },
            "status" => {
                let status = format!(
                    "Game status - Tick: {}\nPlayer position: {}\nNPCs: {}",
                    self.tick, self.player.position, self.npcs.len()
                );
                status
            },
            "help" => {
                "Available commands:\n  move <x> <y> - Move player to coordinates\n  status - Show game status\n  json - Get game state as JSON\n  demo - Run game state demo\n  demo_tags - Run tag system demo\n  demo_mechanics - Run game mechanics demo\n  quit/exit - Exit the game\n  help - Show this help".to_string()
            },
            "json" => {
                match serde_json::to_string_pretty(self) {
                    Ok(json) => json,
                    Err(e) => format!("Error serializing to JSON: {}", e),
                }
            },
            "set" => {
                if parts.len() >= 3 {
                    let key = parts[1];
                    let value = parts[2..].join(" ");
                    self.properties.insert(key.to_string(), value.clone());
                    format!("Property '{}' set to '{}'", key, value)
                } else {
                    "Not enough arguments. Usage: set <key> <value>".to_string()
                }
            },
            "get" => {
                if parts.len() >= 2 {
                    let key = parts[1];
                    match self.properties.get(key) {
                        Some(value) => format!("{}: {}", key, value),
                        None => format!("Property '{}' not found", key),
                    }
                } else {
                    "Not enough arguments. Usage: get <key>".to_string()
                }
            },
            _ => {
                format!("Unknown command: {}. Type 'help' for available commands.", parts[0])
            }
        }
    }
    
    /// Export the game state as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Export the game state as a compact JSON string
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
} 