use super::super::reflect::Reflect;

pub trait Module: Reflect {}

impl dyn Module {
    pub fn downcast_module<T: Module + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}
