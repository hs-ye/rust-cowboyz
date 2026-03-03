// src/assets/config_loader.rs

use serde::Deserialize;
use std::fs;

// Planet configuration structure
#[derive(Debug, Deserialize)]
pub struct PlanetConfig {
    pub id: String,
    pub orbit_radius: f64,
    pub orbit_period: f64,
    pub produces: Vec<String>,
    pub demands: Vec<String>,
    #[serde(default)] // This allows the field to be missing without causing an error
    pub ignores: Vec<String>,
}

// Base commodity config in the game, used to hold base unmodified values
// Some goods are more rarer/expensive than others
// During setup, depending on plantConfig randomise the value for each economy::MarketGood
#[derive(Debug, Deserialize, Clone)]
pub struct GoodConfig {
    pub name: String,
    pub base_value: u32,
}

/// Load planets configuration from YAML
pub fn load_planets_config(path: &str) -> Vec<PlanetConfig> {
    let content = fs::read_to_string(path)
        .expect("Failed to read planets config file");
    serde_yaml::from_str(&content)
        .expect("Failed to parse planets config")
}

/// Load goods configuration from YAML
pub fn load_goods_config(path: &str) -> Vec<GoodConfig> {
    let content = fs::read_to_string(path)
        .expect("Failed to read goods config file");
    serde_yaml::from_str(&content)
        .expect("Failed to parse goods config")
}