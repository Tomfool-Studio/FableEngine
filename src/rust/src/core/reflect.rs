pub trait Reflect: ::std::any::Any {
    fn type_name(&self) -> &'static str;

    fn as_any(&self) -> &dyn ::std::any::Any;
}
