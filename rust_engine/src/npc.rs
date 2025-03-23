use crate::entity_type::EntityType;
use crate::calculated_stats::{CalculatedStats, StatModifier, ModifierType};
use crate::stats::{Stats, StatValue};
use crate::coordinates::Coordinates;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NPC {
    pub id: String,
    pub npc_type: EntityType,
    pub position: Coordinates,
    
    // Generic properties map for any game-specific data
    properties: HashMap<String, StatValue>,
    
    // Using the same CalculatedStats system as Character for maximum flexibility
    #[serde(skip)]
    calculated_stats: CalculatedStats,
    
    // Behavior flags and state
    pub behavior_state: String,
    pub status_effects: Vec<String>,
}

impl NPC {
    pub fn new(id: String, npc_type: EntityType) -> NPC {
        NPC {
            id,
            npc_type,
            position: Coordinates::new_2d(0.0, 0.0),
            properties: HashMap::new(),
            calculated_stats: CalculatedStats::new(),
            behavior_state: "idle".to_string(),
            status_effects: Vec::new(),
        }
    }
    
    /// Create a NPC with a specific number of dimensions for position
    pub fn with_dimensions(id: String, npc_type: EntityType, dimensions: usize) -> NPC {
        NPC {
            id,
            npc_type,
            position: Coordinates::new(dimensions),
            properties: HashMap::new(),
            calculated_stats: CalculatedStats::new(),
            behavior_state: "idle".to_string(),
            status_effects: Vec::new(),
        }
    }
    
    /// Create a NPC in a 1D world (like a timeline)
    pub fn new_1d(id: String, npc_type: EntityType, x: f32) -> NPC {
        NPC {
            id,
            npc_type,
            position: Coordinates::new_1d(x),
            properties: HashMap::new(),
            calculated_stats: CalculatedStats::new(),
            behavior_state: "idle".to_string(),
            status_effects: Vec::new(),
        }
    }
    
    /// Create a NPC in a 3D world
    pub fn new_3d(id: String, npc_type: EntityType, x: f32, y: f32, z: f32) -> NPC {
        NPC {
            id,
            npc_type,
            position: Coordinates::new_3d(x, y, z),
            properties: HashMap::new(),
            calculated_stats: CalculatedStats::new(),
            behavior_state: "idle".to_string(),
            status_effects: Vec::new(),
        }
    }
    
    /// Create a NPC in a 4D world (3D + time)
    pub fn new_4d(id: String, npc_type: EntityType, x: f32, y: f32, z: f32, t: f32) -> NPC {
        NPC {
            id,
            npc_type,
            position: Coordinates::new_4d(x, y, z, t),
            properties: HashMap::new(),
            calculated_stats: CalculatedStats::new(),
            behavior_state: "idle".to_string(),
            status_effects: Vec::new(),
        }
    }
    
    /// Get position value for a specific dimension
    pub fn get_position(&self, dimension: usize) -> Option<f32> {
        self.position.get(dimension)
    }
    
    /// Set position value for a specific dimension
    pub fn set_position_dimension(&mut self, dimension: usize, value: f32) -> bool {
        self.position.set(dimension, value)
    }
    
    /// Get position value by dimension label
    pub fn get_position_by_label(&self, label: &str) -> Option<f32> {
        self.position.get_by_label(label)
    }
    
    /// Set position value by dimension label
    pub fn set_position_by_label(&mut self, label: &str, value: f32) -> bool {
        self.position.set_by_label(label, value)
    }
    
    /// Set all position values at once
    pub fn set_position_values(&mut self, values: Vec<f32>) {
        if values.len() == self.position.dimensions() {
            for (i, value) in values.iter().enumerate() {
                self.position.set(i, *value);
            }
        }
    }
    
    /// For backward compatibility: get x coordinate (first dimension)
    pub fn x(&self) -> f32 {
        self.position.get(0).unwrap_or(0.0)
    }
    
    /// For backward compatibility: get y coordinate (second dimension)
    pub fn y(&self) -> f32 {
        self.position.get(1).unwrap_or(0.0)
    }
    
    // Example factory method for classic health-based NPCs
    pub fn create_combat_npc(id: String, npc_type: EntityType, hp: i32, speed: f32, attack: i32) -> NPC {
        let mut npc = NPC::new(id, npc_type);
        
        // Set health-based stats
        npc.set_base_stat("hp", StatValue::Integer(hp));
        npc.set_base_stat("max_hp", StatValue::Integer(hp));
        npc.set_base_stat("speed", StatValue::Float(speed));
        npc.set_base_stat("attack", StatValue::Integer(attack));
        npc.set_base_stat("attack_cooldown", StatValue::Float(1.0));
        
        npc
    }
    
    // Example factory method for "love-meter" fan-based NPCs
    pub fn create_fan_npc(id: String, npc_type: EntityType, initial_adoration: i32, attention_span: f32) -> NPC {
        let mut npc = NPC::new(id, npc_type);
        
        // Set fan-based stats
        npc.set_base_stat("adoration", StatValue::Integer(initial_adoration));
        npc.set_base_stat("max_adoration", StatValue::Integer(100));
        npc.set_base_stat("attention_span", StatValue::Float(attention_span));
        npc.set_base_stat("fandom_level", StatValue::Integer(1));
        
        npc
    }
    
    // Generic property access
    pub fn set_property(&mut self, key: &str, value: StatValue) {
        self.properties.insert(key.to_string(), value);
    }
    
    pub fn get_property(&self, key: &str) -> Option<&StatValue> {
        self.properties.get(key)
    }
    
    pub fn get_int_property(&self, key: &str) -> Option<i32> {
        match self.get_property(key) {
            Some(StatValue::Integer(val)) => Some(*val),
            _ => None,
        }
    }
    
    pub fn get_float_property(&self, key: &str) -> Option<f32> {
        match self.get_property(key) {
            Some(StatValue::Float(val)) => Some(*val),
            _ => None,
        }
    }
    
    // Base stats access (same API as Character)
    pub fn base_stats(&self) -> &Stats {
        self.calculated_stats.base_stats()
    }
    
    pub fn base_stats_mut(&mut self) -> &mut Stats {
        self.calculated_stats.base_stats_mut()
    }
    
    pub fn set_base_stat(&mut self, key: &str, value: StatValue) {
        self.calculated_stats.base_stats_mut().set(key, value);
    }
    
    // Calculated stats access with modifiers applied
    pub fn get_stat(&self, key: &str) -> Option<StatValue> {
        self.calculated_stats.get(key)
    }
    
    pub fn get_int_stat(&self, key: &str) -> Option<i32> {
        self.calculated_stats.get_int(key)
    }
    
    pub fn get_float_stat(&self, key: &str) -> Option<f32> {
        self.calculated_stats.get_float(key)
    }
    
    // Add a modifier to a stat
    pub fn add_stat_modifier(&mut self, stat: &str, source: &str, mod_type: ModifierType, value: StatValue, priority: i32) {
        let modifier = StatModifier {
            source: source.to_string(),
            modifier_type: mod_type,
            value,
            priority,
        };
        self.calculated_stats.add_modifier(stat, modifier);
    }
    
    // Status effect management
    pub fn add_status_effect(&mut self, effect: &str) {
        if !self.status_effects.contains(&effect.to_string()) {
            self.status_effects.push(effect.to_string());
        }
    }
    
    pub fn remove_status_effect(&mut self, effect: &str) {
        self.status_effects.retain(|e| e != effect);
    }
    
    pub fn has_status_effect(&self, effect: &str) -> bool {
        self.status_effects.contains(&effect.to_string())
    }
    
    // Change behavior state
    pub fn set_behavior_state(&mut self, state: &str) {
        self.behavior_state = state.to_string();
    }
    
    // Movement helpers for backward compatibility
    pub fn set_position(&mut self, x: f32, y: f32) {
        if self.position.dimensions() >= 2 {
            self.position.set(0, x);
            self.position.set(1, y);
        }
    }
    
    // Move toward another position
    pub fn move_toward(&mut self, target: &Coordinates, delta_time: f32) {
        let speed = self.get_float_stat("speed").unwrap_or(1.0);
        let distance = speed * delta_time;
        self.position.move_toward(target, distance);
    }
    
    // Move toward another NPC
    pub fn move_toward_npc(&mut self, target: &NPC, delta_time: f32) {
        self.move_toward(&target.position, delta_time);
    }
    
    /// Calculate distance to another NPC
    pub fn distance_to(&self, other: &NPC) -> f32 {
        self.position.distance(&other.position)
    }
    
    // Game-specific interaction methods
    
    // For classic health-based games
    pub fn take_damage(&mut self, amount: i32) -> bool {
        if let Some(current_hp) = self.get_int_stat("hp") {
            let new_hp = (current_hp - amount).max(0);
            self.set_base_stat("hp", StatValue::Integer(new_hp));
            return new_hp <= 0; // Return true if NPC is defeated
        }
        false
    }
    
    // For adoration-based/fan games
    pub fn receive_adoration(&mut self, amount: i32) -> bool {
        if let Some(current_adoration) = self.get_int_stat("adoration") {
            let max_adoration = self.get_int_stat("max_adoration").unwrap_or(100);
            let new_adoration = (current_adoration + amount).min(max_adoration);
            self.set_base_stat("adoration", StatValue::Integer(new_adoration));
            return new_adoration >= max_adoration; // Return true if max adoration reached
        }
        false
    }
    
    // Attack logic based on whatever stats the game designer chose
    pub fn can_attack(&mut self, current_time: f32) -> bool {
        if let (Some(last_attack_time), Some(cooldown)) = (
            self.get_float_property("last_attack_time"),
            self.get_float_stat("attack_cooldown")
        ) {
            if current_time - last_attack_time >= cooldown {
                self.set_property("last_attack_time", StatValue::Float(current_time));
                return true;
            }
            return false;
        }
        
        // First attack or no cooldown specified
        self.set_property("last_attack_time", StatValue::Float(current_time));
        true
    }
} 