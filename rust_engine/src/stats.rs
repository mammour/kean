use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum StatValue {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    String(String),
}

impl Clone for StatValue {
    fn clone(&self) -> Self {
        match self {
            StatValue::Integer(val) => StatValue::Integer(*val),
            StatValue::Float(val) => StatValue::Float(*val),
            StatValue::Boolean(val) => StatValue::Boolean(*val),
            StatValue::String(val) => StatValue::String(val.clone()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    values: HashMap<String, StatValue>,
    modification_count: u64,
}

impl Stats {
    // Create a completely empty stats object
    pub fn new() -> Stats {
        Stats {
            values: HashMap::new(),
            modification_count: 0,
        }
    }
    
    // Example factory method - moved from default to make it optional
    pub fn create_example_rpg_stats() -> Stats {
        let mut stats = Stats::new();
        stats.set_int("health", 100);
        stats.set_int("attack", 10);
        stats.set_int("defense", 5);
        stats.set_float("speed", 5.0);
        stats
    }
    
    // Getters
    pub fn get(&self, key: &str) -> Option<&StatValue> {
        self.values.get(key)
    }
    
    pub fn get_int(&self, key: &str) -> Option<i32> {
        match self.values.get(key) {
            Some(StatValue::Integer(value)) => Some(*value),
            _ => None,
        }
    }
    
    pub fn get_float(&self, key: &str) -> Option<f32> {
        match self.values.get(key) {
            Some(StatValue::Float(value)) => Some(*value),
            _ => None,
        }
    }
    
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.values.get(key) {
            Some(StatValue::Boolean(value)) => Some(*value),
            _ => None,
        }
    }
    
    pub fn get_string(&self, key: &str) -> Option<&String> {
        match self.values.get(key) {
            Some(StatValue::String(value)) => Some(value),
            _ => None,
        }
    }
    
    // Add method to get the modification counter
    pub fn get_modification_count(&self) -> u64 {
        self.modification_count
    }
    
    // Setters
    pub fn set(&mut self, key: &str, value: StatValue) {
        self.values.insert(key.to_string(), value);
        self.modification_count += 1;
    }
    
    pub fn set_int(&mut self, key: &str, value: i32) {
        self.values.insert(key.to_string(), StatValue::Integer(value));
        self.modification_count += 1;
    }
    
    pub fn set_float(&mut self, key: &str, value: f32) {
        self.values.insert(key.to_string(), StatValue::Float(value));
        self.modification_count += 1;
    }
    
    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.values.insert(key.to_string(), StatValue::Boolean(value));
        self.modification_count += 1;
    }
    
    pub fn set_string(&mut self, key: &str, value: String) {
        self.values.insert(key.to_string(), StatValue::String(value));
        self.modification_count += 1;
    }
    
    // Check if stat exists
    pub fn has_stat(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    
    // Remove a stat
    pub fn remove_stat(&mut self, key: &str) -> Option<StatValue> {
        let result = self.values.remove(key);
        if result.is_some() {
            self.modification_count += 1;
        }
        result
    }
    
    // Clone this stats object
    pub fn clone(&self) -> Stats {
        let mut new_stats = Stats::new();
        
        for (key, value) in &self.values {
            match value {
                StatValue::Integer(val) => new_stats.set_int(key, *val),
                StatValue::Float(val) => new_stats.set_float(key, *val),
                StatValue::Boolean(val) => new_stats.set_bool(key, *val),
                StatValue::String(val) => new_stats.set_string(key, val.clone()),
            }
        }
        
        new_stats
    }
    
    // Apply numerical modifiers to stats 
    pub fn apply_modifier(&mut self, key: &str, modifier: f32) {
        if self.has_stat(key) {
            match self.values.get(key) {
                Some(StatValue::Integer(val)) => {
                    let new_val = (*val as f32 * modifier).round() as i32;
                    self.set_int(key, new_val);
                },
                Some(StatValue::Float(val)) => {
                    let new_val = *val * modifier;
                    self.set_float(key, new_val);
                },
                _ => {} // Do nothing for non-numerical stats
            }
        }
    }
    
    // Get all stat keys
    pub fn get_all_keys(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }
}