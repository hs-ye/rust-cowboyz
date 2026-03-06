//! Rust Cowboyz - Space-Western Trading Game Library
//!
//! This library provides the core game logic that can be used by both
//! the CLI and web interfaces.

pub mod assets;
pub mod game_state;
pub mod player;
pub mod setup;
pub mod simulation;

// UI module - only available with web build
#[cfg(feature = "web")]
pub mod ui;

// Re-export commonly used types
pub use game_state::GameState;
pub use player::Player;
pub use setup::World;

/// Initialize a new game world
pub fn create_world() -> World {
    setup::initialize_world("data/config/goods.yaml", "data/config/planets.yaml")
}