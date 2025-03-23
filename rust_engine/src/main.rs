mod character;
mod npc;
mod entity_type;
mod stats;
mod inventory;
mod calculated_stats;
mod property;
mod tag;
mod utils;
mod coordinates;
mod demos;
mod game_state;
mod files;

use std::time::{Instant, Duration};
use std::thread;
use std::io::{self, BufRead};
use game_state::GameState;

fn main() {
    println!("Starting game backend service...");
    println!("Running asset management demo to test functionality:");
    
    // Run the asset management demo to test it
    demos::demo_asset_management();
    
    // Create game state
    let mut game_state = GameState::new();
    let target_fps = 10; // Update at 10 frames per second
    let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);
    
    println!("\nGame service started! Type 'help' for available commands.");
    
    // Create a separate thread for input handling
    let (tx, rx) = std::sync::mpsc::channel();
    
    std::thread::spawn(move || {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut buffer = String::new();
        
        while let Ok(_) = handle.read_line(&mut buffer) {
            // Send the command to the main thread
            let command = buffer.trim().to_string();
            if command.is_empty() {
                buffer.clear();
                continue;
            }
            
            if tx.send(command).is_err() {
                break; // Main thread has terminated
            }
            
            buffer.clear();
        }
    });
    
    // Main game loop
    while game_state.running {
        let frame_start = Instant::now();
        
        // Process any pending commands from the input thread
        while let Ok(command) = rx.try_recv() {
            // Handle demo commands
            if command == "demo" {
                println!("Running demo_game_state()...");
                demos::demo_game_state();
                continue;
            } else if command == "demo_tags" {
                println!("Running demo_tag_system()...");
                demos::demo_tag_system();
                continue;
            } else if command == "demo_mechanics" {
                println!("Running showcase_different_game_mechanics()...");
                demos::showcase_different_game_mechanics();
                continue;
            }
            
            // Process regular commands
            let response = game_state.process_command(&command);
            println!("{}", response);
        }
        
        // Update game state
        game_state.update(frame_duration.as_secs_f32());
        
        // Sleep to maintain target FPS
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }
    
    println!("Game backend service shut down.");
}
