use crate::simulation::economy::Good;

pub enum PlayerAction {
    Travel(String),
    Buy(Good, u32),
    Sell(Good, u32)
}