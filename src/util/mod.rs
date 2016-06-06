mod math;

pub use self::math::{Vector, Vector2f};

/// Generate the builder setter methods for any parameter.
#[macro_export]
macro_rules! builder_setters {
    (
        options => { $($setter_opt_name:ident { $property_opt_name:ident : $ParameterOptType:ty }),* };
        others => { $($setter_name:ident { $property_name:ident : $ParameterType:ty }),* }
    ) => (
        $(
            #[allow(dead_code)]
            pub fn $setter_name(mut self, value: $ParameterType) -> Self {
                self.$property_name = value; self
            }
        )*
        $(
            #[allow(dead_code)]
            pub fn $setter_opt_name(mut self, value: $ParameterOptType) -> Self {
                self.$property_opt_name = Some(value); self
            }
        )*
    )
}

/// Unwrap the value from Option<T> if it is Some, otherwise throw a Result::Err<T, String>.
#[macro_export]
macro_rules! unwrap_or_err {
    ($option:expr, $err:expr) => (
        match $option {
            Some(value) => value,
            None => return Err($err.into()),
        }
    )
}
