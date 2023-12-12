use std::fmt::Debug;
pub use strong_type_derive::{StrongNumericType, StrongType};

pub trait StrongType: Debug + PartialEq + PartialOrd + Clone + Default + Send + Sync {
    type UnderlyingType;
}
