pub struct DeathSaveModule {
    successes: u8,
    failures: u8,
    keep_failures: bool,
}

impl DeathSaveModule {
    pub fn new(sucesses: u8, failures: u8, keep_failures: bool) -> Self {
        Self { successes: sucesses, failures: failures, keep_failures: keep_failures }
    }

    pub fn successes(&self) -> u8 {
        self.successes
    }

    pub fn add_successes(&mut self, sucesses: u8) -> u8 {
        self.successes = (self.successes + sucesses).min(10);
        self.successes
    }

    pub fn reset_successes(&mut self) {
        self.successes = 0;
    }

    pub fn failures(&self) -> u8 {
        self.failures
    }

    pub fn add_failures(&mut self, failures: u8) -> u8 {
        self.failures = (self.failures + failures).min(10);
        self.failures
    }

    pub fn reset_failures(&mut self) {
        if self.keep_failures { return }
        self.failures = 0;
    }

    pub fn force_reset_failures(&mut self) {
        self.failures = 0;
    }

    pub fn keep_failures(&self) -> bool {
        self.keep_failures
    }

    pub fn set_keep_failures(&mut self, keep_failures: bool) {
        self.keep_failures = keep_failures;
    }

    pub fn reset_all(&mut self) {
        self.reset_successes();
        self.reset_failures();
    }

    pub fn force_reset_all(&mut self) {
        self.reset_successes();
        self.force_reset_failures();
    }
}

crate::impl_reflection!(DeathSaveModule,);
impl super::module::Module for DeathSaveModule {}
impl Default for DeathSaveModule {
    fn default() -> Self {
        DeathSaveModule {
             successes: 0,
             failures: 0,
             keep_failures: false, 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sucesses_add_and_reset() {
        let mut dsm = DeathSaveModule::default();
        assert_eq!(dsm.add_successes(1), 1);
        dsm.reset_successes();
        assert_eq!(dsm.successes(), 0);
    }

    #[test]
    fn failures_are_kept() {
        let mut dsm = DeathSaveModule::default();
        assert_eq!(dsm.add_failures(2), 2);
        dsm.set_keep_failures(true);
        dsm.reset_failures();
        assert_eq!(dsm.failures(), 2);
    }
}

