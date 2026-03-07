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
pub use game_state::{GameState, ValidationResult, validate_game_state};
pub use assets::save_game::{SaveLoadError, LOCAL_STORAGE_KEY};
pub use player::Player;
pub use setup::World;

/// Initialize a new game world
pub fn create_world() -> World {
    setup::initialize_world("data/config/goods.yaml", "data/config/planets.yaml")
}

// Web entry point - mounts the Leptos app when WASM is loaded
#[cfg(feature = "web")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    use leptos::*;
    use wasm_bindgen::prelude::*;

    // Set up panic hook for better error reporting in browser console
    console_error_panic_hook::set_once();

    // Mount the application to the body
    leptos::mount_to_body(|| view! { <ui::web::App/> });
}