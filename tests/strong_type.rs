#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::collections::HashSet;
    use std::fmt::Display;
    use std::ops::Neg;
    use strong_type::{StrongNumericType, StrongType};

    #[test]
    fn test_basic() {
        #[derive(StrongNumericType)]
        struct NamedI8(i8);
        let _ = NamedI8::new(1);

        #[derive(StrongNumericType)]
        struct NamedI16(i16);
        let _ = NamedI16::new(1);

        #[derive(StrongNumericType)]
        struct NamedI32(i32);
        let _ = NamedI32::new(1);

        #[derive(StrongNumericType)]
        struct NamedI64(i64);
        let _ = NamedI64::new(1);

        #[derive(StrongNumericType)]
        struct NamedI128(i128);
        let _ = NamedI128::new(1);

        #[derive(StrongNumericType)]
        struct NamedISize(isize);
        let _ = NamedISize::new(1);

        #[derive(StrongNumericType)]
        struct NamedU8(u8);
        let _ = NamedU8::new(1);

        #[derive(StrongNumericType)]
        struct NamedU16(u16);
        let _ = NamedU16::new(1);

        #[derive(StrongNumericType)]
        struct NamedU32(u32);
        let _ = NamedU32::new(1);

        #[derive(StrongNumericType)]
        struct NamedU64(u64);
        let _ = NamedU64::new(1);

        #[derive(StrongNumericType)]
        struct NamedU128(u128);
        let _ = NamedU128::new(1);

        #[derive(StrongNumericType)]
        struct NamedUSize(usize);
        let _ = NamedUSize::new(1);

        #[derive(StrongNumericType)]
        struct NamedF32(f32);
        let _ = NamedF32::new(1.0);

        #[derive(StrongNumericType)]
        struct NamedF64(f64);
        let _ = NamedF64::new(1.0);

        #[derive(StrongType)]
        struct NamedBool(bool);
        let _ = NamedBool::new(true);

        #[derive(StrongType)]
        struct NamedChar(char);
        let _ = NamedChar::new('a');

        #[derive(StrongType)]
        struct NamedString(String);
        let _ = NamedString::new("string");
    }

    #[test]
    fn test_int_arithmetic() {
        #[derive(StrongNumericType)]
        struct Second(i32);

        let x = Second::new(2);
        let mut y = Second::new(3);
        let x_ref = &x;
        let y_ref = &y;

        assert_eq!(y + x, Second::new(5));
        assert_eq!(y + x_ref, Second::new(5));
        assert_eq!(y_ref + x, Second::new(5));
        assert_eq!(y_ref + x_ref, Second::new(5));

        assert_eq!(y - x, Second::new(1));
        assert_eq!(y - x_ref, Second::new(1));
        assert_eq!(y_ref - x, Second::new(1));
        assert_eq!(y_ref - x_ref, Second::new(1));

        assert_eq!(y * x, Second::new(6));
        assert_eq!(y * x_ref, Second::new(6));
        assert_eq!(y_ref * x, Second::new(6));
        assert_eq!(y_ref * x_ref, Second::new(6));

        assert_eq!(y / x, Second::new(1));
        assert_eq!(y / x_ref, Second::new(1));
        assert_eq!(y_ref / x, Second::new(1));
        assert_eq!(y_ref / x_ref, Second::new(1));

        assert_eq!(y % x, Second::new(1));
        assert_eq!(y % x_ref, Second::new(1));
        assert_eq!(y_ref % x, Second::new(1));
        assert_eq!(y_ref % x_ref, Second::new(1));

        y += x;
        assert_eq!(y, Second::new(5));
        y -= x;
        assert_eq!(y, Second::new(3));
        y *= x;
        assert_eq!(y, Second::new(6));
        y /= x;
        assert_eq!(y, Second::new(3));
        y %= x;
        assert_eq!(y, Second::new(1));

        let mut y = Second::new(3);
        y += x_ref;
        assert_eq!(y, Second::new(5));
        y -= x_ref;
        assert_eq!(y, Second::new(3));
        y *= x_ref;
        assert_eq!(y, Second::new(6));
        y /= x_ref;
        assert_eq!(y, Second::new(3));
        y %= x_ref;
        assert_eq!(y, Second::new(1));

        let z = Second::new(2);

        assert_eq!(z.neg(), Second::new(-2));
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
    }

    #[test]
    fn test_min_max() {
        #[derive(StrongType)]
        struct Second(i32);

        assert_eq!(Second::max().value(), i32::MAX);
        assert_eq!(Second::min().value(), i32::MIN);

        #[derive(StrongType)]
        struct Meter(f64);

        assert_eq!(Meter::max().value(), f64::MAX);
        assert_eq!(Meter::min().value(), f64::MIN);
    }

    #[test]
    fn test_float_arithmetic() {
        #[derive(StrongNumericType)]
        struct Second(f64);

        let x = Second::new(2.0);
        let mut y = Second::new(3.0);
        let x_ref = &x;
        let y_ref = &y;

        assert_eq!(y + x, Second::new(5.0));
        assert_eq!(y + x_ref, Second::new(5.0));
        assert_eq!(y_ref + x, Second::new(5.0));
        assert_eq!(y_ref + x_ref, Second::new(5.0));

        assert_eq!(y - x, Second::new(1.0));
        assert_eq!(y - x_ref, Second::new(1.0));
        assert_eq!(y_ref - x, Second::new(1.0));
        assert_eq!(y_ref - x_ref, Second::new(1.0));

        assert_eq!(y * x, Second::new(6.0));
        assert_eq!(y * x_ref, Second::new(6.0));
        assert_eq!(y_ref * x, Second::new(6.0));
        assert_eq!(y_ref * x_ref, Second::new(6.0));

        assert_eq!(y / x, Second::new(1.5));
        assert_eq!(y / x_ref, Second::new(1.5));
        assert_eq!(y_ref / x, Second::new(1.5));
        assert_eq!(y_ref / x_ref, Second::new(1.5));

        assert_eq!(y % x, Second::new(1.0));
        assert_eq!(y % x_ref, Second::new(1.0));
        assert_eq!(y_ref % x, Second::new(1.0));
        assert_eq!(y_ref % x_ref, Second::new(1.0));

        y += x;
        assert_eq!(y, Second::new(5.0));
        y -= x;
        assert_eq!(y, Second::new(3.0));
        y *= x;
        assert_eq!(y, Second::new(6.0));
        y /= x;
        assert_eq!(y, Second::new(3.0));
        y %= x;
        assert_eq!(y, Second::new(1.0));

        let mut y = Second::new(3.0);
        y += x_ref;
        assert_eq!(y, Second::new(5.0));
        y -= x_ref;
        assert_eq!(y, Second::new(3.0));
        y *= x_ref;
        assert_eq!(y, Second::new(6.0));
        y /= x_ref;
        assert_eq!(y, Second::new(3.0));
        y %= x_ref;
        assert_eq!(y, Second::new(1.0));

        let z = Second::new(2.0);

        assert_eq!(z.neg(), Second::new(-2.0));
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
    fn test_bool_negate() {
        #[derive(StrongType)]
        struct IsTrue(bool);
        let is_true = IsTrue(true);

        assert!(is_true.value());
        assert!(!(!is_true).value());
        assert!((!!is_true).value());
    }

    #[test]
    fn test_string_ctor() {
        #[derive(StrongType)]
        struct Meter(String);

        let _: Meter = "String".to_string().into();
        let _: Meter = "String".into();
        let _ = Meter::new("&str");
    }

    #[test]
    fn test_display() {
        #[derive(StrongNumericType)]
        struct Meter(i32);
        assert_eq!(format!("{}", Meter(-2)), "Meter(-2)");
        assert_eq!(format!("{:?}", Meter(-2)), "Meter { value: -2 }");

        #[derive(StrongNumericType)]
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
}
