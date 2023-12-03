#[cfg(test)]
mod tests {
    use named_type::{NamedNumeric, NamedType};
    use std::any::Any;
    use std::collections::HashSet;
    use std::fmt::Display;

    #[test]
    fn test_basic() {
        #[derive(NamedNumeric)]
        struct NamedI8(i8);
        let _ = NamedI8(1);

        #[derive(NamedNumeric)]
        struct NamedI16(i16);
        let _ = NamedI16(1);

        #[derive(NamedNumeric)]
        struct NamedI32(i32);
        let _ = NamedI32(1);

        #[derive(NamedNumeric)]
        struct NamedI64(i64);
        let _ = NamedI64(1);

        #[derive(NamedNumeric)]
        struct NamedI128(i128);
        let _ = NamedI128(1);

        #[derive(NamedNumeric)]
        struct NamedISize(isize);
        let _ = NamedISize(1);

        #[derive(NamedNumeric)]
        struct NamedU8(u8);
        let _ = NamedU8(1);

        #[derive(NamedNumeric)]
        struct NamedU16(u16);
        let _ = NamedU16(1);

        #[derive(NamedNumeric)]
        struct NamedU32(u32);
        let _ = NamedU32(1);

        #[derive(NamedNumeric)]
        struct NamedU64(u64);
        let _ = NamedU64(1);

        #[derive(NamedNumeric)]
        struct NamedU128(u128);
        let _ = NamedU128(1);

        #[derive(NamedNumeric)]
        struct NamedUSize(usize);
        let _ = NamedUSize(1);

        #[derive(NamedNumeric)]
        struct NamedF32(f32);
        let _ = NamedF32(1.0);

        #[derive(NamedNumeric)]
        struct NamedF64(f64);
        let _ = NamedF64(1.0);

        #[derive(NamedType)]
        struct NamedBool(bool);
        let _ = NamedBool(true);

        #[derive(NamedType)]
        struct NamedChar(char);
        let _ = NamedChar('a');

        #[derive(NamedType)]
        struct NamedString(String);
        let _ = NamedString("string".to_string());
    }

    #[test]
    fn test_arithmetic() {
        #[derive(NamedNumeric)]
        struct Second(i32);

        let x = Second(2);
        let mut y = Second(3);
        assert_eq!(y + x, Second(5));
        assert_eq!(y - x, Second(1));
        assert_eq!(y * x, Second(6));
        assert_eq!(y / x, Second(1));
        assert!(x < y);
        assert!(y >= x);

        y += x;
        assert_eq!(y, Second(5));
        y -= x;
        assert_eq!(y, Second(3));
        y *= x;
        assert_eq!(y, Second(6));
        y /= x;
        assert_eq!(y, Second(3));
        #[derive(NamedNumeric)]
        struct Minute(i32);
        let z = Minute(2);

        assert_eq!(-z, Minute(-2));

        assert_eq!(Minute::max().value(), i32::MAX);
        assert_eq!(Minute::min().value(), i32::MIN);
    }

    #[test]
    fn test_strong_type() {
        #[derive(NamedType)]
        struct Second(i32);

        #[derive(NamedType)]
        struct Minute(i32);

        let x = Second(2);
        let y = Second(3);
        let z = Minute(3);

        assert_eq!(x.type_id(), y.type_id());
        assert_ne!(y.type_id(), z.type_id());
    }

    #[test]
    fn test_float() {
        #[derive(NamedType)]
        struct Meter(f64);

        let y = Meter::nan();
        assert!(y.is_nan());
        assert!(y.value().is_nan());

        let z = Meter(0.0);
        assert!(!z.is_nan());
        assert!(!z.value().is_nan());
    }

    #[test]
    fn test_bool() {
        #[derive(NamedType)]
        struct IsTrue(bool);
        let is_true = IsTrue(true);

        assert!(is_true.value());
        assert!(!(!is_true).value());
        assert!((!!is_true).value());
    }

    #[test]
    fn test_string_ctor() {
        #[derive(NamedType)]
        struct Meter(String);

        let _: Meter = "String".to_string().into();
        let _: Meter = "String".into();
        let _ = Meter("String".to_string());
        let _ = Meter::new("&str");
    }

    #[test]
    fn test_display() {
        #[derive(NamedNumeric)]
        struct Meter(i32);
        assert_eq!(format!("{}", Meter(-2)), "Meter(-2)");
        assert_eq!(format!("{:?}", Meter(-2)), "Meter { value: -2 }");

        #[derive(NamedNumeric)]
        #[custom_display]
        struct Mile(f64);

        impl Display for Mile {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "Mile({:.2})", &self.0)
            }
        }
        assert_eq!(format!("{}", Mile(std::f64::consts::E)), "Mile(2.72)");
        assert_eq!(
            format!("{:?}", Mile(std::f64::consts::E)),
            format!("Mile {{ value: {} }}", std::f64::consts::E)
        );
    }

    #[test]
    fn test_hash() {
        #[derive(NamedType)]
        struct Sign(bool);

        let mut map = HashSet::<Sign>::new();
        map.insert(Sign(true));
        map.insert(Sign(false));
        map.insert(Sign(false));
        assert_eq!(map.len(), 2);

        #[derive(NamedType)]
        struct Tag(String);

        let mut map = HashSet::<Tag>::new();
        map.insert(Tag::new("dev"));
        map.insert(Tag::new("prod"));
        assert_eq!(map.len(), 2);
    }
}
