# Kean Game Engine

A flexible Rust-based game engine designed for implementing diverse game mechanics and coordinate systems.

## Overview

Kean is a game engine with a focus on flexibility and reusability. It provides core systems for:

- Entity management with tags and properties
- Flexible multi-dimensional coordinate system
- Stats and modifiers for various game mechanics
- Inventory and item systems
- NPC behavior
- Game state management

## Flexible Coordinate System

The engine features a powerful coordinate system that supports:

- Any number of dimensions (1D, 2D, 3D, 4D, or custom)
- Named dimensions with labels (x, y, z, time, etc.)
- Vector operations (addition, subtraction, scalar multiplication)
- Distance calculations
- Movement toward targets

### Example Usage

```rust
// Create characters in different dimensions
let timeline_character = Character::new_1d(0.0);
let regular_character = Character::new_2d(10.0, 15.0);
let space_character = Character::new_3d(1.0, 2.0, 3.0);
let space_time_character = Character::new_4d(1.0, 2.0, 3.0, 0.0);

// Custom dimensions with labels
let mut custom_dims = Character::with_dimensions(5);
custom_dims.position = custom_dims.position.with_labels(
    vec!["physical_x", "physical_y", "mental_x", "mental_y", "ethical"]
);

// Access coordinates by index or label
let x_pos = character.get_position(0).unwrap_or(0.0);
let y_pos = character.get_position_by_label("y").unwrap_or(0.0);

// Calculate distance between entities
let distance = character.distance_to(&other_character);

// Move toward a target
let target = Coordinates::new_2d(20.0, 30.0);
character.move_toward(&target, 5.0);
```

## Game State Management

The `GameState` struct provides a central hub for managing the game world:

```rust
// Create a new game state
let mut game_state = GameState::new();

// Add entities and NPCs to the game
let goblin_type = EntityType::new("goblin", "Goblin")
    .with_category("hostile")
    .with_property("combat_style", "melee");
    
game_state.entity_types.insert("goblin".to_string(), goblin_type.clone());

// Create and add an NPC
let mut goblin = NPC::new("Goblin Guard".to_string(), goblin_type);
goblin.base_stats_mut().set("health", StatValue::Integer(50));
goblin.position.set(0, 10.0);
goblin.position.set(1, 15.0);
game_state.npcs.push(goblin);

// Update the game state (typically called in game loop)
game_state.update(delta_time);

// Process commands
let response = game_state.process_command("move 10 15");
println!("{}", response);
```

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

## Demo Examples

The engine includes several demo functions to showcase its capabilities:

- `demo_tag_system()`: Demonstrates the tag system with entities and properties
- `showcase_different_game_mechanics()`: Shows how the engine can support various game types
- `demo_game_state()`: Highlights the GameState features and command processing

Run these demos with:

```rust
// In main.rs or via the command system
demos::demo_tag_system();
demos::showcase_different_game_mechanics();
demos::demo_game_state();
```

## Project Structure

```
src/
├── calculated_stats.rs - Stats calculation with modifiers
├── character.rs - Player character implementation
├── coordinates.rs - Flexible coordinate system
├── demos.rs - Demo functions showcasing features
├── entity_type.rs - Entity type definitions with tags
├── game_state.rs - Central game state management
├── inventory.rs - Inventory and item systems
├── lib.rs - Public exports and module organization
├── main.rs - Command processing and game loop
├── npc.rs - Non-player character implementation
├── property.rs - Property system for entities
├── stats.rs - Base stats system
├── tag.rs - Tag system for categorization
└── utils.rs - Utility functions
```

## Getting Started

1. Clone the repository
2. Run with `cargo run`
3. Try commands:
   - `help` - Show available commands
   - `demo` - Run the game state demo
   - `demo_tags` - Run the tag system demo
   - `demo_mechanics` - Run the game mechanics demo
   - `status` - Show current game state
   - `move <x> <y>` - Move the player
   - `exit` - Quit the application

## Future Development

- Serialization/deserialization improvements
- Event system integration
- Client-server architecture
- More demo scenarios
- Visual editor 