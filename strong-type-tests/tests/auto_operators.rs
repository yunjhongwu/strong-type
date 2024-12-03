#[cfg(test)]
mod tests {
    use strong_type::StrongType;

    #[test]
    fn test_int_arithmetic() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
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

        let arr = [Second(2), Second(3), Second(5)];
        assert_eq!(arr.iter().sum::<Second>(), Second(10));
        assert_eq!(arr.iter().copied().sum::<Second>(), Second(10));
        assert_eq!(arr.iter().product::<Second>(), Second(30));
        assert_eq!(arr.iter().copied().product::<Second>(), Second(30));
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
        #[strong_type(auto_operators)]
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

        let arr = [Second(2.0), Second(3.5), Second(4.5)];
        assert_eq!(arr.iter().sum::<Second>(), Second(10.0));
        assert_eq!(arr.iter().copied().sum::<Second>(), Second(10.0));
        assert_eq!(arr.iter().product::<Second>(), Second(31.5));
        assert_eq!(arr.iter().copied().product::<Second>(), Second(31.5));
    }

    #[test]
    fn test_bool_ops() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
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
    fn test_bit_shift() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        struct CacheSize(i32);

        let x = CacheSize::new(5);
        let x_ref = &x;
        let shift = 2;

        assert_eq!(x << shift, CacheSize::new(20));
        assert_eq!(x_ref << shift, CacheSize::new(20));
        assert_eq!(x >> shift, CacheSize::new(1));
        assert_eq!(x_ref >> shift, CacheSize::new(1));
    }

    #[test]
    fn test_bit_logical() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        struct Mask(i32);

        let y = Mask::new(37);
        let y_ref = &y;

        let x = Mask::new(19);
        let x_ref = &x;
        assert_eq!(x & y, Mask::new(1));
        assert_eq!(x_ref & y, Mask::new(1));
        assert_eq!(x & y_ref, Mask::new(1));
        assert_eq!(x_ref & y_ref, Mask::new(1));

        let mut x = Mask::new(19);
        x &= y;
        assert_eq!(x, Mask::new(1));

        let mut x = Mask::new(19);
        x &= y_ref;
        assert_eq!(x, Mask::new(1));

        let x = Mask::new(19);
        let x_ref = &x;
        assert_eq!(x | y, Mask::new(55));
        assert_eq!(x_ref | y, Mask::new(55));
        assert_eq!(x | y_ref, Mask::new(55));
        assert_eq!(x_ref | y_ref, Mask::new(55));

        let mut x = Mask::new(19);
        x |= y;
        assert_eq!(x, Mask::new(55));

        let mut x = Mask::new(19);
        x |= y_ref;
        assert_eq!(x, Mask::new(55));

        let x = Mask::new(19);
        let x_ref = &x;
        assert_eq!(x ^ y, Mask::new(54));
        assert_eq!(x_ref ^ y, Mask::new(54));
        assert_eq!(x ^ y_ref, Mask::new(54));
        assert_eq!(x_ref ^ y_ref, Mask::new(54));

        let mut x = Mask::new(19);
        x ^= y;
        assert_eq!(x, Mask::new(54));

        let mut x = Mask::new(19);
        x ^= y_ref;
        assert_eq!(x, Mask::new(54));
    }

    #[test]
    fn test_addable() {
        #[derive(StrongType)]
        #[strong_type(addable)]
        struct Second(i32);

        let a = Second(15);

        assert_eq!(a + Second(2), Second(17));
        assert_eq!(a + Second(3), Second(18));
        assert_eq!(a + Second(4), Second(19));

        let mut d = Second(10);
        d += Second(2);
        assert_eq!(d, Second(12));

        d += Second(3);
        assert_eq!(d, Second(15));

        d += Second(4);
        assert_eq!(d, Second(19));
    }

    #[test]
    fn test_scalable() {
        #[derive(StrongType)]
        #[strong_type(scalable)]
        struct Second(i32);

        let a = Second(15);

        assert_eq!(&a * 2_i32, Second(30));
        assert_eq!(2_i32 * &a, Second(30));
        assert_eq!(&a / 3_i32, Second(5));
        assert_eq!(&a % 4_i32, Second(3));

        let mut d = Second(10);
        d *= 2_i32;
        assert_eq!(d, Second(20));

        d /= 4_i32;
        assert_eq!(d, Second(5));

        d %= 3_i32;
        assert_eq!(d, Second(2));
    }
}
