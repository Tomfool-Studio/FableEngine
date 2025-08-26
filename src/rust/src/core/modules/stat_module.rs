pub struct StatModule {
    name: String,
    value: i32,
}

impl StatModule {
    pub fn new(name: String, value: i32) -> Self {
        Self { name: name, value: value }
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

crate::impl_reflection!(StatModule,);
impl super::module::Module for StatModule {}
impl Default for StatModule {
    fn default() -> Self {
        StatModule {
            name: String::from("Stat name"),
            value: 0,
        }
    }
}

