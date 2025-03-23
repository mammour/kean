use std::collections::{HashMap, HashSet};
use crate::property::Property;
use serde::{Serialize, Deserialize};
use crate::stats::StatValue as Stats_StatValue;

// Tag structure with ID, name, and properties
#[derive(Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub properties: Vec<Property>,
    pub metadata: HashMap<String, String>,
}

impl Tag {
    pub fn new(id: i32, name: &str) -> Self {
        Tag {
            id,
            name: name.to_string(),
            properties: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    // Add a property to this tag
    pub fn with_property(mut self, property: Property) -> Self {
        self.properties.push(property);
        self
    }
    
    // Add multiple properties
    pub fn with_properties(mut self, properties: Vec<Property>) -> Self {
        self.properties.extend(properties);
        self
    }
    
    // Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
    
    // Get properties of a certain type
    pub fn get_properties_by_type(&self, property_type: &crate::property::PropertyType) -> Vec<&Property> {
        self.properties.iter()
            .filter(|p| &p.property_type == property_type)
            .collect()
    }
    
    // Get properties that apply in a context
    pub fn get_properties_in_context(&self, context: &str) -> Vec<&Property> {
        self.properties.iter()
            .filter(|p| p.applies_in_context(context))
            .collect()
    }
}

// A collection of tags with lookup capabilities
#[derive(Serialize, Deserialize)]
pub struct TagCollection {
    tags: HashMap<i32, Tag>,
    name_to_id: HashMap<String, i32>,
    next_id: i32,
}

impl TagCollection {
    pub fn new() -> Self {
        TagCollection {
            tags: HashMap::new(),
            name_to_id: HashMap::new(),
            next_id: 1, // Start IDs at 1
        }
    }
    
    // Add a tag with auto-incremented ID
    pub fn add_tag(&mut self, name: &str) -> i32 {
        let id = self.next_id;
        self.add_tag_with_id(id, name);
        self.next_id += 1;
        id
    }
    
    // Add a tag with specific ID
    pub fn add_tag_with_id(&mut self, id: i32, name: &str) -> bool {
        if self.tags.contains_key(&id) || self.name_to_id.contains_key(name) {
            return false; // Tag already exists
        }
        
        self.tags.insert(id, Tag::new(id, name));
        self.name_to_id.insert(name.to_string(), id);
        true
    }
    
    // Get a tag by ID
    pub fn get_tag(&self, id: i32) -> Option<&Tag> {
        self.tags.get(&id)
    }
    
    // Get a tag by name
    pub fn get_tag_by_name(&self, name: &str) -> Option<&Tag> {
        self.name_to_id.get(name).and_then(|id| self.tags.get(id))
    }
    
    // Get a mutable tag reference by ID
    pub fn get_tag_mut(&mut self, id: i32) -> Option<&mut Tag> {
        self.tags.get_mut(&id)
    }
    
    // Get a mutable tag reference by name
    pub fn get_tag_mut_by_name(&mut self, name: &str) -> Option<&mut Tag> {
        if let Some(id) = self.name_to_id.get(name) {
            self.tags.get_mut(id)
        } else {
            None
        }
    }
    
    // Remove a tag by ID
    pub fn remove_tag(&mut self, id: i32) -> bool {
        if let Some(tag) = self.tags.remove(&id) {
            self.name_to_id.remove(&tag.name);
            true
        } else {
            false
        }
    }
    
    // Get all tags with a certain property type
    pub fn get_tags_with_property_type(&self, property_type: &crate::property::PropertyType) -> Vec<&Tag> {
        self.tags.values()
            .filter(|tag| tag.properties.iter().any(|p| p.property_type == *property_type))
            .collect()
    }
    
    // Get all tags that apply in a certain context
    pub fn get_tags_in_context(&self, context: &str) -> Vec<&Tag> {
        self.tags.values()
            .filter(|tag| tag.properties.iter().any(|p| p.applies_in_context(context)))
            .collect()
    }
    
    // Get all tags
    pub fn get_all_tags(&self) -> Vec<&Tag> {
        self.tags.values().collect()
    }
    
    // Get tags that match a filter function
    pub fn filter_tags<F>(&self, filter: F) -> Vec<&Tag>
    where F: Fn(&Tag) -> bool {
        self.tags.values().filter(|tag| filter(tag)).collect()
    }
    
    // Get the number of tags in the collection
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::{Property, PropertyType, PropertyValue};
    use crate::stats::StatValue as Stats_StatValue;

    #[test]
    fn test_tag_creation() {
        let tag = Tag::new(1, "fire");
        
        assert_eq!(tag.id, 1);
        assert_eq!(tag.name, "fire");
        assert_eq!(tag.properties.len(), 0);
        assert_eq!(tag.metadata.len(), 0);
    }

    #[test]
    fn test_tag_with_properties() {
        let property = Property::stat_modifier("damage", Stats_StatValue::Integer(5));
        
        let tag = Tag::new(1, "fire")
            .with_property(property);
        
        assert_eq!(tag.properties.len(), 1);
        if let PropertyValue::Stat(name, _) = &tag.properties[0].value {
            assert_eq!(name, "damage");
        } else {
            panic!("Expected Stat property type");
        }
    }

    #[test]
    fn test_tag_with_metadata() {
        let tag = Tag::new(1, "fire")
            .with_metadata("element", "fire")
            .with_metadata("color", "#FF4500");
        
        assert_eq!(tag.metadata.len(), 2);
        assert_eq!(tag.metadata.get("element"), Some(&"fire".to_string()));
        assert_eq!(tag.metadata.get("color"), Some(&"#FF4500".to_string()));
    }

    #[test]
    fn test_tag_get_properties_in_context() {
        let property1 = Property::stat_modifier("damage", Stats_StatValue::Integer(5))
            .with_context("combat");
        
        let property2 = Property::stat_modifier("speed", Stats_StatValue::Float(0.8))
            .with_context("movement");
        
        let property3 = Property::stat_modifier("resistance", Stats_StatValue::Integer(10))
            .with_context("combat");
        
        let tag = Tag::new(1, "fire")
            .with_property(property1)
            .with_property(property2)
            .with_property(property3);
        
        // Properties with "combat" context plus those with "default" context
        let combat_properties = tag.get_properties_in_context("combat");
        assert_eq!(combat_properties.len(), 3);  // All 3 properties apply in their respective contexts or default
        
        // Check the property names
        let prop_names: Vec<String> = combat_properties.iter()
            .filter_map(|p| {
                if let PropertyValue::Stat(name, _) = &p.value {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();
        
        assert!(prop_names.contains(&"damage".to_string()));
        assert!(prop_names.contains(&"resistance".to_string()));
        assert!(prop_names.contains(&"speed".to_string())); // This is visible because of default context
        
        // Properties with "movement" context plus those with "default" context
        let movement_properties = tag.get_properties_in_context("movement");
        assert_eq!(movement_properties.len(), 3);  // All 3 properties apply
    }

    #[test]
    fn test_tag_collection() {
        let mut collection = TagCollection::new();
        
        // Add tags
        let fire_id = collection.add_tag("fire");
        let ice_id = collection.add_tag("ice");
        let poison_id = collection.add_tag("poison");
        
        // Check IDs are sequential
        assert_eq!(fire_id, 1);
        assert_eq!(ice_id, 2);
        assert_eq!(poison_id, 3);
        
        // Check tag count
        assert_eq!(collection.tags.len(), 3);
        
        // Get tags
        let fire_tag = collection.get_tag(fire_id).unwrap();
        assert_eq!(fire_tag.name, "fire");
        
        let ice_tag = collection.get_tag(ice_id).unwrap();
        assert_eq!(ice_tag.name, "ice");
        
        // Get non-existent tag should return None
        assert!(collection.get_tag(99).is_none());
    }

    #[test]
    fn test_tag_collection_modify_tags() {
        let mut collection = TagCollection::new();
        
        // Add a tag
        let tag_id = collection.add_tag("test");
        
        // Clone and modify the tag
        if let Some(tag) = collection.get_tag(tag_id) {
            let modified_tag = tag.clone().with_metadata("key", "value");
            // Replace the original tag with the modified version
            if let Some(mutable_tag) = collection.get_tag_mut(tag_id) {
                *mutable_tag = modified_tag;
            }
        }
        
        // Check modification was applied
        let tag = collection.get_tag(tag_id).unwrap();
        assert_eq!(tag.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_tag_collection_get_tags_in_context() {
        let mut collection = TagCollection::new();
        
        // Add tags with properties
        let fire_id = collection.add_tag("fire");
        if let Some(tag) = collection.get_tag_mut(fire_id) {
            let damage_property = Property::stat_modifier("damage", Stats_StatValue::Integer(5))
                .with_context("combat");
            *tag = tag.clone().with_property(damage_property);
        }
        
        let ice_id = collection.add_tag("ice");
        if let Some(tag) = collection.get_tag_mut(ice_id) {
            let slow_property = Property::stat_modifier("slow", Stats_StatValue::Float(0.5))
                .with_context("combat");
            *tag = tag.clone().with_property(slow_property);
        }
        
        let stone_id = collection.add_tag("stone");
        if let Some(tag) = collection.get_tag_mut(stone_id) {
            let weight_property = Property::stat_modifier("weight", Stats_StatValue::Integer(10))
                .with_context("physics");
            *tag = tag.clone().with_property(weight_property);
        }
        
        // Get tags with combat context - will include all tags due to "default" context
        let combat_tags = collection.get_tags_in_context("combat");
        assert_eq!(combat_tags.len(), 3);  // All tags appear due to default context
        assert!(combat_tags.iter().any(|t| t.name == "fire"));
        assert!(combat_tags.iter().any(|t| t.name == "ice"));
        assert!(combat_tags.iter().any(|t| t.name == "stone"));
        
        // Get tags with physics context - will include all tags due to "default" context
        let physics_tags = collection.get_tags_in_context("physics");
        assert_eq!(physics_tags.len(), 3);
        assert!(physics_tags.iter().any(|t| t.name == "fire"));
        assert!(physics_tags.iter().any(|t| t.name == "ice"));
        assert!(physics_tags.iter().any(|t| t.name == "stone"));
    }
} 