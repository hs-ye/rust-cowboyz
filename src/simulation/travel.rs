use crate::simulation::orbits::{Planet, Position};

fn distance(pos1: &Position, pos2: &Position) -> f64 {
    ((pos2.x - pos1.x).powi(2) + (pos2.y - pos1.y).powi(2)).sqrt()
}

pub fn calculate_travel_time(origin: &Planet, target: &Planet, ship_speed: f64) -> u32 {
    let dist = distance(&origin.position, &target.position);
    (dist / ship_speed).ceil() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::economy::PlanetEconomy;

    #[test]
    fn test_calculate_travel_time() {
        let planet1 = Planet {
            id: "earth".to_string(),
            orbit_radius: 1.0,
            orbit_period: 12.0,
            position: Position { x: 1.0, y: 0.0 },
            economy: PlanetEconomy { market: vec![] },
        };
        let planet2 = Planet {
            id: "mars".to_string(),
            orbit_radius: 1.5,
            orbit_period: 24.0,
            position: Position { x: -1.5, y: 0.0 },
            economy: PlanetEconomy { market: vec![] },
        };

        let travel_time = calculate_travel_time(&planet1, &planet2, 0.5);
        assert_eq!(travel_time, 5); // distance is 2.5, 2.5 / 0.5 = 5
    }
}