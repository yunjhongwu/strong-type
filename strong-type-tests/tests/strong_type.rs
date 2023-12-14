#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::collections::HashSet;
    use std::fmt::Display;
    use std::ops::Neg;
    use strong_type::StrongType;

    fn are_basic_traits_implemented<
        T: std::fmt::Debug + Clone + Send + Sync + Default + PartialEq,
    >() {
    }
    fn test_type<T: StrongType>(default_value: T) {
        are_basic_traits_implemented::<T>();
        assert_eq!(T::default(), default_value);
    }

    #[test]
    fn test_basic() {
        #[derive(StrongType)]
        struct NamedI8(i8);
        test_type::<NamedI8>(NamedI8::new(0));

        #[derive(StrongType)]
        struct NamedI16(i16);
        test_type::<NamedI16>(NamedI16::new(0));

        #[derive(StrongType)]
        struct NamedI32(i32);
        test_type::<NamedI32>(NamedI32::new(0));

        #[derive(StrongType)]
        struct NamedI64(i64);
        test_type::<NamedI64>(NamedI64::new(0));

        #[derive(StrongType)]
        struct NamedI128(i128);
        test_type::<NamedI128>(NamedI128::new(0));

        #[derive(StrongType)]
        struct NamedISize(isize);
        test_type::<NamedISize>(NamedISize::new(0));

        #[derive(StrongType)]
        struct NamedU8(u8);
        test_type::<NamedU8>(NamedU8::new(0));

        #[derive(StrongType)]
        struct NamedU16(u16);
        test_type::<NamedU16>(NamedU16::new(0));

        #[derive(StrongType)]
        struct NamedU32(u32);
        test_type::<NamedU32>(NamedU32::new(0));

        #[derive(StrongType)]
        struct NamedU64(u64);
        test_type::<NamedU64>(NamedU64::new(0));

        #[derive(StrongType)]
        struct NamedU128(u128);
        test_type::<NamedU128>(NamedU128::new(0));

        #[derive(StrongType)]
        struct NamedUSize(usize);
        test_type::<NamedUSize>(NamedUSize::new(0));

        #[derive(StrongType)]
        struct NamedF32(f32);
        test_type::<NamedF32>(NamedF32::new(0.0));

        #[derive(StrongType)]
        struct NamedF64(f64);
        test_type::<NamedF64>(NamedF64::new(0.0));

        #[derive(StrongType)]
        struct NamedBool(bool);
        test_type::<NamedBool>(NamedBool::new(false));

        #[derive(StrongType)]
        struct NamedChar(char);
        test_type::<NamedChar>(NamedChar::new('\0'));

        #[derive(StrongType)]
        struct NamedString(String);
        test_type::<NamedString>(NamedString::new(""));
    }

    #[test]
    fn test_int_arithmetic() {
        #[derive(StrongType)]
        #[numeric]
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
        #[derive(StrongType)]
        #[numeric]
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
    fn test_bool_ops() {
        #[derive(StrongType)]
        #[numeric]
        struct IsTrue(bool);

        let x = IsTrue::new(true);
        let y = IsTrue::new(false);
        let x_ref = &x;
        let y_ref = &y;

        assert_eq!(x & y, IsTrue::new(false));
        assert_eq!(x | y, IsTrue::new(true));
        assert_eq!(x ^ y, IsTrue::new(true));

        assert_eq!(x_ref & y, IsTrue::new(false));
        assert_eq!(x_ref | y, IsTrue::new(true));
        assert_eq!(x_ref ^ y, IsTrue::new(true));

        assert_eq!(x & y_ref, IsTrue::new(false));
        assert_eq!(x | y_ref, IsTrue::new(true));
        assert_eq!(x ^ y_ref, IsTrue::new(true));

        assert_eq!(x_ref & y_ref, IsTrue::new(false));
        assert_eq!(x_ref | y_ref, IsTrue::new(true));

        assert_eq!(!x, IsTrue::new(false));
        assert_eq!(!x_ref, IsTrue::new(false));
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
        #[derive(StrongType)]
        struct Meter(i32);
        assert_eq!(format!("{}", Meter(-2)), "Meter(-2)");
        assert_eq!(format!("{:?}", Meter(-2)), "Meter { value: -2 }");

        #[derive(StrongType)]
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