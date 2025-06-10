pub enum PlayerAction {
    Travel(PlanetID),
    Buy(Good, u32),
    Sell(Good, u32)
}