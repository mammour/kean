use std::collections::HashMap;
use crate::stats::{Stats, StatValue};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ItemValue {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    String(String),
    Stats(Stats),
}

impl Clone for ItemValue {
    fn clone(&self) -> Self {
        match self {
            ItemValue::Integer(val) => ItemValue::Integer(*val),
            ItemValue::Float(val) => ItemValue::Float(*val),
            ItemValue::Boolean(val) => ItemValue::Boolean(*val),
            ItemValue::String(val) => ItemValue::String(val.clone()),
            ItemValue::Stats(val) => ItemValue::Stats(val.clone()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: String,
    name: String,
    properties: HashMap<String, ItemValue>,
}

impl Item {
    pub fn new(id: &str, name: &str) -> Item {
        Item {
            id: id.to_string(),
            name: name.to_string(),
            properties: HashMap::new(),
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    
    // Property getters
    pub fn get(&self, key: &str) -> Option<&ItemValue> {
        self.properties.get(key)
    }
    
    pub fn get_int(&self, key: &str) -> Option<i32> {
        match self.properties.get(key) {
            Some(ItemValue::Integer(value)) => Some(*value),
            _ => None,
        }
    }
    
    pub fn get_float(&self, key: &str) -> Option<f32> {
        match self.properties.get(key) {
            Some(ItemValue::Float(value)) => Some(*value),
            _ => None,
        }
    }
    
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.properties.get(key) {
            Some(ItemValue::Boolean(value)) => Some(*value),
            _ => None,
        }
    }
    
    pub fn get_string(&self, key: &str) -> Option<&String> {
        match self.properties.get(key) {
            Some(ItemValue::String(value)) => Some(value),
            _ => None,
        }
    }
    
    pub fn get_stats(&self, key: &str) -> Option<&Stats> {
        match self.properties.get(key) {
            Some(ItemValue::Stats(value)) => Some(value),
            _ => None,
        }
    }
    
    // Property setters
    pub fn set(&mut self, key: &str, value: ItemValue) {
        self.properties.insert(key.to_string(), value);
    }
    
    pub fn set_int(&mut self, key: &str, value: i32) {
        self.properties.insert(key.to_string(), ItemValue::Integer(value));
    }
    
    pub fn set_float(&mut self, key: &str, value: f32) {
        self.properties.insert(key.to_string(), ItemValue::Float(value));
    }
    
    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.properties.insert(key.to_string(), ItemValue::Boolean(value));
    }
    
    pub fn set_string(&mut self, key: &str, value: String) {
        self.properties.insert(key.to_string(), ItemValue::String(value));
    }
    
    pub fn set_stats(&mut self, key: &str, value: Stats) {
        self.properties.insert(key.to_string(), ItemValue::Stats(value));
    }
    
    // Check if property exists
    pub fn has_property(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }
    
    // Remove a property
    pub fn remove_property(&mut self, key: &str) -> Option<ItemValue> {
        self.properties.remove(key)
    }
    
    // Clone this item
    pub fn clone(&self) -> Item {
        let mut new_item = Item::new(&self.id, &self.name);
        
        for (key, value) in &self.properties {
            new_item.properties.insert(key.clone(), value.clone());
        }
        
        new_item
    }
}

// Move specific factory functions to a separate module or make them examples
pub mod examples {
    use super::*;
    
    pub fn create_weapon(id: &str, name: &str, damage: i32) -> Item {
        let mut item = Item::new(id, name);
        item.set_string("type", "weapon".to_string());
        item.set_int("damage", damage);
        item
    }
    
    pub fn create_armor(id: &str, name: &str, defense: i32) -> Item {
        let mut item = Item::new(id, name);
        item.set_string("type", "armor".to_string());
        item.set_int("defense", defense);
        item
    }
    
    pub fn create_potion(id: &str, name: &str, healing: i32) -> Item {
        let mut item = Item::new(id, name);
        item.set_string("type", "potion".to_string());
        item.set_int("healing", healing);
        item
    }
}

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    items: HashMap<String, Item>,
    capacity: Option<usize>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: HashMap::new(),
            capacity: None,
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Inventory {
        Inventory {
            items: HashMap::new(),
            capacity: Some(capacity),
        }
    }
    
    pub fn capacity(&self) -> Option<usize> {
        self.capacity
    }
    
    pub fn set_capacity(&mut self, capacity: Option<usize>) {
        self.capacity = capacity;
    }
    
    pub fn count(&self) -> usize {
        self.items.len()
    }
    
    pub fn is_full(&self) -> bool {
        if let Some(capacity) = self.capacity {
            self.items.len() >= capacity
        } else {
            false
        }
    }
    
    pub fn add_item(&mut self, item: Item) -> bool {
        // Check capacity
        if self.is_full() {
            return false;
        }
        
        let item_id = item.id().to_string();
        self.items.insert(item_id, item);
        true
    }
    
    pub fn remove_item(&mut self, item_id: &str) -> Option<Item> {
        self.items.remove(item_id)
    }
    
    pub fn has_item(&self, item_id: &str) -> bool {
        self.items.contains_key(item_id)
    }
    
    pub fn get_item(&self, item_id: &str) -> Option<&Item> {
        self.items.get(item_id)
    }
    
    pub fn get_mut_item(&mut self, item_id: &str) -> Option<&mut Item> {
        self.items.get_mut(item_id)
    }
    
    pub fn get_all_items(&self) -> Vec<&Item> {
        self.items.values().collect()
    }
    
    pub fn get_all_item_ids(&self) -> Vec<String> {
        self.items.keys().cloned().collect()
    }
    
    // Filter items by a property
    pub fn filter_by_property(&self, key: &str, value: &ItemValue) -> Vec<&Item> {
        self.items.values()
            .filter(|item| {
                if let Some(prop_value) = item.get(key) {
                    match (prop_value, value) {
                        (ItemValue::Integer(a), ItemValue::Integer(b)) => a == b,
                        (ItemValue::Float(a), ItemValue::Float(b)) => a == b,
                        (ItemValue::Boolean(a), ItemValue::Boolean(b)) => a == b,
                        (ItemValue::String(a), ItemValue::String(b)) => a == b,
                        _ => false,
                    }
                } else {
                    false
                }
            })
            .collect()
    }
    
    // Find all items by type
    pub fn get_items_by_type(&self, item_type: &str) -> Vec<&Item> {
        self.filter_by_property("type", &ItemValue::String(item_type.to_string()))
    }
    
    // Clone this inventory
    pub fn clone(&self) -> Inventory {
        let mut new_inventory = match self.capacity {
            Some(cap) => Inventory::with_capacity(cap),
            None => Inventory::new(),
        };
        
        for item in self.items.values() {
            new_inventory.add_item(item.clone());
        }
        
        new_inventory
    }
} 