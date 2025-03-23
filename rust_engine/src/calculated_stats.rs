use crate::stats::{Stats, StatValue};
use crate::inventory::{Inventory, Item};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Define a struct to represent a modifier
#[derive(Serialize, Deserialize)]
pub struct StatModifier {
    pub source: String,      // Where the modifier comes from (e.g., "Sword of Power", "Warrior Buff")
    pub modifier_type: ModifierType,
    pub value: StatValue,
    pub priority: i32,       // For determining order of application
}

#[derive(Serialize, Deserialize)]
pub enum ModifierType {
    Additive,        // Simple addition/subtraction
    Multiplicative,  // Percentage-based multiplier
    Override,        // Completely replaces the value
}

#[derive(Serialize, Deserialize)]
pub struct CalculatedStats {
    // Base stats (the starting point)
    base_stats: Stats,
    
    // Store modifiers instead of separate stat collections
    modifiers: HashMap<String, Vec<StatModifier>>, // stat_name -> list of modifiers
    
    // No need for multiple dirty flags or modification counts
    #[serde(skip)]
    cache_valid: bool,
    #[serde(skip)]
    cached_results: HashMap<String, StatValue>, // For frequently accessed stats
}

impl CalculatedStats {
    pub fn new() -> Self {
        CalculatedStats {
            base_stats: Stats::new(),
            modifiers: HashMap::new(),
            cache_valid: false,
            cached_results: HashMap::new(),
        }
    }
    
    // Add a modifier to a stat
    pub fn add_modifier(&mut self, stat: &str, modifier: StatModifier) {
        let stat_modifiers = self.modifiers.entry(stat.to_string()).or_insert_with(Vec::new);
        stat_modifiers.push(modifier);
        
        // Sort modifiers by priority to ensure consistent application
        stat_modifiers.sort_by_key(|m| m.priority);
        
        // Invalidate cache for this stat
        self.cached_results.remove(stat);
        self.cache_valid = false;
    }
    
    // Remove modifiers from a particular source
    pub fn remove_modifiers_by_source(&mut self, source: &str) {
        for (_, modifiers) in self.modifiers.iter_mut() {
            modifiers.retain(|m| m.source != source);
        }
        
        // Clear entire cache when removing modifiers
        self.cached_results.clear();
        self.cache_valid = false;
    }
    
    // Calculate a stat value by applying all modifiers
    pub fn calculate_stat(&self, stat: &str) -> Option<StatValue> {
        // First check if it's in the cache
        if self.cache_valid {
            if let Some(cached) = self.cached_results.get(stat) {
                return Some(cached.clone());
            }
        }
        
        // Start with base stat
        let mut result = match self.base_stats.get(stat) {
            Some(val) => val.clone(),
            None => return None, // No base stat and no modifiers
        };
        
        // Apply modifiers in priority order
        if let Some(modifiers) = self.modifiers.get(stat) {
            for modifier in modifiers {
                match modifier.modifier_type {
                    ModifierType::Additive => {
                        // Add/subtract value
                        match (&result, &modifier.value) {
                            (StatValue::Integer(base), StatValue::Integer(mod_val)) => {
                                result = StatValue::Integer(base + mod_val);
                            },
                            (StatValue::Float(base), StatValue::Float(mod_val)) => {
                                result = StatValue::Float(base + mod_val);
                            },
                            // Handle other combinations...
                            _ => {} // Incompatible types, skip
                        }
                    },
                    ModifierType::Multiplicative => {
                        // Multiply by value
                        match (&result, &modifier.value) {
                            (StatValue::Integer(base), StatValue::Float(mod_val)) => {
                                result = StatValue::Integer(((*base as f32) * mod_val).round() as i32);
                            },
                            (StatValue::Float(base), StatValue::Float(mod_val)) => {
                                result = StatValue::Float(base * mod_val);
                            },
                            // Handle other combinations...
                            _ => {} // Incompatible types, skip
                        }
                    },
                    ModifierType::Override => {
                        // Just replace the value
                        result = modifier.value.clone();
                    }
                }
            }
        }
        
        // Store in cache
        // (in a real implementation you'd want to be selective about what gets cached)
        let mut cached_results = self.cached_results.clone();
        cached_results.insert(stat.to_string(), result.clone());
        
        Some(result)
    }
    
    // Access methods for stats with modifier application
    pub fn get(&self, key: &str) -> Option<StatValue> {
        self.calculate_stat(key)
    }
    
    pub fn get_int(&self, key: &str) -> Option<i32> {
        match self.calculate_stat(key) {
            Some(StatValue::Integer(val)) => Some(val),
            _ => None,
        }
    }
    
    pub fn get_float(&self, key: &str) -> Option<f32> {
        match self.calculate_stat(key) {
            Some(StatValue::Float(val)) => Some(val),
            _ => None,
        }
    }
    
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.calculate_stat(key) {
            Some(StatValue::Boolean(val)) => Some(val),
            _ => None,
        }
    }
    
    pub fn get_string(&self, key: &str) -> Option<String> {
        match self.calculate_stat(key) {
            Some(StatValue::String(val)) => Some(val.clone()),
            _ => None,
        }
    }
    
    // Helper to update modifiers based on inventory
    pub fn update_from_inventory(&mut self, inventory: &Inventory) {
        // Remove all equipment modifiers
        self.remove_modifiers_by_source("equipment");
        
        // Add new modifiers from equipment
        for item_id in inventory.get_all_item_ids() {
            if let Some(item) = inventory.get_item(&item_id) {
                if item.get_bool("equipped").unwrap_or(false) {
                    self.apply_item_modifiers(item);
                }
            }
        }
    }
    
    fn apply_item_modifiers(&mut self, item: &Item) {
        // Example implementation
        if let Some(damage) = item.get_int("damage") {
            self.add_modifier("attack", StatModifier {
                source: format!("equipment:{}", item.id()),
                modifier_type: ModifierType::Additive,
                value: StatValue::Integer(damage),
                priority: 10, // Equipment is applied before buffs
            });
        }
        
        // Apply other item stats...
    }
    
    // Base stat accessors
    pub fn base_stats(&self) -> &Stats {
        &self.base_stats
    }
    
    pub fn base_stats_mut(&mut self) -> &mut Stats {
        self.cache_valid = false;
        self.cached_results.clear();
        &mut self.base_stats
    }
    
    // Methods for buff management using the modifier system
    pub fn add_buff(&mut self, name: &str, stat: &str, value: StatValue, _duration: Option<f32>) {
        self.add_modifier(stat, StatModifier {
            source: format!("buff:{}", name),
            modifier_type: ModifierType::Additive, // Or whatever is appropriate
            value,
            priority: 20, // Buffs applied after equipment
        });
    }
    
    pub fn remove_buff(&mut self, name: &str) {
        self.remove_modifiers_by_source(&format!("buff:{}", name));
    }
    
    // Add this method to match the old API
    pub fn with_base_stats(base_stats: Stats) -> Self {
        let mut stats = CalculatedStats::new();
        stats.base_stats = base_stats;
        stats.cache_valid = false;
        stats
    }
    
    // Add this method to match the old API
    pub fn invalidate_cache(&mut self) {
        self.cache_valid = false;
        self.cached_results.clear();
    }
}

// Implement Default for CalculatedStats to support Serialize/Deserialize
impl Default for CalculatedStats {
    fn default() -> Self {
        CalculatedStats::new()
    }
} 