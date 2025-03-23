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
                "Available commands:\n  move <x> <y> - Move player to coordinates\n  status - Show game status\n  json - Get game state as JSON\n  demo - Run game state demo\n  demo_tags - Run tag system demo\n  demo_mechanics - Run game mechanics demo\n  demo_assets - Run asset management demo\n  quit/exit - Exit the game\n  help - Show this help".to_string()
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
            "demo_tags" => {
                use crate::demos::demo_tag_system;
                demo_tag_system();
                "Tag system demo completed".to_string()
            },
            "demo_mechanics" => {
                use crate::demos::showcase_different_game_mechanics;
                showcase_different_game_mechanics();
                "Game mechanics demo completed".to_string()
            },
            "demo" => {
                use crate::demos::demo_game_state;
                demo_game_state();
                "Game state demo completed".to_string()
            },
            "demo_assets" => {
                use crate::demos::demo_asset_management;
                demo_asset_management();
                "Asset management demo completed".to_string()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_type::EntityType;
    use crate::npc::NPC;

    #[test]
    fn test_game_state_creation() {
        let game_state = GameState::new();
        
        // Check default values
        assert_eq!(game_state.tick, 0);
        assert_eq!(game_state.npcs.len(), 0);
        assert_eq!(game_state.entity_types.len(), 0);
        assert_eq!(game_state.tag_collection.tag_count(), 0);
        assert_eq!(game_state.game_time, 0.0);
        assert!(game_state.running);
        assert_eq!(game_state.properties.len(), 0);
    }

    #[test]
    fn test_game_state_update() {
        let mut game_state = GameState::new();
        
        // Initial values
        assert_eq!(game_state.tick, 0);
        assert_eq!(game_state.game_time, 0.0);
        
        // Update with delta time
        game_state.update(0.1);
        
        // Check updated values
        assert_eq!(game_state.tick, 1);
        assert_eq!(game_state.game_time, 0.1);
        
        // Multiple updates
        game_state.update(0.1);
        game_state.update(0.1);
        
        assert_eq!(game_state.tick, 3);
        assert_eq!(game_state.game_time, 0.3);
    }

    #[test]
    fn test_process_command_move() {
        let mut game_state = GameState::new();
        
        // Test initial position
        assert_eq!(game_state.player.position.get(0), Some(0.0));
        assert_eq!(game_state.player.position.get(1), Some(0.0));
        
        // Process move command
        let result = game_state.process_command("move 10.5 -3.25");
        
        // Check player moved
        assert_eq!(game_state.player.position.get(0), Some(10.5));
        assert_eq!(game_state.player.position.get(1), Some(-3.25));
        
        // Check result message
        assert!(result.contains("Player moved to"));
    }

    #[test]
    fn test_process_command_invalid_move() {
        let mut game_state = GameState::new();
        
        // Invalid move command (missing coordinates)
        let result = game_state.process_command("move");
        assert!(result.contains("Not enough arguments"));
        
        // Invalid move command (non-numeric coordinates)
        let result = game_state.process_command("move abc def");
        assert!(result.contains("Invalid coordinates"));
    }

    #[test]
    fn test_process_command_status() {
        let mut game_state = GameState::new();
        
        // Process status command
        let result = game_state.process_command("status");
        
        // Check result contains expected information
        assert!(result.contains("Game status"));
        assert!(result.contains("Tick: 0"));
        assert!(result.contains("Player position"));
        assert!(result.contains("NPCs: 0"));
    }

    #[test]
    fn test_process_command_help() {
        let mut game_state = GameState::new();
        
        // Process help command
        let result = game_state.process_command("help");
        
        // Check result contains expected information
        assert!(result.contains("Available commands"));
        assert!(result.contains("move <x> <y>"));
        assert!(result.contains("status"));
        assert!(result.contains("help"));
    }

    #[test]
    fn test_process_command_set_get() {
        let mut game_state = GameState::new();
        
        // Set a property
        let set_result = game_state.process_command("set difficulty hard");
        assert!(set_result.contains("Property 'difficulty' set to 'hard'"));
        
        // Get the property
        let get_result = game_state.process_command("get difficulty");
        assert_eq!(get_result, "difficulty: hard");
        
        // Get a non-existent property
        let get_nonexistent = game_state.process_command("get nonexistent");
        assert!(get_nonexistent.contains("not found"));
    }

    #[test]
    fn test_add_npcs() {
        let mut game_state = GameState::new();
        
        // Create entity type and NPC
        let entity_type = EntityType::new("goblin", "Goblin")
            .with_description("A small green creature")
            .with_category("enemy");
        
        let npc = NPC::new("goblin1".to_string(), entity_type.clone());
        
        // Add entity type to game state
        game_state.entity_types.insert("goblin".to_string(), entity_type);
        
        // Add NPC to game state
        game_state.npcs.push(npc);
        
        // Check NPC was added
        assert_eq!(game_state.npcs.len(), 1);
        assert_eq!(game_state.npcs[0].id, "goblin1");
        assert_eq!(game_state.entity_types.len(), 1);
        assert!(game_state.entity_types.contains_key("goblin"));
    }

    #[test]
    fn test_exit_command() {
        let mut game_state = GameState::new();
        assert!(game_state.running);
        
        // Process exit command
        let result = game_state.process_command("exit");
        assert_eq!(result, "Shutting down...");
        assert!(!game_state.running);
        
        // Process quit command (alternative)
        let mut game_state = GameState::new();
        assert!(game_state.running);
        
        let result = game_state.process_command("quit");
        assert_eq!(result, "Shutting down...");
        assert!(!game_state.running);
    }
} 