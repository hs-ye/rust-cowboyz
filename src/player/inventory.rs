use std::collections::HashMap;

#[derive(Debug)]
pub struct CargoHold {
    pub capacity: u32,
    pub goods: HashMap<String, u32>, // Store goods by their ID (String) and quantity (u32)
}

impl CargoHold {
    pub fn new(capacity: u32) -> Self {
        CargoHold {
            capacity,
            goods: HashMap::new(),
        }
    }

    pub fn add_good(&mut self, good_id: String, quantity: u32) -> Result<(), String> {
        let current_load: u32 = self.goods.values().sum();
        if current_load + quantity > self.capacity {
            return Err(format!("Not enough cargo space. Available: {}", self.capacity - current_load));
        }
        *self.goods.entry(good_id).or_insert(0) += quantity;
        Ok(())
    }

    pub fn remove_good(&mut self, good_id: String, quantity: u32) -> Result<(), String> {
        let current_quantity = *self.goods.get(&good_id).unwrap_or(&0);
        if current_quantity < quantity {
            return Err(format!("Not enough {} in cargo. Available: {}", good_id, current_quantity));
        }
        *self.goods.entry(good_id.clone()).or_insert(0) -= quantity;
        if *self.goods.get(&good_id).unwrap() == 0 {
            self.goods.remove(&good_id);
        }
        Ok(())
    }

    pub fn get_goods_list(&self) -> Vec<(&String, &u32)> {
        self.goods.iter().collect()
    }

    pub fn current_load(&self) -> u32 {
        self.goods.values().sum()
    }
}