use crate::character::Character;
use crate::npc::NPC;
use crate::entity_type::EntityType;
use crate::stats::StatValue;
use crate::calculated_stats::{StatModifier, ModifierType};
use crate::property::{Property, PropertyValue};
use crate::tag::TagCollection;
use crate::coordinates::Coordinates;
use crate::utils;
use super::files::{AssetManager, AssetType, Asset};

pub fn demo_tag_system() {
    println!("\n=== TAG SYSTEM DEMO ===\n");
    
    // Create a tag collection
    let mut tag_collection = TagCollection::new();
    
    // Create some tags with properties
    let fire_id = tag_collection.add_tag("fire");
    if let Some(fire_tag) = tag_collection.get_tag_mut(fire_id) {
        *fire_tag = fire_tag.clone()
            .with_property(Property::stat_modifier("damage", StatValue::Integer(5))
                .with_context("combat"))
            .with_property(Property::stat_modifier("fire_resistance", StatValue::Float(0.5))
                .with_context("defense"))
            .with_metadata("element", "fire")
            .with_metadata("color", "#FF4500");
    }
    
    let ice_id = tag_collection.add_tag("ice");
    if let Some(ice_tag) = tag_collection.get_tag_mut(ice_id) {
        *ice_tag = ice_tag.clone()
            .with_property(Property::stat_modifier("speed", StatValue::Float(0.7))
                .with_context("movement"))
            .with_property(Property::stat_modifier("ice_resistance", StatValue::Float(0.5))
                .with_context("defense"))
            .with_metadata("element", "ice")
            .with_metadata("color", "#ADD8E6");
    }
    
    let poison_id = tag_collection.add_tag("poison");
    if let Some(poison_tag) = tag_collection.get_tag_mut(poison_id) {
        *poison_tag = poison_tag.clone()
            .with_property(Property::stat_modifier("health", StatValue::Integer(-2))
                .with_context("over_time"))
            .with_metadata("element", "nature")
            .with_metadata("color", "#008000");
    }
    
    let ancient_id = tag_collection.add_tag("ancient");
    if let Some(ancient_tag) = tag_collection.get_tag_mut(ancient_id) {
        *ancient_tag = ancient_tag.clone()
            .with_property(Property::stat_modifier("magic_resistance", StatValue::Float(0.3))
                .with_context("defense"))
            .with_property(Property::stat_modifier("value", StatValue::Float(1.5))
                .with_context("economy"))
            .with_metadata("rarity", "uncommon");
    }
    
    // Create entity types with tags
    let fire_sword = EntityType::new("fire_sword", "Flaming Sword")
        .with_description("A sword engulfed in flames")
        .with_category("weapon")
        .with_tag_id(fire_id)
        .with_tag_id(ancient_id)
        .with_property("damage_type", "slashing")
        .with_property("two_handed", "false");
        
    let ice_staff = EntityType::new("ice_staff", "Frost Staff")
        .with_description("A staff that shoots ice projectiles")
        .with_category("weapon")
        .with_tag_id(ice_id)
        .with_property("damage_type", "magic")
        .with_property("two_handed", "true");
        
    let poison_dagger = EntityType::new("poison_dagger", "Venomous Dagger")
        .with_description("A dagger coated with poison")
        .with_category("weapon")
        .with_tag_id(poison_id)
        .with_property("damage_type", "piercing")
        .with_property("two_handed", "false");
        
    // Show entity types and their tags
    println!("Created entity types with tags:");
    print!("{}", utils::format_entity_with_tags(&fire_sword, &tag_collection));
    print!("{}", utils::format_entity_with_tags(&ice_staff, &tag_collection));
    print!("{}", utils::format_entity_with_tags(&poison_dagger, &tag_collection));
    
    // Demo tag searches
    println!("\nEntity types with 'fire' tag:");
    let entity_types = vec![&fire_sword, &ice_staff, &poison_dagger];
    for entity in entity_types.iter() {
        if entity.has_tag_id(fire_id) {
            println!("- {}", entity.name);
        }
    }
    
    // Demo tag properties in context
    println!("\nCombat properties of Flaming Sword:");
    let combat_properties = fire_sword.get_tag_properties_in_context(&tag_collection, "combat");
    for prop in combat_properties {
        if let PropertyValue::Stat(stat_name, StatValue::Integer(value)) = &prop.value {
            println!("- +{} {}", value, stat_name);
        }
    }
    
    // Search for all entities with specific tag property
    println!("\nAll weapons that affect movement:");
    for entity in entity_types.iter() {
        let movement_properties = entity.get_tag_properties_in_context(&tag_collection, "movement");
        if !movement_properties.is_empty() {
            println!("- {}", entity.name);
            for prop in movement_properties {
                if let PropertyValue::Stat(stat_name, StatValue::Float(value)) = &prop.value {
                    println!("  * {} modifier: {:.1}x", stat_name, value);
                }
            }
        }
    }
    
    // Demo finding all tags with a specific property type
    println!("\nAll tags that provide defensive properties:");
    let defensive_tags = tag_collection.get_tags_in_context("defense");
    for tag in defensive_tags {
        println!("- {} (ID: {})", tag.name, tag.id);
        for prop in tag.get_properties_in_context("defense") {
            if let PropertyValue::Stat(stat_name, value) = &prop.value {
                match value {
                    StatValue::Float(v) => println!("  * {} modifier: {:.1}x", stat_name, v),
                    StatValue::Integer(v) => println!("  * {} modifier: {:+}", stat_name, v),
                    _ => {}
                }
            }
        }
    }
}

// Demonstrate how the engine can be used for wildly different game mechanics
pub fn showcase_different_game_mechanics() {
    println!("\n=== GAME MECHANICS DEMO ===\n");
    
    // Create entity types for different NPCs
    let combat_type = EntityType::new("goblin", "Goblin")
        .with_category("hostile")
        .with_property("combat_style", "melee")
        .with_property("monster", "true");
    
    let fan_type = EntityType::new("superfan", "Superfan")
        .with_category("neutral")
        .with_description("An enthusiastic fan who can be won over with attention")
        .with_property("audience", "true");

    // Traditional combat game mechanics
    let mut combat_npc = NPC::new("Gobby the Goblin".to_string(), combat_type.clone());
    combat_npc.base_stats_mut().set("health", StatValue::Integer(50));
    combat_npc.base_stats_mut().set("damage", StatValue::Integer(5));
    
    println!("Created a {} with {} health", combat_npc.npc_type.name, combat_npc.get_int_stat("health").unwrap_or(0));
    
    // Attack the NPC
    let damage = 20;
    println!("Player attacks {} for {} damage", combat_npc.npc_type.name, damage);
    if let Some(current_health) = combat_npc.get_int_stat("health") {
        combat_npc.base_stats_mut().set("health", StatValue::Integer(current_health - damage));
    }
    
    println!("{} now has {} health", combat_npc.npc_type.name, combat_npc.get_int_stat("health").unwrap_or(0));
    
    // Add a status effect
    let _poison_effect = StatModifier {
        source: "poison".to_string(), 
        modifier_type: ModifierType::Additive,
        value: StatValue::Integer(-2),
        priority: 10
    };
    combat_npc.add_status_effect("poisoned");
    combat_npc.add_stat_modifier("health", "poison", ModifierType::Additive, StatValue::Integer(-2), 10);
    println!("Applied poison to {}, dealing 2 damage per turn", combat_npc.npc_type.name);
    
    // Fan adoration game mechanics using the same NPC system
    let mut fan_npc = NPC::new("Sammy the Superfan".to_string(), fan_type.clone());
    fan_npc.base_stats_mut().set("adoration", StatValue::Integer(0));
    fan_npc.base_stats_mut().set("loyalty", StatValue::Float(0.1));
    
    println!("\nCreated a {} with adoration level {}", fan_npc.npc_type.name, fan_npc.get_int_stat("adoration").unwrap_or(0));
    
    // Player interacts with fan
    let adoration_gain = 15;
    println!("Player interacts with {}, gaining {} adoration", fan_npc.npc_type.name, adoration_gain);
    if let Some(current_adoration) = fan_npc.get_int_stat("adoration") {
        fan_npc.base_stats_mut().set("adoration", StatValue::Integer(current_adoration + adoration_gain));
    }
    
    if let Some(current_loyalty) = fan_npc.get_float_stat("loyalty") {
        fan_npc.base_stats_mut().set("loyalty", StatValue::Float(current_loyalty + 0.1));
    }
    
    println!("{} now has {} adoration and {:.1} loyalty", 
        fan_npc.npc_type.name, 
        fan_npc.get_int_stat("adoration").unwrap_or(0),
        fan_npc.get_float_stat("loyalty").unwrap_or(0.0));
    
    // Add a buff
    fan_npc.add_stat_modifier("adoration", "excitement", ModifierType::Multiplicative, StatValue::Float(1.5), 5);
    println!("Fan feels excited, increasing adoration gain by 50%");
    
    // Demonstrate shared mechanics - both NPCs can move
    combat_npc.set_position(10.0, 15.0);
    println!("\n{} is at position {}", combat_npc.npc_type.name, combat_npc.position);
    
    fan_npc.set_position(5.0, 8.0);
    println!("{} is at position {}", fan_npc.npc_type.name, fan_npc.position);
    
    // Move both NPCs
    let target_position = Coordinates::new_2d(12.0, 14.0);
    combat_npc.move_toward(&target_position, 1.0);
    
    fan_npc.position.set(0, fan_npc.x() + 1.0);
    fan_npc.position.set(1, fan_npc.y() + 1.0);
    
    println!("\nAfter movement:");
    println!("{} is now at position {}", combat_npc.npc_type.name, combat_npc.position);
    println!("{} is now at position {}", fan_npc.npc_type.name, fan_npc.position);
    
    // Demonstrate different dimensions
    println!("\n=== DIFFERENT DIMENSION DEMOS ===\n");
    
    // 1D timeline character
    let mut timeline_character = Character::new_1d(0.0);
    println!("Timeline character starts at {}", timeline_character.position);
    
    // Advance the timeline character
    timeline_character.position.set(0, 10.0);
    println!("Timeline character advances to {}", timeline_character.position);
    
    // 3D character
    let space_character = Character::new_3d(1.0, 2.0, 3.0);
    println!("3D character starts at {}", space_character.position);
    
    // 4D character (3D + time)
    let space_time_character = Character::new_4d(1.0, 2.0, 3.0, 0.0);
    println!("4D character starts at {}", space_time_character.position);
    
    // Custom dimension character
    let mut custom_dims = Character::with_dimensions(5);
    custom_dims.position = custom_dims.position.with_labels(vec!["physical_x", "physical_y", "mental_x", "mental_y", "ethical"]);
    println!("Custom dimensional character exists in {}", custom_dims.position);
}

// Add a new demo function for GameState at the end of the file
pub fn demo_game_state() {
    println!("\n=== GAME STATE DEMO ===\n");
    
    // Create a new game state
    let mut game_state = crate::game_state::GameState::new();
    
    // Register some basic entity types
    let goblin_type = EntityType::new("goblin", "Goblin")
        .with_category("hostile")
        .with_property("combat_style", "melee")
        .with_property("monster", "true");
    
    game_state.entity_types.insert("goblin".to_string(), goblin_type.clone());
    
    // Add some NPCs
    let mut goblin = NPC::new("Goblin Guard".to_string(), goblin_type);
    goblin.base_stats_mut().set("health", crate::stats::StatValue::Integer(50));
    goblin.base_stats_mut().set("damage", crate::stats::StatValue::Integer(5));
    goblin.position.set(0, 10.0);
    goblin.position.set(1, 15.0);
    game_state.npcs.push(goblin);
    
    println!("Added 1 NPC to the game state");
    
    // Set player position
    game_state.player.position.set(0, 5.0);
    game_state.player.position.set(1, 5.0);
    
    // Update game state a few times
    for i in 0..5 {
        println!("\nUpdate {}:", i+1);
        
        // Example: Make NPCs wander randomly
        for npc in &mut game_state.npcs {
            let x_move = ((i as f32) * 0.1).sin() * 0.5;
            let y_move = ((i as f32) * 0.1).cos() * 0.5;
            
            npc.position.set(0, npc.x() + x_move);
            npc.position.set(1, npc.y() + y_move);
            
            println!("  {} moved to {}", npc.npc_type.name, npc.position);
        }
        
        game_state.update(0.1);
    }
    
    // Try some commands
    let commands = vec!["status", "move 10 15", "status"];
    
    for cmd in commands {
        println!("\n> {}", cmd);
        let response = game_state.process_command(cmd);
        println!("{}", response);
    }
}

// Add a new demo function for asset management at the end of the file
pub fn demo_asset_management() {
    println!("\n=== ASSET MANAGEMENT DEMO ===\n");
    
    // Create a new asset manager
    let mut asset_manager = AssetManager::new();
    println!("Created AssetManager with default paths:");
    println!("- Images: assets/images");
    println!("- Sounds: assets/sounds");
    println!("- Videos: assets/videos");
    
    // Showcase path handling
    println!("\nDemonstrating path handling:");
    let image_path = asset_manager.get_asset_path(AssetType::Image, "player/character.png");
    let sound_path = asset_manager.get_asset_path(AssetType::Sound, "effects/explosion.wav");
    let video_path = asset_manager.get_asset_path(AssetType::Video, "cutscenes/intro.mp4");
    
    println!("- Image path: {:?}", image_path);
    println!("- Sound path: {:?}", sound_path);
    println!("- Video path: {:?}", video_path);
    
    // Demonstrate asset loading simulation
    println!("\nSimulating asset loading (note: no actual files loaded):");
    println!("(In a real game, you would load actual files from disk)");
    
    // Create a simulated asset for demonstration
    let test_data = vec![1, 2, 3, 4, 5]; // Simulated binary data
    let test_asset = Asset::new(
        AssetType::Image, 
        "assets/images/test.png", 
        "test_image".to_string(), 
        test_data
    ).with_metadata("width", "128")
     .with_metadata("height", "128")
     .with_metadata("format", "png");
    
    // Add the asset manually to the manager for demonstration
    asset_manager.assets.insert("test_image".to_string(), test_asset);
    
    // Show asset access
    println!("\nAccessing loaded asset:");
    if let Some(asset) = asset_manager.get_asset("test_image") {
        println!("- Name: {}", asset.name);
        println!("- Type: {:?}", asset.asset_type);
        println!("- Path: {:?}", asset.path);
        println!("- Data size: {} bytes", asset.data.len());
        println!("- Metadata:");
        for (key, value) in &asset.metadata {
            println!("  * {}: {}", key, value);
        }
    }
    
    // Demonstrate asset duplication and transformation
    println!("\nDuplicating and transforming assets:");
    match asset_manager.duplicate_asset("test_image", "transformed_image") {
        Ok(_) => {
            // Apply a transformation to the duplicated asset
            if let Ok(_) = asset_manager.transform_asset("transformed_image", |asset| {
                // Simulated transformation - just change metadata
                asset.metadata.insert("width".to_string(), "256".to_string());
                asset.metadata.insert("height".to_string(), "256".to_string());
                asset.metadata.insert("transformed".to_string(), "true".to_string());
                // In a real implementation, we would modify the actual asset data
            }) {
                if let Some(asset) = asset_manager.get_asset("transformed_image") {
                    println!("- Name: {}", asset.name);
                    println!("- Metadata after transformation:");
                    for (key, value) in &asset.metadata {
                        println!("  * {}: {}", key, value);
                    }
                }
            }
        },
        Err(e) => println!("Error duplicating asset: {:?}", e),
    }
    
    // Demonstrate asset organization by type
    println!("\nAsset organization by type:");
    println!("- Images would be stored in: assets/images/");
    println!("- Sounds would be stored in: assets/sounds/");
    println!("- Videos would be stored in: assets/videos/");
    
    // Show examples of how to use assets in games
    println!("\nExample usage in games:");
    println!("1. Load game sprites: asset_manager.load_asset(AssetType::Image, \"player.png\", None)");
    println!("2. Play sound effect: audio_system.play(asset_manager.get_asset(\"explosion.wav\"))");
    println!("3. Stream video: video_player.play(asset_manager.get_asset(\"intro.mp4\"))");
    
    // Example for the updated load_directory function
    println!("\nLoading assets from a directory:");
    println!("let asset_names = asset_manager.load_directory(AssetType::Sound, \"effects\")");
    println!("// Would return a Vec<String> with all asset names loaded");
    
    // Cleanup
    println!("\nClearing all assets...");
    asset_manager.clear();
    println!("Asset manager now contains {} assets", 
             asset_manager.assets.len());
             
    println!("\nAsset management system ready for your game development!");
} 