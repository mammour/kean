use std::collections::{HashMap, HashSet};
use crate::tag::{Tag, TagCollection};
use crate::property::{Property, PropertyValue, PropertyType};
use crate::stats::StatValue;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct EntityType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,    // e.g., "hostile", "friendly", "neutral", "humanoid", "animal", "plant", "object", "structure", "decoration", "resource", "item", "currency", "quest", "event", "weather", "location", "biome", "zone", "area", "region", "continent", "world", "universe", ...
    
    // Store tag IDs instead of strings for more efficient lookup and to enable properties
    pub tag_ids: HashSet<i32>,
    
    // Additional properties that don't belong to any tag
    pub properties: Vec<Property>,
}

impl EntityType {
    pub fn new(id: &str, name: &str) -> Self {
        EntityType {
            id: id.to_string(),
            name: name.to_string(),
            description: None,
            category: None,
            tag_ids: HashSet::new(),
            properties: Vec::new(),
        }
    }
    
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    
    pub fn with_category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }
    
    // Add a tag by ID
    pub fn with_tag_id(mut self, tag_id: i32) -> Self {
        self.tag_ids.insert(tag_id);
        self
    }
    
    // Add multiple tag IDs
    pub fn with_tag_ids(mut self, tag_ids: &[i32]) -> Self {
        for &id in tag_ids {
            self.tag_ids.insert(id);
        }
        self
    }
    
    // Add a property as a key-value
    pub fn with_property(mut self, key: &str, value: &str) -> Self {
        let property = Property {
            property_type: PropertyType::Custom(key.to_string()),
            value: PropertyValue::Text(value.to_string()),
            context: vec!["entity".to_string()],
            conditions: Vec::new(),
            metadata: HashMap::new(),
        };
        self.properties.push(property);
        self
    }
    
    // Add a property directly
    pub fn with_property_object(mut self, property: Property) -> Self {
        self.properties.push(property);
        self
    }
    
    // Get property by key
    pub fn get_property(&self, key: &str) -> Option<&Property> {
        self.properties.iter().find(|p| {
            match &p.property_type {
                PropertyType::Custom(k) => k == key,
                _ => false
            }
        })
    }
    
    // Get property value as text
    pub fn get_property_value(&self, key: &str) -> Option<&str> {
        if let Some(property) = self.get_property(key) {
            if let PropertyValue::Text(text) = &property.value {
                return Some(text);
            }
        }
        None
    }
    
    // Check if entity has a tag
    pub fn has_tag_id(&self, tag_id: i32) -> bool {
        self.tag_ids.contains(&tag_id)
    }
    
    // Get all tags this entity has (needs TagCollection to resolve IDs to Tags)
    pub fn get_tags<'a>(&self, tag_collection: &'a TagCollection) -> Vec<&'a Tag> {
        self.tag_ids.iter()
            .filter_map(|&id| tag_collection.get_tag(id))
            .collect()
    }
    
    // Get properties of all tags of this entity in a specific context
    pub fn get_tag_properties_in_context<'a>(&self, tag_collection: &'a TagCollection, context: &str) -> Vec<&'a crate::property::Property> {
        self.get_tags(tag_collection)
            .iter()
            .flat_map(|tag| tag.get_properties_in_context(context))
            .collect()
    }
    
    // Get entity properties in a specific context
    pub fn get_properties_in_context(&self, context: &str) -> Vec<&Property> {
        self.properties.iter()
            .filter(|p| p.applies_in_context(context))
            .collect()
    }
    
    // Get all properties (from entity and its tags)
    pub fn get_all_properties_in_context<'a>(&'a self, tag_collection: &'a TagCollection, context: &str) -> Vec<&'a Property> {
        let mut properties = self.get_tag_properties_in_context(tag_collection, context);
        let entity_properties: Vec<&'a Property> = self.properties.iter()
            .filter(|p| p.applies_in_context(context))
            .collect();
        
        properties.extend(entity_properties);
        properties
    }
    
    // Helper method to add tag by name (needs mutable TagCollection to register new tags if needed)
    pub fn with_tag_by_name(self, tag_name: &str, tag_collection: &mut TagCollection) -> Self {
        // Get existing tag ID or create new tag
        let tag_id = if let Some(tag) = tag_collection.get_tag_by_name(tag_name) {
            tag.id
        } else {
            tag_collection.add_tag(tag_name)
        };
        
        self.with_tag_id(tag_id)
    }
} 