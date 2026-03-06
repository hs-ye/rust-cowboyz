//! Save and load game functionality
//! Based on ADR 0006: Data Persistence Implementation

use crate::game_state::{GameState, ValidationResult, validate_game_state};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Local storage key for browser storage
pub const LOCAL_STORAGE_KEY: &str = "rust_cowboyz_game_state";

/// Error types for save/load operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SaveLoadError {
    IoError(String),
    SerializationError(String),
    ValidationError(Vec<String>),
    VersionMismatch { expected: String, found: String },
    NotFound,
}

impl std::fmt::Display for SaveLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveLoadError::IoError(msg) => write!(f, "IO Error: {}", msg),
            SaveLoadError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            SaveLoadError::ValidationError(errors) => {
                write!(f, "Validation Error: {}", errors.join(", "))
            }
            SaveLoadError::VersionMismatch { expected, found } => write!(
                f,
                "Version Mismatch: expected {}, found {}",
                expected, found
            ),
            SaveLoadError::NotFound => write!(f, "Save file not found"),
        }
    }
}

impl std::error::Error for SaveLoadError {}

/// Save game state to a file
/// 
/// # Arguments
/// * `state` - The game state to save
/// * `path` - The file path to save to
/// 
/// # Returns
/// * `Ok(())` on success
/// * `Err(SaveLoadError)` on failure
pub fn save_game_to_file(state: &GameState, path: &str) -> Result<(), SaveLoadError> {
    // Validate the game state before saving
    let validation = validate_game_state(state);
    if !validation.is_valid {
        return Err(SaveLoadError::ValidationError(validation.errors));
    }

    // Serialize to JSON
    let json = serde_json::to_string_pretty(state)
        .map_err(|e| SaveLoadError::SerializationError(e.to_string()))?;

    // Write to file
    let mut file = fs::File::create(path)
        .map_err(|e| SaveLoadError::IoError(e.to_string()))?;
    
    file.write_all(json.as_bytes())
        .map_err(|e| SaveLoadError::IoError(e.to_string()))?;

    Ok(())
}

/// Load game state from a file
/// 
/// # Arguments
/// * `path` - The file path to load from
/// 
/// # Returns
/// * `Ok(GameState)` on success
/// * `Err(SaveLoadError)` on failure
pub fn load_game_from_file(path: &str) -> Result<GameState, SaveLoadError> {
    // Check if file exists
    if !Path::new(path).exists() {
        return Err(SaveLoadError::NotFound);
    }

    // Read file contents
    let contents = fs::read_to_string(path)
        .map_err(|e| SaveLoadError::IoError(e.to_string()))?;

    // Deserialize from JSON
    let state: GameState = serde_json::from_str(&contents)
        .map_err(|e| SaveLoadError::SerializationError(e.to_string()))?;

    // Validate the loaded game state
    let validation = validate_game_state(&state);
    if !validation.is_valid {
        return Err(SaveLoadError::ValidationError(validation.errors));
    }

    Ok(state)
}

/// Save game state to browser localStorage (for web builds)
/// 
/// Note: This function is designed for use with wasm-bindgen in web builds.
/// In native builds, it will return an error.
#[cfg(target_arch = "wasm32")]
pub fn save_game_to_browser(state: &GameState) -> Result<(), SaveLoadError> {
    use wasm_bindgen::JsValue;
    
    // Validate the game state before saving
    let validation = validate_game_state(state);
    if !validation.is_valid {
        return Err(SaveLoadError::ValidationError(validation.errors));
    }

    // Serialize to JSON
    let json = serde_json::to_string(state)
        .map_err(|e| SaveLoadError::SerializationError(e.to_string()))?;

    // Get the window object and localStorage
    let window = web_sys::window().ok_or_else(|| {
        SaveLoadError::IoError("Could not get window object".to_string())
    })?;
    
    let local_storage = window.local_storage()
        .map_err(|_| SaveLoadError::IoError("Could not get localStorage".to_string()))?
        .ok_or_else(|| SaveLoadError::IoError("localStorage not available".to_string()))?;

    // Save to localStorage
    let key = JsValue::from_str(LOCAL_STORAGE_KEY);
    let value = JsValue::from_str(&json);
    
    local_storage.set(&key, &value)
        .map_err(|_| SaveLoadError::IoError("Failed to save to localStorage".to_string()))?;

    Ok(())
}

/// Load game state from browser localStorage (for web builds)
/// 
/// Note: This function is designed for use with wasm-bindgen in web builds.
/// In native builds, it will return an error.
#[cfg(target_arch = "wasm32")]
pub fn load_game_from_browser() -> Result<GameState, SaveLoadError> {
    use wasm_bindgen::JsValue;
    
    // Get the window object and localStorage
    let window = web_sys::window().ok_or_else(|| {
        SaveLoadError::IoError("Could not get window object".to_string())
    })?;
    
    let local_storage = window.local_storage()
        .map_err(|_| SaveLoadError::IoError("Could not get localStorage".to_string()))?
        .ok_or_else(|| SaveLoadError::IoError("localStorage not available".to_string()))?;

    // Try to get the saved game state
    let key = JsValue::from_str(LOCAL_STORAGE_KEY);
    let value = local_storage.get(&key)
        .map_err(|_| SaveLoadError::IoError("Failed to read from localStorage".to_string()))?;

    match value {
        Some(json_str) => {
            // Deserialize from JSON
            let state: GameState = serde_json::from_str(&json_str)
                .map_err(|e| SaveLoadError::SerializationError(e.to_string()))?;

            // Validate the loaded game state
            let validation = validate_game_state(&state);
            if !validation.is_valid {
                return Err(SaveLoadError::ValidationError(validation.errors));
            }

            Ok(state)
        }
        None => Err(SaveLoadError::NotFound),
    }
}

/// Check if a saved game exists in browser localStorage
#[cfg(target_arch = "wasm32")]
pub fn has_saved_game() -> bool {
    use wasm_bindgen::JsValue;
    
    if let Some(window) = web_sys::window() {
        if let Some(local_storage) = window.local_storage().ok().flatten() {
            let key = JsValue::from_str(LOCAL_STORAGE_KEY);
            return local_storage.get(&key).ok().flatten().is_some();
        }
    }
    false
}

/// Delete saved game from browser localStorage
#[cfg(target_arch = "wasm32")]
pub fn delete_saved_game() -> Result<(), SaveLoadError> {
    use wasm_bindgen::JsValue;
    
    let window = web_sys::window().ok_or_else(|| {
        SaveLoadError::IoError("Could not get window object".to_string())
    })?;
    
    let local_storage = window.local_storage()
        .map_err(|_| SaveLoadError::IoError("Could not get localStorage".to_string()))?
        .ok_or_else(|| SaveLoadError::IoError("localStorage not available".to_string()))?;

    let key = JsValue::from_str(LOCAL_STORAGE_KEY);
    local_storage.delete(&key)
        .map_err(|_| SaveLoadError::IoError("Failed to delete save".to_string()))?;

    Ok(())
}

// Non-web fallback implementations
#[cfg(not(target_arch = "wasm32"))]
pub fn save_game_to_browser(_state: &GameState) -> Result<(), SaveLoadError> {
    Err(SaveLoadError::IoError("Browser storage not available in native builds".to_string()))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_game_from_browser() -> Result<GameState, SaveLoadError> {
    Err(SaveLoadError::IoError("Browser storage not available in native builds".to_string()))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn has_saved_game() -> bool {
    false
}

#[cfg(not(target_arch = "wasm32"))]
pub fn delete_saved_game() -> Result<(), SaveLoadError> {
    Err(SaveLoadError::IoError("Browser storage not available in native builds".to_string()))
}

/// Export game state to JSON string
pub fn export_to_json(state: &GameState) -> Result<String, SaveLoadError> {
    serde_json::to_string_pretty(state)
        .map_err(|e| SaveLoadError::SerializationError(e.to_string()))
}

/// Import game state from JSON string
pub fn import_from_json(json: &str) -> Result<GameState, SaveLoadError> {
    let state: GameState = serde_json::from_str(json)
        .map_err(|e| SaveLoadError::SerializationError(e.to_string()))?;

    let validation = validate_game_state(&state);
    if !validation.is_valid {
        return Err(SaveLoadError::ValidationError(validation.errors));
    }

    Ok(state)
}

/// Get the default save directory path
pub fn get_default_save_path() -> String {
    #[cfg(target_os = "windows")]
    {
        if let Some(app_data) = std::env::var_os("APPDATA") {
            return format!("{}/rust-cowboyz/savegame.json", app_data);
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return format!("{}/Library/Application Support/rust-cowboyz/savegame.json", home);
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return format!("{}/.local/share/rust-cowboyz/savegame.json", home.to_string_lossy());
        }
    }
    
    "savegame.json".to_string()
}

/// Ensure the save directory exists
pub fn ensure_save_directory() -> Result<(), SaveLoadError> {
    let path = get_default_save_path();
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| SaveLoadError::IoError(e.to_string()))?;
    }
    Ok(())
}

/// Quick save to default location
pub fn quick_save(state: &GameState) -> Result<(), SaveLoadError> {
    ensure_save_directory()?;
    save_game_to_file(state, &get_default_save_path())
}

/// Quick load from default location
pub fn quick_load() -> Result<GameState, SaveLoadError> {
    load_game_from_file(&get_default_save_path())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{GameSettings, GameDifficulty, SolarSystem, Planet};
    use crate::simulation::planet_types::PlanetType;
    use tempfile::tempdir;

    /// Helper function to create a valid game state for testing
    fn create_valid_test_state() -> GameState {
        let mut state = GameState::new();
        // Add at least one planet that matches the player's location
        state.solar_system.planets.push(Planet::new(
            "earth".to_string(),
            "Earth".to_string(),
            5,
            10,
            PlanetType::Agricultural,
        ));
        state.player.location = "earth".to_string();
        state
    }

    #[test]
    fn test_save_and_load_game() {
        // Create a temporary directory
        let dir = tempdir().expect("Failed to create temp dir");
        let save_path = dir.path().join("test_save.json");

        // Create a game state with valid planet
        let mut state = create_valid_test_state();
        state.player.money = 5000;
        state.settings.difficulty = GameDifficulty::Hard;

        // Save the game
        save_game_to_file(&state, save_path.to_str().unwrap()).expect("Failed to save game");

        // Load the game
        let loaded = load_game_from_file(save_path.to_str().unwrap()).expect("Failed to load game");

        // Verify the data
        assert_eq!(loaded.player.money, 5000);
        assert_eq!(loaded.settings.difficulty, GameDifficulty::Hard);
    }

    #[test]
    fn test_export_import_json() {
        let mut state = create_valid_test_state();
        state.player.money = 12345;

        let json = export_to_json(&state).expect("Failed to export");
        let imported = import_from_json(&json).expect("Failed to import");

        assert_eq!(imported.player.money, 12345);
    }

    #[test]
    fn test_validation() {
        let state = create_valid_test_state();
        let validation = validate_game_state(&state);

        // Should be valid with proper planet setup
        assert!(validation.is_valid);
    }

    #[test]
    fn test_validation_errors() {
        let mut state = GameState::new();
        state.player.location = "nonexistent_planet".to_string();

        let validation = validate_game_state(&state);

        // Should be invalid due to invalid player location
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }
}