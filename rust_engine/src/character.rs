use crate::stats::{Stats, StatValue};
use crate::inventory::{Inventory, Item};
use crate::calculated_stats::CalculatedStats;
use crate::coordinates::Coordinates;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub position: Coordinates,
    pub inventory: Inventory,
    #[serde(skip)]
    cached_stats: CalculatedStats,
}

impl Character {
    pub fn new() -> Character {
        Character {
            position: Coordinates::new_2d(0.0, 0.0),
            inventory: Inventory::new(),
            cached_stats: CalculatedStats::new(),
        }
    }
    
    /// Create a character with a specific number of dimensions for position
    pub fn with_dimensions(dimensions: usize) -> Character {
        Character {
            position: Coordinates::new(dimensions),
            inventory: Inventory::new(),
            cached_stats: CalculatedStats::new(),
        }
    }
    
    /// Create a character in a 1D world (like a timeline)
    pub fn new_1d(x: f32) -> Character {
        Character {
            position: Coordinates::new_1d(x),
            inventory: Inventory::new(),
            cached_stats: CalculatedStats::new(),
        }
    }
    
    /// Create a character in a 3D world
    pub fn new_3d(x: f32, y: f32, z: f32) -> Character {
        Character {
            position: Coordinates::new_3d(x, y, z),
            inventory: Inventory::new(),
            cached_stats: CalculatedStats::new(),
        }
    }
    
    /// Create a character in a 4D world (3D + time)
    pub fn new_4d(x: f32, y: f32, z: f32, t: f32) -> Character {
        Character {
            position: Coordinates::new_4d(x, y, z, t),
            inventory: Inventory::new(),
            cached_stats: CalculatedStats::new(),
        }
    }
    
    /// Get position value for a specific dimension
    pub fn get_position(&self, dimension: usize) -> Option<f32> {
        self.position.get(dimension)
    }
    
    /// Set position value for a specific dimension
    pub fn set_position(&mut self, dimension: usize, value: f32) -> bool {
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
    
    /// Move toward a target position by a specific distance
    pub fn move_toward(&mut self, target: &Coordinates, distance: f32) -> bool {
        self.position.move_toward(target, distance)
    }
    
    /// Calculate distance to another character
    pub fn distance_to(&self, other: &Character) -> f32 {
        self.position.distance(&other.position)
    }
    
    /// For backward compatibility: get x coordinate (first dimension)
    pub fn x(&self) -> f32 {
        self.position.get(0).unwrap_or(0.0)
    }
    
    /// For backward compatibility: get y coordinate (second dimension)
    pub fn y(&self) -> f32 {
        self.position.get(1).unwrap_or(0.0)
    }
    
    // Create a character with custom base stats
    pub fn with_stats(base_stats: Stats) -> Character {
        Character {
            position: Coordinates::new_2d(0.0, 0.0),
            inventory: Inventory::new(),
            cached_stats: CalculatedStats::with_base_stats(base_stats),
        }
    }
    
    // Create a character with custom inventory
    pub fn with_inventory(custom_inventory: Inventory) -> Character {
        let mut character = Character {
            position: Coordinates::new_2d(0.0, 0.0),
            inventory: custom_inventory,
            cached_stats: CalculatedStats::new(),
        };
        // Update stats based on inventory
        character.update_stats_from_inventory();
        character
    }
    
    // Access to base stats
    pub fn base_stats(&self) -> &Stats {
        self.cached_stats.base_stats()
    }
    
    pub fn base_stats_mut(&mut self) -> &mut Stats {
        self.cached_stats.base_stats_mut()
    }
    
    // Get a stat from the calculated stats
    pub fn get_stat(&mut self, key: &str) -> Option<StatValue> {
        self.cached_stats.get(key)
    }
    
    pub fn get_int_stat(&mut self, key: &str) -> Option<i32> {
        self.cached_stats.get_int(key)
    }
    
    pub fn get_float_stat(&mut self, key: &str) -> Option<f32> {
        self.cached_stats.get_float(key)
    }
    
    pub fn get_string_stat(&mut self, key: &str) -> Option<String> {
        self.cached_stats.get_string(key)
    }
    
    // Set a base stat
    pub fn set_base_stat(&mut self, key: &str, value: StatValue) {
        self.cached_stats.base_stats_mut().set(key, value);
    }
    
    // Update equipment stats when inventory changes
    pub fn update_stats_from_inventory(&mut self) {
        self.cached_stats.update_from_inventory(&self.inventory);
    }
    
    // Inventory operations with stat updates
    pub fn add_item(&mut self, item: Item) -> bool {
        let result = self.inventory.add_item(item);
        if result {
            self.update_stats_from_inventory();
        }
        result
    }
    
    pub fn remove_item(&mut self, item_id: &str) -> Option<Item> {
        let result = self.inventory.remove_item(item_id);
        if result.is_some() {
            self.update_stats_from_inventory();
        }
        result
    }
    
    pub fn equip_item(&mut self, item_id: &str) -> bool {
        if let Some(item) = self.inventory.get_mut_item(item_id) {
            item.set_bool("equipped", true);
            self.update_stats_from_inventory();
            true
        } else {
            false
        }
    }
    
    pub fn unequip_item(&mut self, item_id: &str) -> bool {
        if let Some(item) = self.inventory.get_mut_item(item_id) {
            item.set_bool("equipped", false);
            self.update_stats_from_inventory();
            true
        } else {
            false
        }
    }
    
    // ... other inventory methods ...
    
    // Buff management
    pub fn add_buff(&mut self, name: &str, stat: &str, value: StatValue, _duration: Option<f32>) {
        self.cached_stats.add_buff(name, stat, value, _duration);
    }
    
    pub fn remove_buff(&mut self, stat: &str) {
        self.cached_stats.remove_buff(stat);
    }
    
    // Force recalculation of stats if needed
    pub fn invalidate_stat_cache(&mut self) {
        self.cached_stats.invalidate_cache();
    }
} 