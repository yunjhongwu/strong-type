#[cfg(test)]
mod tests {
    use strong_type::StrongType;

    #[test]
    fn test_minimal_int_arithmetic() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "minimal")]
        struct Second(i32);

        let x = Second(2);
        let mut y = Second(3);

        // Test owned + owned operations (should work)
        assert_eq!(y + x, Second(5));
        assert_eq!(y - x, Second(1));
        assert_eq!(y * x, Second(6));
        assert_eq!(y / x, Second(1));
        assert_eq!(y % x, Second(1));

        // Test assignment operators (should work)
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

        // Test negation (should work)
        let z = Second(2);
        assert_eq!(-z, Second(-2));

        // Test iterator traits (should work)
        let arr = [Second(2), Second(3), Second(5)];
        assert_eq!(arr.into_iter().sum::<Second>(), Second(10));
        assert_eq!(arr.into_iter().product::<Second>(), Second(30));
    }

    #[test]
    fn test_minimal_uint_arithmetic() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "minimal")]
        struct Count(u32);

        let x = Count(2);
        let mut y = Count(3);

        // Test owned + owned operations
        assert_eq!(y + x, Count(5));
        assert_eq!(y - x, Count(1));
        assert_eq!(y * x, Count(6));
        assert_eq!(y / x, Count(1));
        assert_eq!(y % x, Count(1));

        // Test assignment operators
        y += x;
        assert_eq!(y, Count(5));
        y -= x;
        assert_eq!(y, Count(3));
        y *= x;
        assert_eq!(y, Count(6));
        y /= x;
        assert_eq!(y, Count(3));
        y %= x;
        assert_eq!(y, Count(1));

        // Test iterator traits
        let arr = [Count(2), Count(3), Count(5)];
        assert_eq!(arr.into_iter().sum::<Count>(), Count(10));
        assert_eq!(arr.into_iter().product::<Count>(), Count(30));
    }

    #[test]
    fn test_minimal_float_arithmetic() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "minimal")]
        struct Meter(f64);

        let x = Meter(2.0);
        let mut y = Meter(3.0);

        // Test owned + owned operations
        assert_eq!(y + x, Meter(5.0));
        assert_eq!(y - x, Meter(1.0));
        assert_eq!(y * x, Meter(6.0));
        assert_eq!(y / x, Meter(1.5));

        // Test assignment operators
        y += x;
        assert_eq!(y, Meter(5.0));
        y -= x;
        assert_eq!(y, Meter(3.0));
        y *= x;
        assert_eq!(y, Meter(6.0));
        y /= x;
        assert_eq!(y, Meter(3.0));

        // Test negation
        let z = Meter(2.0);
        assert_eq!(-z, Meter(-2.0));

        // Test iterator traits
        let arr = [Meter(2.0), Meter(3.0), Meter(5.0)];
        assert_eq!(arr.into_iter().sum::<Meter>(), Meter(10.0));
        assert_eq!(arr.into_iter().product::<Meter>(), Meter(30.0));
    }

    #[test]
    fn test_minimal_bool_operations() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "minimal")]
        struct Flag(bool);

        let x = Flag(true);
        let y = Flag(false);
        let mut z = Flag(true);

        // Test owned + owned operations
        assert_eq!(x & y, Flag(false));
        assert_eq!(x | y, Flag(true));
        assert_eq!(x ^ y, Flag(true));
        assert_eq!(!x, Flag(false));
        assert_eq!(!y, Flag(true));

        // Test assignment operators
        z &= y;
        assert_eq!(z, Flag(false));
        z |= x;
        assert_eq!(z, Flag(true));
        z ^= x;
        assert_eq!(z, Flag(false));
    }

    #[test]
    fn test_full_mode_backward_compatibility() {
        // Ensure that not specifying a value defaults to full mode
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        struct Distance(i32);

        let x = Distance(2);
        let x_ref = &x;
        let y = Distance(3);
        let y_ref = &y;

        // Should work with all ownership variants (full mode)
        assert_eq!(y + x, Distance(5));
        assert_eq!(y + x_ref, Distance(5));
        assert_eq!(y_ref + x, Distance(5));
        assert_eq!(y_ref + x_ref, Distance(5));
    }

    #[test]
    fn test_explicit_full_mode() {
        // Ensure that explicitly specifying "full" works
        #[derive(StrongType)]
        #[strong_type(auto_operators = "full")]
        struct Height(i32);

        let x = Height(2);
        let x_ref = &x;
        let y = Height(3);
        let y_ref = &y;

        // Should work with all ownership variants (full mode)
        assert_eq!(y + x, Height(5));
        assert_eq!(y + x_ref, Height(5));
        assert_eq!(y_ref + x, Height(5));
        assert_eq!(y_ref + x_ref, Height(5));
    }
}
