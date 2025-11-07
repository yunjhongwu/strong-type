#[cfg(test)]
mod tests {
    use static_assertions::assert_not_impl_any;
    use strong_type::StrongType;

    #[derive(StrongType)]
    struct Seconds(i32);

    #[test]
    fn test_inner_helpers_and_traits() {
        let mut seconds = Seconds::new(10);
        assert_eq!(*seconds.as_ref(), 10);

        *seconds.as_mut() = 20;
        assert_eq!(seconds.value(), 20);

        fn takes_as_ref(value: impl AsRef<i32>) -> i32 {
            *value.as_ref()
        }
        assert_eq!(takes_as_ref(Seconds::new(5)), 5);

        assert_eq!(seconds.into_inner(), 20);
    }

    #[derive(StrongType)]
    #[strong_type(conversion)]
    struct WithConversions(u32);

    #[test]
    fn test_additional_conversions() {
        let base = 7u32;

        let cloned = WithConversions::from(&base);
        assert_eq!(cloned.value(), base);

        let borrowed: &u32 = (&cloned).into();
        assert_eq!(*borrowed, base);

        let mut mutable = WithConversions::new(1u32);
        {
            let mutable_ref: &mut u32 = (&mut mutable).into();
            *mutable_ref = 9;
        }
        assert_eq!(mutable.value(), 9);
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct Manual(i32);

    impl StrongType for Manual {
        type UnderlyingType = i32;
        type PrimitiveType = i32;
    }

    #[test]
    fn test_manual_strong_type_does_not_require_default() {
        assert_not_impl_any!(Manual: Default);

        fn accepts_strong_type<T: StrongType>(value: T) -> T {
            value
        }

        let manual = Manual(42);
        let _ = accepts_strong_type(manual);
    }
}
