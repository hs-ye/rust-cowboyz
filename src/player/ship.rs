/// Ship configuration for the player
/// Based on ADR 0002: Movement Mechanics System
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub speed: f64,          // Legacy field (kept for compatibility)
    pub acceleration: u32,   // Ship acceleration in units/turn² (default: 1)
    pub cargo_capacity: u32, // Maximum cargo capacity
}

impl Ship {
    /// Create a new ship with default acceleration of 1 unit/turn²
    pub fn new(speed: f64, cargo_capacity: u32) -> Self {
        Ship {
            speed,
            acceleration: 1, // Default acceleration
            cargo_capacity,
        }
    }

    /// Create a ship with custom acceleration
    pub fn with_acceleration(speed: f64, acceleration: u32, cargo_capacity: u32) -> Self {
        Ship {
            speed,
            acceleration: acceleration.max(1), // Ensure at least 1
            cargo_capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_new() {
        let ship = Ship::new(10.0, 50);
        assert_eq!(ship.speed, 10.0);
        assert_eq!(ship.acceleration, 1); // Default
        assert_eq!(ship.cargo_capacity, 50);
    }

    #[test]
    fn test_ship_with_acceleration() {
        let ship = Ship::with_acceleration(10.0, 4, 50);
        assert_eq!(ship.speed, 10.0);
        assert_eq!(ship.acceleration, 4);
        assert_eq!(ship.cargo_capacity, 50);
    }

    #[test]
    fn test_ship_acceleration_minimum() {
        // Acceleration should be at least 1
        let ship = Ship::with_acceleration(10.0, 0, 50);
        assert_eq!(ship.acceleration, 1);
    }
}
