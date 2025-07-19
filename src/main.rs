mod assets;
mod simulation; // Added line for the simulation module
mod setup;

fn main() {
    let mut world = setup::initialize_world(
        "data/config/goods.yaml",
        "data/config/planets.yaml",
    );
    
    // Calculate initial planet positions
    world.initialize_positions();
    
    // Now you has a fully initialized game world
    // Proceed with game loop...
}
