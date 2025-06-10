pub struct EventSystem {
    pub events: Vec<Event>
}

pub enum Event {
    MarketCrash(Good),
    Discovery(PlanetID),
    // ...
}