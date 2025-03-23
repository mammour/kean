use crate::entity_type::EntityType;
use crate::tag::TagCollection;
use crate::property::{PropertyType, PropertyValue};
use crate::stats::StatValue;

/// Get a formatted string representation of an entity with its tags
pub fn format_entity_with_tags(entity: &EntityType, tag_collection: &TagCollection) -> String {
    let mut result = String::new();
    
    result.push_str(&format!("{} ({})\n", entity.name, entity.id));
    result.push_str(&format!("Description: {}\n", entity.description.as_ref().unwrap_or(&"None".to_string())));
    result.push_str(&format!("Category: {}\n", entity.category.as_ref().unwrap_or(&"None".to_string())));
    
    result.push_str("Tags:\n");
    for tag in entity.get_tags(tag_collection) {
        result.push_str(&format!("- {} (ID: {})\n", tag.name, tag.id));
        if let Some(element) = tag.metadata.get("element") {
            result.push_str(&format!("  * Element: {}\n", element));
        }
        if let Some(color) = tag.metadata.get("color") {
            result.push_str(&format!("  * Color: {}\n", color));
        }
    }
    
    result.push_str("Properties:\n");
    for property in &entity.properties {
        if let PropertyType::Custom(key) = &property.property_type {
            if let PropertyValue::Text(value) = &property.value {
                result.push_str(&format!("- {}: {}\n", key, value));
            }
        }
    }
    
    result
}

/// Calculate damage with modifiers
pub fn calculate_damage(base_damage: i32, modifiers: &[f32]) -> i32 {
    let mut final_damage = base_damage as f32;
    
    // Apply all modifiers
    for modifier in modifiers {
        final_damage *= modifier;
    }
    
    final_damage as i32
}

/// Check if an entity has any tag from a list of tags
pub fn entity_has_any_tag(entity: &EntityType, tag_ids: &[i32]) -> bool {
    for tag_id in tag_ids {
        if entity.has_tag_id(*tag_id) {
            return true;
        }
    }
    false
}

/// Find all entities that have specific tag
pub fn find_entities_with_tag<'a>(entities: &[&'a EntityType], tag_id: i32) -> Vec<&'a EntityType> {
    entities.iter()
        .filter(|entity| entity.has_tag_id(tag_id))
        .copied()
        .collect()
}

/// Find all entities with a property matching a specific value
pub fn find_entities_with_property<'a>(entities: &[&'a EntityType], key: &str, value: &str) -> Vec<&'a EntityType> {
    entities.iter()
        .filter(|entity| {
            for property in &entity.properties {
                if let PropertyType::Custom(property_key) = &property.property_type {
                    if property_key == key {
                        if let PropertyValue::Text(property_value) = &property.value {
                            if property_value == value {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        })
        .copied()
        .collect()
}

/// Calculate distance between two points
pub fn calculate_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// Find entities within a certain radius
pub fn find_entities_in_radius<'a, T>(
    entities: &[&'a T],
    center_x: f32, 
    center_y: f32, 
    radius: f32,
    position_getter: fn(&T) -> (f32, f32)
) -> Vec<&'a T> {
    entities.iter()
        .filter(|entity| {
            let (entity_x, entity_y) = position_getter(entity);
            calculate_distance(center_x, center_y, entity_x, entity_y) <= radius
        })
        .copied()
        .collect()
}

/// Check if a line of sight exists between two points (no obstacles)
pub fn has_line_of_sight(
    start_x: f32, 
    start_y: f32, 
    end_x: f32, 
    end_y: f32, 
    obstacles: &[(f32, f32, f32)]  // x, y, radius
) -> bool {
    // Create a line from start to end
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let line_length = (dx.powi(2) + dy.powi(2)).sqrt();
    
    // Normalize direction
    let direction_x = dx / line_length;
    let direction_y = dy / line_length;
    
    // Check if any obstacle intersects the line
    for &(obs_x, obs_y, obs_radius) in obstacles {
        // Find closest point on line to obstacle
        let t = direction_x * (obs_x - start_x) + direction_y * (obs_y - start_y);
        
        // If closest point is outside line segment, continue
        if t < 0.0 || t > line_length {
            continue;
        }
        
        // Calculate closest point on line
        let closest_x = start_x + t * direction_x;
        let closest_y = start_y + t * direction_y;
        
        // Check if obstacle blocks line of sight
        let distance = calculate_distance(closest_x, closest_y, obs_x, obs_y);
        if distance <= obs_radius {
            return false;
        }
    }
    
    true
} 