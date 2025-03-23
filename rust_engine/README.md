# Kean Game Engine

A flexible Rust-based game engine designed for easily implementing different game mechanics.

## Overview

Kean is a game engine with a focus on flexibility and reusability. It provides core systems for:

- Entity management with tags and properties
- Stats and modifiers for various game mechanics
- Inventory and item systems
- NPC behavior

## Tag System

The tag system is a powerful way to categorize entities and apply properties based on tags. This allows for searching, filtering, and applying effects to entities in a flexible manner.

### Key Components

#### Property

Properties are the fundamental building blocks that define behaviors, stats modifications, and abilities:

```rust
// Create a stat modifier property
let fire_damage = Property::stat_modifier("damage", StatValue::Integer(5))
    .with_context("combat");

// Create an ability property
let teleport = Property::ability("teleport")
    .with_context("movement");
```

Properties can have:
- A specific context (combat, movement, etc.)
- Conditions that determine when they apply
- Metadata for additional information

#### Tag

Tags combine a name, ID, and a set of properties:

```rust
// Create a tag with properties
let fire_tag = Tag::new(1, "fire")
    .with_property(Property::stat_modifier("damage", StatValue::Integer(5))
        .with_context("combat"))
    .with_property(Property::stat_modifier("fire_resistance", StatValue::Float(0.5))
        .with_context("defense"))
    .with_metadata("element", "fire");
```

#### TagCollection

TagCollection manages all tags in the game:

```rust
// Create a tag collection
let mut tag_collection = TagCollection::new();

// Add a tag and get its ID
let fire_id = tag_collection.add_tag("fire");

// Get a tag by ID
if let Some(fire_tag) = tag_collection.get_tag_mut(fire_id) {
    // Modify the tag
}

// Find tags for a specific context
let combat_tags = tag_collection.get_tags_in_context("combat");
```

#### EntityType with Tags and Properties

EntityType now supports tags and stores properties as a collection of Property objects:

```rust
// Create an entity type with tags
let fire_sword = EntityType::new("fire_sword", "Fire Sword")
    .with_description("A sword engulfed in flames")
    .with_category("weapon")
    .with_tag_id(fire_id)
    .with_tag_id(ancient_id)
    .with_property("damage_type", "slashing");

// Add a property object directly
let special_property = Property::stat_modifier("critical_chance", StatValue::Float(0.15))
    .with_context("combat");
let fire_sword = fire_sword.with_property_object(special_property);

// Check if an entity has a specific tag
if entity.has_tag_id(fire_id) {
    // Do something with fire entities
}

// Get properties from tags in a specific context
let combat_properties = entity.get_tag_properties_in_context(&tag_collection, "combat");
for prop in combat_properties {
    // Apply combat effects
}

// Get entity's own properties in a context
let movement_properties = entity.get_properties_in_context("movement");
```

## Property Storage

All components in the engine that use properties now store them as `Vec<Property>` instead of simple key-value pairs. This provides several advantages:

1. **Rich Property Definition**: Properties can have types, contexts, conditions, and metadata.
2. **Contextual Application**: Properties can be applied only in specific contexts.
3. **Conditional Effects**: Properties can have conditions that determine when they apply.
4. **Flexible Value Types**: Properties can store different types of values, not just strings.
5. **Metadata**: Additional information can be attached to properties.

## Example Usage

See the `demo_tag_system()` function in `main.rs` for a complete demonstration of the tag system. The demo shows:

1. Creating tags with properties
2. Applying tags to entity types
3. Searching for entities with specific tags
4. Getting properties in different contexts
5. Filtering tags based on properties

## Benefits of the Tag System

- **Flexibility**: Define properties that can apply in different contexts or conditions
- **Modularity**: Tags can be reused across different entity types
- **Searchability**: Easily find entities with specific tags or properties
- **Game Design**: Makes it easy to implement complex game mechanics with simple building blocks
- **Rich Properties**: Properties are full objects with contexts, conditions, and metadata

## Future Development

- Condition-based property application
- Event system that integrates with the tag system
- Serialization/deserialization of tags and properties
- Visual editor for creating and managing tags 