#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::collections::HashSet;
    use strong_type::StrongType;

    fn test_type<T: std::fmt::Debug + Clone + Send + Sync + Default + PartialEq>() {}

    #[test]
    fn test_basic() {
        #[derive(StrongType)]
        struct NamedI8(i8);
        test_type::<NamedI8>();

        #[derive(StrongType)]
        struct NamedI16(i16);
        test_type::<NamedI16>();

        #[derive(StrongType)]
        struct NamedI32(i32);
        test_type::<NamedI32>();

        #[derive(StrongType)]
        struct NamedI64(i64);
        test_type::<NamedI64>();

        #[derive(StrongType)]
        struct NamedI128(i128);
        test_type::<NamedI128>();

        #[derive(StrongType)]
        struct NamedISize(isize);
        test_type::<NamedISize>();

        #[derive(StrongType)]
        struct NamedU8(u8);
        test_type::<NamedU8>();

        #[derive(StrongType)]
        struct NamedU16(u16);
        test_type::<NamedU16>();

        #[derive(StrongType)]
        struct NamedU32(u32);
        test_type::<NamedU32>();

        #[derive(StrongType)]
        struct NamedU64(u64);
        test_type::<NamedU64>();

        #[derive(StrongType)]
        struct NamedU128(u128);
        test_type::<NamedU128>();

        #[derive(StrongType)]
        struct NamedUSize(usize);
        test_type::<NamedUSize>();

        #[derive(StrongType)]
        struct NamedF32(f32);
        test_type::<NamedF32>();

        #[derive(StrongType)]
        struct NamedF64(f64);
        test_type::<NamedF64>();

        #[derive(StrongType)]
        struct NamedBool(bool);
        test_type::<NamedBool>();

        #[derive(StrongType)]
        struct NamedChar(char);
        test_type::<NamedChar>();

        #[derive(StrongType)]
        struct NamedString(String);
        test_type::<NamedString>();
    }

    #[test]
    fn test_min_max() {
        #[derive(StrongType)]
        struct Second(i32);

        assert_eq!(Second::MAX.value(), i32::MAX);
        assert_eq!(Second::MIN.value(), i32::MIN);

        #[derive(StrongType)]
        struct Meter(f64);

        assert_eq!(Meter::MAX.value(), f64::MAX);
        assert_eq!(Meter::MIN.value(), f64::MIN);
    }

    #[test]
    fn test_constructor() {
        #[derive(StrongType)]
        struct Second(i32);
        let _ = Second(1);
        let _ = Second::new(1);

        #[derive(StrongType)]
        struct Days(u32);
        let _ = Days(1);
        let _ = Days::new(1u32);

        #[derive(StrongType)]
        struct Value(f64);
        let _ = Value(1.0);
        let _ = Value::new(1.0);

        #[derive(StrongType)]
        struct Flag(bool);
        let _ = Flag(true);
        let _ = Flag::new(false);

        #[derive(StrongType)]
        struct Level(char);
        let _ = Level('A');
        let _ = Level::new('A');

        #[derive(StrongType)]
        struct Name(String);
        let _ = Name("Tim".to_string());
        let _ = Name::new("Tim");
        let _ = Name::new("Tim".to_string());
    }

    #[test]
    fn test_comparison() {
        #[derive(StrongType)]
        struct Second(i32);

        let x = Second::new(2);
        let y = Second::new(3);
        let z = Second::new(2);

        assert!(x < y);
        assert!(x <= y);
        assert!(x <= z);
        assert!(y > x);
        assert!(y >= x);
        assert!(y >= z);
        assert_eq!(x, z);
        assert_ne!(x, y);
        assert_eq!(x.max(y), y);
        assert_eq!(x.min(y), x);

        #[derive(StrongType)]
        struct Meter(f64);

        let x = Meter::new(2.0);
        let y = Meter::new(3.0);
        let z = Meter::new(2.0);

        assert!(x < y);
        assert!(x <= y);
        assert!(x <= z);
        assert!(y > x);
        assert!(y >= x);
        assert!(y >= z);
        assert_eq!(x, z);
        assert_ne!(x, y);

        #[derive(StrongType)]
        struct Name(String);

        let x = Name::new("Tim");
        let y = Name::new("Tom");

        assert!(x < y);
        assert!(x <= y);
        assert!(y > x);
        assert!(y >= x);
    }

    #[test]
    fn test_strong_type() {
        #[derive(StrongType)]
        struct Second(i32);

        #[derive(StrongType)]
        struct Minute(i32);

        let x = Second::new(2);
        let y = Second::new(3);
        let z = Minute::new(3);

        assert_eq!(x.type_id(), y.type_id());
        assert_ne!(y.type_id(), z.type_id());
    }

    #[test]
    fn test_float_nan() {
        #[derive(StrongType)]
        struct Meter(f64);

        let y = Meter::nan();
        assert!(y.is_nan());
        assert!(y.value().is_nan());

        let z = Meter(0.0);
        assert!(!z.is_nan());
        assert!(!z.value().is_nan());
    }

    #[test]
    fn test_hash() {
        #[derive(StrongType)]
        struct Sign(bool);

        let mut map = HashSet::<Sign>::new();
        map.insert(Sign::new(true));
        map.insert(Sign::new(false));
        map.insert(Sign::new(false));
        assert_eq!(map.len(), 2);

        #[derive(StrongType)]
        struct Tag(String);

        let mut map = HashSet::<Tag>::new();
        map.insert(Tag::new("dev"));
        map.insert(Tag::new("prod"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_parse_attributes() {
        #[derive(StrongType)]
        #[strong_type(auto_operators, custom_display)]
        struct Sign(i32);
        let x = -Sign::new(1);

        impl std::fmt::Display for Sign {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "CustomDisplaySign({})", self.value())
            }
        }

        assert_eq!(format!("{}", x), "CustomDisplaySign(-1)");
    }
}
