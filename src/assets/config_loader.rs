// src/assets/config_loader.rs

use serde::Deserialize;
use std::fs;

// Planet configuration structure
#[derive(Debug, Deserialize)]
pub struct PlanetConfig {
    pub id: String,
    #[serde(default)]
    pub name: String,        // Human-readable planet name
    pub orbit_radius: u32,   // Integer distance from star
    pub orbit_period: u32,   // Turns to complete one orbit
    pub planet_type: String, // Planet type as string, will be converted to enum
    #[serde(default)] // This allows the field to be missing without causing an error
    pub produces: Vec<String>, // Optional override for produces
    #[serde(default)] // This allows the field to be missing without causing an error
    pub demands: Vec<String>, // Optional override for demands
    #[serde(default)] // This allows the field to be missing without causing an error
    #[allow(dead_code)]
    pub ignores: Vec<String>, // Optional override for ignores
}

// Base commodity config in the game, used to hold base unmodified values
// Some goods are more rarer/expensive than others
// During setup, depending on plantConfig randomise the value for each economy::MarketGood
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct GoodConfig {
    pub name: String,
    pub base_value: u32,
}

/// Load planets configuration from YAML
pub fn load_planets_config(path: &str) -> Vec<PlanetConfig> {
    let content = fs::read_to_string(path).expect("Failed to read planets config file");
    serde_yaml::from_str(&content).expect("Failed to parse planets config")
}

/// Load goods configuration from YAML
#[allow(dead_code)]
pub fn load_goods_config(path: &str) -> Vec<GoodConfig> {
    let content = fs::read_to_string(path).expect("Failed to read goods config file");
    serde_yaml::from_str(&content).expect("Failed to parse goods config")
}
