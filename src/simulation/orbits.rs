#[derive(Debug, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Planet {
    pub id: String,
    pub orbit_radius: f64,  // AU (astronomical units)
    pub orbit_period: f64,  // Months for full orbit
    pub position: Position,
    pub economy: economy::PlanetEconomy,
}

/// Calculates planet position at given time
pub fn calculate_position(radius: f64, period: f64, time: f64) -> Position {
    // Simple circular orbit calculation
    let angle = 2.0 * std::f64::consts::PI * (time / period);
    Position {
        x: radius * angle.cos(),
        y: radius * angle.sin(),
    }
}