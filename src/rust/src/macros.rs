#[macro_export]
macro_rules! impl_reflection {
    ($type:ty, $($impl_functions:item)*) => {
        impl crate::core::reflect::Reflect for $type {
            fn type_name(&self) -> &'static str {
                stringify!($type)
            }

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            $($impl_functions)*
        }
    };
}

