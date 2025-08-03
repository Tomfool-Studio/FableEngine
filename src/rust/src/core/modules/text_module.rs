use super::module::Module;
use crate::impl_base_module;

pub struct TextModule {
    name: String,
    text: String,
}

impl TextModule {
    pub fn new(name: String, text: String) -> Self {
        Self { name: name, text: text }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl_base_module!(TextModule,);
impl Default for TextModule {
    fn default() -> Self {
        TextModule {
            name: String::from("Text title"),
            text: String::from("Text about something here"),
        }
    }
}
