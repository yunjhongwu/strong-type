use std::fmt::Debug;
pub use strong_type_derive::StrongType;

pub trait StrongType: Debug + PartialEq + PartialOrd + Clone + Default + Send + Sync {
    type UnderlyingType: Default;
    fn new(value: impl Into<Self::UnderlyingType>) -> Self;
}
