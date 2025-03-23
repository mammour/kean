use std::collections::{HashMap, HashSet};
use crate::property::Property;
use serde::{Serialize, Deserialize};

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
} 