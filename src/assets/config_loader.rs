// src/assets/config_loader.rs

use serde::Deserialize;
use std::fs;

// Planet configuration structure
#[derive(Debug, Deserialize)]
pub struct PlanetConfig {
    pub id: String,
    pub orbit_radius: f64,
    pub orbit_period: f64,
    #[serde(default)]
    pub economy: PlanetEconomy,  // Add this line to include the economy data
}

// Good configuration structure
#[derive(Debug, Deserialize)]
pub struct GoodConfig {
    pub id: String,
    pub base_value: f64,
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