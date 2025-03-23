use std::collections::HashMap;
use crate::stats::StatValue;
use serde::{Serialize, Deserialize};

// A flexible property that can represent various attributes and behaviors
#[derive(Clone, Serialize, Deserialize)]
pub struct Property {
    pub property_type: PropertyType,
    pub value: PropertyValue,
    pub context: Vec<String>,        // In what contexts this property applies (e.g., "combat", "exploration")
    pub conditions: Vec<Condition>,  // Conditions under which this property is active
    pub metadata: HashMap<String, String>, // Additional metadata for special use cases
}

// Different types of properties
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyType {
    StatModifier,    // Modifies a stat (attack, defense, etc.)
    Ability,         // Provides a special ability
    Behavior,        // Affects behavior (AI, movement, etc.)
    Reaction,        // How entity reacts to something
    Trigger,         // Causes something to happen
    Requirement,     // Requirement for something to happen
    Visual,          // Visual effect
    Audio,           // Sound effect
    Custom(String),  // Custom property type for game-specific uses
}

// The value of a property
#[derive(Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    Stat(String, StatValue),    // Stat name and value modifier
    Function(String),           // Function ID to call
    Script(String),             // Script to execute
    Asset(String),              // Asset path
    Data(HashMap<String, StatValue>), // Structured data
    Flag(bool),                 // Simple boolean flag
    Text(String),               // Text description
    Custom(String, String),     // Custom key-value for game-specific uses
}

// Conditions under which a property applies
#[derive(Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub parameters: HashMap<String, StatValue>,
}

// Types of conditions
#[derive(Clone, Serialize, Deserialize)]
pub enum ConditionType {
    StatThreshold,     // When a stat is above/below threshold
    HasTag,            // When entity has a specific tag
    InState,           // When entity is in a specific state
    TimeOfDay,         // Based on game time
    Proximity,         // When near/far from something
    InventoryContains, // When inventory has an item
    Custom(String),    // Custom condition
}

impl Property {
    // Create a stat modifier property
    pub fn stat_modifier(stat_name: &str, value: StatValue) -> Self {
        Property {
            property_type: PropertyType::StatModifier,
            value: PropertyValue::Stat(stat_name.to_string(), value),
            context: vec!["default".to_string()],
            conditions: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    // Create an ability property
    pub fn ability(ability_id: &str) -> Self {
        Property {
            property_type: PropertyType::Ability,
            value: PropertyValue::Function(ability_id.to_string()),
            context: vec!["default".to_string()],
            conditions: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    // Add a context to a property
    pub fn with_context(mut self, context: &str) -> Self {
        self.context.push(context.to_string());
        self
    }
    
    // Add a condition to a property
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }
    
    // Add metadata to a property
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
    
    // Check if property applies in a given context
    pub fn applies_in_context(&self, context: &str) -> bool {
        self.context.contains(&context.to_string()) || 
        self.context.contains(&"default".to_string())
    }
    
    // Helper for creating a stat threshold condition
    pub fn create_stat_threshold_condition(stat: &str, threshold: StatValue, is_greater_than: bool) -> Condition {
        let mut parameters = HashMap::new();
        parameters.insert("stat".to_string(), StatValue::String(stat.to_string()));
        parameters.insert("threshold".to_string(), threshold);
        parameters.insert("is_greater_than".to_string(), StatValue::Boolean(is_greater_than));
        
        Condition {
            condition_type: ConditionType::StatThreshold,
            parameters,
        }
    }
    
    // Helper for creating a has tag condition
    pub fn create_has_tag_condition(tag: &str) -> Condition {
        let mut parameters = HashMap::new();
        parameters.insert("tag".to_string(), StatValue::String(tag.to_string()));
        
        Condition {
            condition_type: ConditionType::HasTag,
            parameters,
        }
    }
} 