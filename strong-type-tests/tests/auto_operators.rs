#[cfg(test)]
mod tests {
    use strong_type::StrongType;

    #[test]
    fn test_int_arithmetic() {
        #[derive(StrongType)]
        #[auto_operators]
        struct Second(i32);

        let x = Second(2);
        let mut y = Second(3);
        let x_ref = &x;
        let y_ref = &y;

        assert_eq!(y + x, Second(5));
        assert_eq!(y + x_ref, Second(5));
        assert_eq!(y_ref + x, Second(5));
        assert_eq!(y_ref + x_ref, Second(5));

        assert_eq!(y - x, Second(1));
        assert_eq!(y - x_ref, Second(1));
        assert_eq!(y_ref - x, Second(1));
        assert_eq!(y_ref - x_ref, Second(1));

        assert_eq!(y * x, Second(6));
        assert_eq!(y * x_ref, Second(6));
        assert_eq!(y_ref * x, Second(6));
        assert_eq!(y_ref * x_ref, Second(6));

        assert_eq!(y / x, Second(1));
        assert_eq!(y / x_ref, Second(1));
        assert_eq!(y_ref / x, Second(1));
        assert_eq!(y_ref / x_ref, Second(1));

        assert_eq!(y % x, Second(1));
        assert_eq!(y % x_ref, Second(1));
        assert_eq!(y_ref % x, Second(1));
        assert_eq!(y_ref % x_ref, Second(1));

        y += x;
        assert_eq!(y, Second(5));
        y -= x;
        assert_eq!(y, Second(3));
        y *= x;
        assert_eq!(y, Second(6));
        y /= x;
        assert_eq!(y, Second(3));
        y %= x;
        assert_eq!(y, Second(1));

        let mut y = Second(3);
        y += x_ref;
        assert_eq!(y, Second(5));
        y -= x_ref;
        assert_eq!(y, Second(3));
        y *= x_ref;
        assert_eq!(y, Second(6));
        y /= x_ref;
        assert_eq!(y, Second(3));
        y %= x_ref;
        assert_eq!(y, Second(1));

        let z = Second(2);

        assert_eq!(-z, Second(-2));
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
    fn test_float_arithmetic() {
        #[derive(StrongType)]
        #[auto_operators]
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

        assert_eq!(-z, Second::new(-2.0));
    }

    #[test]
    fn test_bool_ops() {
        #[derive(StrongType)]
        #[auto_operators]
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
}
