pub mod stats;
pub mod character;
pub mod inventory;
pub mod npc;
pub mod entity_type;
pub mod calculated_stats;
pub mod property;
pub mod tag;
pub mod utils;
pub mod coordinates;
pub mod demos;
pub mod game_state;
pub mod files;

// Re-export commonly used structures
pub use stats::{Stats, StatValue};
pub use character::Character;
pub use inventory::{Inventory, Item};
pub use npc::NPC;
pub use entity_type::EntityType;
pub use calculated_stats::{CalculatedStats, StatModifier, ModifierType};
pub use property::{Property, PropertyType, PropertyValue, Condition, ConditionType};
pub use tag::{Tag, TagCollection};
pub use coordinates::Coordinates;
pub use demos::{demo_tag_system, showcase_different_game_mechanics, demo_game_state, demo_asset_management};
pub use game_state::GameState;
pub use utils::{
    format_entity_with_tags, 
    calculate_damage, 
    entity_has_any_tag, 
    find_entities_with_tag, 
    find_entities_with_property,
    calculate_distance, 
    find_entities_in_radius, 
    has_line_of_sight
}; 
pub use files::{Asset, AssetManager, AssetType, AssetResult, AssetError, transform_copy}; 