#[derive(Debug)]
pub struct Ship {
    pub speed: f64,
    pub cargo_capacity: u32,
}

impl Ship {
    pub fn new(speed: f64, cargo_capacity: u32) -> Self {
        Ship {
            speed,
            cargo_capacity,
        }
    }
}