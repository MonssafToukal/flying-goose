use core::fmt;

pub struct InvalidEnumConversionError;

impl fmt::Display for InvalidEnumConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid integer value for enum converstion")
    }
}
