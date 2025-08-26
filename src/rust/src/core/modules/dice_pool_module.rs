use crate::core::world::mechanics::die::Die;

pub struct DicePoolModule {
    label: String,
    die: Die,
    max_quantity: u16,
    remaining_quantity: u16,
}

impl DicePoolModule {
    pub fn new(label: String, die: Die, max_quantity: u16) -> Self {
        Self { label: label, die: die, remaining_quantity: max_quantity, max_quantity: max_quantity }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn die(&self) -> &Die {
        &self.die
    }

    pub fn set_die(&mut self, die: Die) {
        self.die = die;
    }

    pub fn max_quantity(&self) -> u16 {
        self.max_quantity
    }

    pub fn set_max_quantity(&mut self, max_quantity: u16) -> u16 {
        self.max_quantity = max_quantity.max(1);
        if self.remaining_quantity > self.max_quantity {
            self.remaining_quantity = self.max_quantity
        }
        self.max_quantity
    }

    pub fn remaining_quantity(&self) -> u16 {
        self.remaining_quantity
    }

    pub fn set_remaining_quantity(&mut self, remaining_quantity: u16) -> u16 {
        self.remaining_quantity = remaining_quantity.min(self.max_quantity);
        self.remaining_quantity
    }

    pub fn add_dice(&mut self, quantity: u16) -> u16 {
        self.remaining_quantity = (self.remaining_quantity + quantity).min(self.max_quantity);
        self.remaining_quantity
    }

    pub fn remove_dice(&mut self, quantity: u16) -> u16 {
        self.remaining_quantity = self.remaining_quantity.saturating_sub(quantity);
        self.remaining_quantity
    }

    pub fn reset_pool(&mut self) {
        self.remaining_quantity = self.max_quantity;
    }

    pub fn roll_die(&self) -> u16 {
        self.die.roll()
    }

    pub fn roll_and_remove_dice(&mut self, quantity: u16) -> Vec<u16> {
        self.remove_dice(quantity);
        self.die.roll_multiple(quantity)
    }
}

crate::impl_reflection!(DicePoolModule,);
impl super::module::Module for DicePoolModule {}
impl Default for DicePoolModule {
    fn default() -> Self {
        Self {
            label: String::from("Dice pool"),
            die: Die::D6,
            max_quantity: 2,
            remaining_quantity: 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_pool(max: u16) -> DicePoolModule {
        DicePoolModule::new("Test Pool".to_string(), Die::D6, max)
    }

    #[test]
    fn add_dice_clamps_to_max() {
        let mut pool = setup_pool(5);
        pool.set_remaining_quantity(4);
        assert_eq!(pool.add_dice(10), 5);
    }

    #[test]
    fn remove_dice_does_not_underflow() {
        let mut pool = setup_pool(5);
        pool.set_remaining_quantity(2);
        pool.remove_dice(10);
        assert_eq!(pool.remaining_quantity(), 0);
    }

    #[test]
    fn set_max_quantity_adjusts_remaining_quantity() {
        let mut pool = setup_pool(5);
        pool.set_remaining_quantity(5);
        assert_eq!(pool.set_max_quantity(3), 3);
        assert_eq!(pool.remaining_quantity(), 3);
    }

    #[test]
    fn roll_and_remove_dice_respects_quantity() {
        let mut pool = setup_pool(5);
        pool.set_remaining_quantity(3);
        let rolls = pool.roll_and_remove_dice(2);
        assert_eq!(rolls.len(), 2);
        assert_eq!(pool.remaining_quantity(), 1);
    }

    #[test]
    fn reset_pool_sets_remaining_to_max() {
        let mut pool = setup_pool(4);
        pool.set_remaining_quantity(1);
        pool.reset_pool();
        assert_eq!(pool.remaining_quantity(), pool.max_quantity());
    }
}

