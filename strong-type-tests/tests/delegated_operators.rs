//! Tests for delegated operator mode
//!
//! These tests verify that the `auto_operators = "delegated"` mode works correctly.
//! Delegated mode generates operators that delegate to shared implementations,
//! reducing binary size by 30-50% compared to full mode.

#[cfg(test)]
mod tests {
    use strong_type::StrongType;

    #[test]
    fn test_delegated_int_arithmetic() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct Distance(i32);

        let a = Distance::new(10);
        let b = Distance::new(5);

        assert_eq!(a + b, Distance::new(15));
        assert_eq!(a - b, Distance::new(5));
        assert_eq!(a * b, Distance::new(50));
        assert_eq!(a / b, Distance::new(2));
        assert_eq!(a % b, Distance::new(0));

        // Test negation
        assert_eq!(-a, Distance::new(-10));

        // Test reference variants (intentionally using references)
        let a2 = Distance::new(10);
        let b2 = Distance::new(5);
        #[allow(clippy::op_ref)]
        {
            assert_eq!(&a2 + b2, Distance::new(15));
            assert_eq!(&a2 - &b2, Distance::new(5));
        }
    }

    #[test]
    fn test_delegated_uint_arithmetic() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct Count(u32);

        let a = Count::new(10u32);
        let b = Count::new(5u32);

        assert_eq!(a + b, Count::new(15u32));
        assert_eq!(a - b, Count::new(5u32));
        assert_eq!(a * b, Count::new(50u32));
        assert_eq!(a / b, Count::new(2u32));
        assert_eq!(a % b, Count::new(0u32));
    }

    #[test]
    fn test_delegated_float_arithmetic() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct Price(f64);

        let a = Price::new(10.5);
        let b = Price::new(2.0);

        assert_eq!(a + b, Price::new(12.5));
        assert_eq!(a - b, Price::new(8.5));
        assert_eq!(a * b, Price::new(21.0));
        assert_eq!(a / b, Price::new(5.25));

        // Test negation
        assert_eq!(-a, Price::new(-10.5));
    }

    #[test]
    fn test_delegated_bool_operations() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct Flag(bool);

        let t = Flag::new(true);
        let f = Flag::new(false);

        assert_eq!(t & f, Flag::new(false));
        assert_eq!(t | f, Flag::new(true));
        assert_eq!(t ^ f, Flag::new(true));
        assert_eq!(!t, Flag::new(false));
    }

    #[test]
    fn test_delegated_bit_shifts() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct Bits(i32);

        let a = Bits::new(8);

        assert_eq!(a << 1, Bits::new(16));
        assert_eq!(a >> 1, Bits::new(4));
        assert_eq!(a << 2u8, Bits::new(32));
        assert_eq!(a >> 2u8, Bits::new(2));
    }

    #[test]
    fn test_delegated_with_scalable() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated", scalable)]
        struct Length(f64);

        let a = Length::new(10.0);

        assert_eq!(a * 2.0, Length::new(20.0));
        assert_eq!(a / 2.0, Length::new(5.0));
        assert_eq!(2.0 * a, Length::new(20.0));
    }

    #[test]
    fn test_delegated_assignment_operators() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct Value(i32);

        let mut a = Value::new(10);

        a += Value::new(5);
        assert_eq!(a, Value::new(15));

        a -= Value::new(3);
        assert_eq!(a, Value::new(12));

        a *= Value::new(2);
        assert_eq!(a, Value::new(24));

        a /= Value::new(4);
        assert_eq!(a, Value::new(6));
    }

    #[test]
    fn test_delegated_iterator_traits() {
        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct Amount(i32);

        let values = [Amount::new(1), Amount::new(2), Amount::new(3)];

        // Test Sum
        let sum: Amount = values.iter().copied().sum();
        assert_eq!(sum, Amount::new(6));

        // Test Product
        let product: Amount = values.iter().copied().product();
        assert_eq!(product, Amount::new(6));
    }

    #[test]
    fn test_multiple_delegated_types() {
        // This test verifies that multiple types can use delegated mode
        // and that the delegation functions are properly shared

        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct TypeA(i32);

        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct TypeB(i32);

        #[derive(StrongType)]
        #[strong_type(auto_operators = "delegated")]
        struct TypeC(i32);

        let a = TypeA::new(10);
        let b = TypeB::new(20);
        let c = TypeC::new(30);

        assert_eq!(a + a, TypeA::new(20));
        assert_eq!(b + b, TypeB::new(40));
        assert_eq!(c + c, TypeC::new(60));
    }
}
