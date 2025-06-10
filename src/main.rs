
// src/main.rs

mod setup;
use std::collections::HashMap;

fn main() {
    let mut world = setup::initialize_world(
        "data/config/goods.yaml",
        "data/config/planets.yaml",
    );
    
    // Calculate initial planet positions
    world.initialize_positions();
    
    // Now you have a fully initialized game world
    // Proceed with game loop...
}
}
