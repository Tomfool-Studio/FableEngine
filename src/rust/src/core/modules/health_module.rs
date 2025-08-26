pub struct HealthModule {
    max_health: i32,
    current_health: i32,
    temporary_health: i32,
}

impl HealthModule {
    pub fn new(max_health: i32) -> Self {
        Self { max_health: max_health, current_health: max_health, temporary_health: 0 }
    }

    pub fn max_health(&self) -> i32 {
        self.max_health
    }

    pub fn set_max_health(&mut self, max_health: i32) -> i32 {
        self.max_health = max_health.max(1);
        if self.current_health > self.max_health {
            self.current_health = self.max_health;
        }
        self.max_health
    }

    pub fn add_max_health(&mut self, max_health: i32) -> i32 {
        self.set_max_health(self.max_health + max_health);
        self.max_health
    }

    pub fn current_health(&self) -> i32 {
        self.current_health
    }

    pub fn set_current_health(&mut self, health: i32) -> i32 {
        self.current_health = health.clamp(0, self.max_health);
        self.current_health
    }

    pub fn add_current_health(&mut self, health: i32) -> i32 {
        self.set_current_health(self.current_health + health);
        self.current_health
    }

    pub fn temporary_health(&self) -> i32 {
        self.temporary_health
    }

    pub fn set_temporary_health(&mut self, temporary_health: i32) -> i32 {
        self.temporary_health = temporary_health.max(0);
        self.temporary_health
    }
}

crate::impl_reflection!(HealthModule,);
impl super::module::Module for HealthModule {}
impl Default for HealthModule {
    fn default() -> Self {
        HealthModule {
            max_health: 10,
            current_health: 10,
            temporary_health: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn max_health_higher_than_0() {
        let mut hm = HealthModule::new(20);
        assert_eq!(hm.set_max_health(0), 1);
    }

    #[test]
    fn current_and_max_health_update() {
        let mut hm = HealthModule::new(20);
        assert_eq!(hm.current_health(), 20);
        assert_eq!(hm.set_max_health(10), 10);
        assert_eq!(hm.current_health(), 10);
    }

    #[test]
    fn max_health_can_add() {
        let mut hm = HealthModule::new(20);
        assert_eq!(hm.add_max_health(10), 30);
        assert_eq!(hm.current_health(), 20);

        assert_eq!(hm.add_max_health(-30), 1);
        assert_eq!(hm.current_health(), 1);
    }

    #[test]
    fn current_health_between_0_and_max() {
        let mut hm = HealthModule::new(20);
        assert_eq!(hm.set_current_health(-21), 0);
        assert_eq!(hm.set_current_health(21), 20);
        assert_eq!(hm.set_current_health(10), 10);
    }

    #[test]
    fn current_health_can_add() {
        let mut hm = HealthModule::new(20);
        assert_eq!(hm.add_current_health(10), 20);
        assert_eq!(hm.add_current_health(-21), 0);
        assert_eq!(hm.add_current_health(10), 10);
    }
}

