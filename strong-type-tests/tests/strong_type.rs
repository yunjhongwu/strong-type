#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::collections::HashSet;
    use std::mem;
    use strong_type::StrongType;

    fn test_type<T: std::fmt::Debug + Clone + Send + Sync + Default + PartialEq>() {}

    #[test]
    fn test_basic_with_new() {
        #[derive(StrongType)]
        struct NamedI8(i8);
        test_type::<NamedI8>();
        assert_eq!(mem::size_of::<NamedI8>(), mem::size_of::<i8>());

        #[derive(StrongType)]
        struct NamedI16(i16);
        test_type::<NamedI16>();
        assert_eq!(mem::size_of::<NamedI16>(), mem::size_of::<i16>());

        #[derive(StrongType)]
        struct NamedI32(i32);
        test_type::<NamedI32>();
        assert_eq!(mem::size_of::<NamedI32>(), mem::size_of::<i32>());

        #[derive(StrongType)]
        struct NamedI64(i64);
        test_type::<NamedI64>();
        assert_eq!(mem::size_of::<NamedI64>(), mem::size_of::<i64>());

        #[derive(StrongType)]
        struct NamedI128(i128);
        test_type::<NamedI128>();
        assert_eq!(mem::size_of::<NamedI128>(), mem::size_of::<i128>());

        #[derive(StrongType)]
        struct NamedISize(isize);
        test_type::<NamedISize>();
        assert_eq!(mem::size_of::<NamedISize>(), mem::size_of::<isize>());

        #[derive(StrongType)]
        struct NamedU8(u8);
        test_type::<NamedU8>();
        assert_eq!(mem::size_of::<NamedU8>(), mem::size_of::<u8>());

        #[derive(StrongType)]
        struct NamedU16(u16);
        test_type::<NamedU16>();
        assert_eq!(mem::size_of::<NamedU16>(), mem::size_of::<u16>());

        #[derive(StrongType)]
        struct NamedU32(u32);
        test_type::<NamedU32>();
        assert_eq!(mem::size_of::<NamedU32>(), mem::size_of::<u32>());

        #[derive(StrongType)]
        struct NamedU64(u64);
        test_type::<NamedU64>();
        assert_eq!(mem::size_of::<NamedU64>(), mem::size_of::<u64>());

        #[derive(StrongType)]
        struct NamedU128(u128);
        test_type::<NamedU128>();
        assert_eq!(mem::size_of::<NamedU128>(), mem::size_of::<u128>());

        #[derive(StrongType)]
        struct NamedUSize(usize);
        test_type::<NamedUSize>();
        assert_eq!(mem::size_of::<NamedUSize>(), mem::size_of::<usize>());

        #[derive(StrongType)]
        struct NamedF32(f32);
        test_type::<NamedF32>();
        assert_eq!(mem::size_of::<NamedF32>(), mem::size_of::<f32>());

        #[derive(StrongType)]
        struct NamedF64(f64);
        test_type::<NamedF64>();
        assert_eq!(mem::size_of::<NamedF64>(), mem::size_of::<f64>());

        #[derive(StrongType)]
        struct NamedBool(bool);
        test_type::<NamedBool>();
        assert_eq!(mem::size_of::<NamedBool>(), mem::size_of::<bool>());

        #[derive(StrongType)]
        struct NamedChar(char);
        test_type::<NamedChar>();
        assert_eq!(mem::size_of::<NamedChar>(), mem::size_of::<char>());

        #[derive(StrongType)]
        struct NamedString(String);
        test_type::<NamedString>();
        assert_eq!(mem::size_of::<NamedString>(), mem::size_of::<String>());
    }

    #[test]
    fn test_constants() {
        #[derive(StrongType)]
        struct Second(i32);

        assert_eq!(Second::MAX.value(), i32::MAX);
        assert_eq!(Second::MIN.value(), i32::MIN);
        assert_eq!(Second::ZERO.value(), 0i32);
        assert_eq!(Second::ONE.value(), 1i32);

        #[derive(StrongType)]
        struct State(u8);

        assert_eq!(State::MAX.value(), u8::MAX);
        assert_eq!(State::MIN.value(), u8::MIN);
        assert_eq!(State::ZERO.value(), 0u8);
        assert_eq!(State::ONE.value(), 1u8);

        #[derive(StrongType)]
        struct Meter(f32);

        assert_eq!(Meter::MAX.value(), f32::MAX);
        assert_eq!(Meter::MIN.value(), f32::MIN);
        assert_eq!(Meter::INFINITY.value(), f32::INFINITY);
        assert_eq!(Meter::NEG_INFINITY.value(), f32::NEG_INFINITY);
        assert_eq!(Meter::ZERO.value(), 0f32);
        assert_eq!(Meter::ONE.value(), 1f32);
    }

    #[test]
    fn test_constructor() {
        #[derive(StrongType)]
        struct Second(i32);
        let _ = Second::new(1);

        #[derive(StrongType)]
        struct Days(u32);
        let _ = Days::new(1u32);

        #[derive(StrongType)]
        struct Value(f64);
        let _ = Value::new(1.0);

        #[derive(StrongType)]
        struct Flag(bool);
        let _ = Flag::new(false);

        #[derive(StrongType)]
        struct Level(char);
        let _ = Level::new('A');

        #[derive(StrongType)]
        struct Name(String);
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
    fn test_float_nan_and_infinity() {
        #[derive(StrongType)]
        struct Meter(f64);

        let y = Meter::NAN;
        assert!(y.is_nan());
        assert!(y.value().is_nan());
        assert!(!y.is_finite());

        let z = Meter::new(0.0);
        assert!(!z.is_nan());
        assert!(!z.value().is_nan());
        assert!(z.is_finite());

        let w = Meter::INFINITY;
        assert!(!w.is_nan());
        assert!(!w.value().is_nan());
        assert!(!w.is_finite());

        let u = Meter::NEG_INFINITY;
        assert!(!u.is_nan());
        assert!(!u.value().is_nan());
        assert!(!u.is_finite());
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
        let _ = -Sign::new(1);

        impl std::fmt::Display for Sign {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "CustomDisplaySign({})", self.value())
            }
        }
    }

    #[test]
    fn test_const_new() {
        #[derive(StrongType)]
        struct NamedI32(i32);

        const NAMED_I32: NamedI32 = NamedI32::const_new(1);

        assert_eq!(NAMED_I32, NamedI32::new(1));
    }

    #[test]
    fn test_pub_type() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        pub struct NamedI32(pub i32);
        let mut x = NamedI32(1);
        x.0 += 2;
        assert_eq!(x, NamedI32::new(1) + NamedI32::new(2));
        assert_eq!(x.to_string(), "NamedI32(3)");

        #[derive(StrongType)]
        pub struct NamedString(pub String);
        let mut s = NamedString("Tim".to_string());
        s.0.push_str("er");

        assert_eq!(s, NamedString::new("Timer"));
        assert_eq!(s.to_string(), "NamedString(Timer)");
    }
}
