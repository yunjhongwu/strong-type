#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;
    use core::cmp::Ordering;
    use core::hash::{Hash, Hasher};
    use strong_type::StrongType;

    // A minimal hasher for testing Hash impl without std::collections
    struct SimpleHasher(u64);

    impl SimpleHasher {
        fn new() -> Self {
            Self(0)
        }

        fn finish_value(&self) -> u64 {
            self.0
        }
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.0
        }

        fn write(&mut self, bytes: &[u8]) {
            for byte in bytes {
                self.0 = self.0.wrapping_mul(31).wrapping_add(*byte as u64);
            }
        }
    }

    fn hash_value<T: Hash>(value: &T) -> u64 {
        let mut hasher = SimpleHasher::new();
        value.hash(&mut hasher);
        hasher.finish_value()
    }

    // ========== Primitive type tests (no alloc required) ==========

    #[test]
    fn test_no_std_integer_types() {
        #[derive(StrongType)]
        struct Count(i32);

        let a = Count::new(10);
        let b = Count::new(20);
        let c = Count::new(10);

        // Debug (uses core::fmt::Debug)
        let debug_str = alloc::format!("{:?}", a);
        assert!(debug_str.contains("Count"));

        // PartialEq (uses core::cmp::PartialEq)
        assert_eq!(a, c);
        assert_ne!(a, b);

        // PartialOrd (uses core::cmp::PartialOrd)
        assert!(a < b);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));

        // Ord (uses core::cmp::Ord)
        assert_eq!(a.cmp(&b), Ordering::Less);

        // Hash (uses core::hash::Hash)
        assert_eq!(hash_value(&a), hash_value(&c));

        // Copy (also tests Clone since Copy implies Clone)
        let copied = a;
        assert_eq!(a, copied);

        // Default (uses core::default::Default)
        let default = Count::default();
        assert_eq!(default.value(), 0);
    }

    #[test]
    fn test_no_std_unsigned_types() {
        #[derive(StrongType)]
        struct Size(usize);

        let a = Size::new(100usize);
        let b = Size::new(200usize);

        assert!(a < b);
        assert_eq!(Size::MIN.value(), usize::MIN);
        assert_eq!(Size::MAX.value(), usize::MAX);
        assert_eq!(Size::ZERO.value(), 0usize);
        assert_eq!(Size::ONE.value(), 1usize);
    }

    #[test]
    fn test_no_std_float_types() {
        #[derive(StrongType)]
        struct Distance(f64);

        let a = Distance::new(1.5);
        let b = Distance::new(2.5);

        assert!(a < b);
        assert!(Distance::NAN.is_nan());
        assert!(!Distance::INFINITY.is_finite());
        assert_eq!(Distance::ZERO.value(), 0.0);
    }

    #[test]
    fn test_no_std_bool_type() {
        #[derive(StrongType)]
        struct Flag(bool);

        let t = Flag::new(true);
        let f = Flag::new(false);

        assert_ne!(t, f);
        assert_eq!(hash_value(&t), hash_value(&Flag::new(true)));
    }

    #[test]
    fn test_no_std_char_type() {
        #[derive(StrongType)]
        struct Letter(char);

        let a = Letter::new('a');
        let b = Letter::new('b');

        assert!(a < b);
        assert_eq!(a.value(), 'a');
    }

    // ========== String type tests (requires alloc) ==========

    #[test]
    fn test_no_std_string_type() {
        #[derive(StrongType)]
        struct Name(String);

        let a = Name::new("Alice");
        let b = Name::new("Bob");
        let c = Name::new("Alice");

        // Debug
        let debug_str = alloc::format!("{:?}", a);
        assert!(debug_str.contains("Name"));

        // PartialEq
        assert_eq!(a, c);
        assert_ne!(a, b);

        // PartialOrd / Ord
        assert!(a < b);
        assert_eq!(a.cmp(&b), Ordering::Less);

        // Hash
        assert_eq!(hash_value(&a), hash_value(&c));
        assert_ne!(hash_value(&a), hash_value(&b));

        // Clone (not Copy)
        let cloned = a.clone();
        assert_eq!(a, cloned);

        // value() returns &str
        assert_eq!(a.value(), "Alice");
    }

    // ========== Operator tests (use core::ops::*) ==========

    #[test]
    fn test_no_std_arithmetic_operators() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        struct Value(i32);

        let a = Value::new(10);
        let b = Value::new(3);

        // core::ops::Add
        assert_eq!((a + b).value(), 13);

        // core::ops::Sub
        assert_eq!((a - b).value(), 7);

        // core::ops::Mul
        assert_eq!((a * b).value(), 30);

        // core::ops::Div
        assert_eq!((a / b).value(), 3);

        // core::ops::Rem
        assert_eq!((a % b).value(), 1);

        // core::ops::Neg
        assert_eq!((-a).value(), -10);
    }

    #[test]
    fn test_no_std_bitwise_operators() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        struct Bits(i32);

        let a = Bits::new(0b1100);
        let b = Bits::new(0b1010);

        // core::ops::BitAnd
        assert_eq!((a & b).value(), 0b1000);

        // core::ops::BitOr
        assert_eq!((a | b).value(), 0b1110);

        // core::ops::BitXor
        assert_eq!((a ^ b).value(), 0b0110);

        // core::ops::Shl / Shr
        let shift: i32 = 1;
        assert_eq!((a << shift).value(), 0b11000);
        assert_eq!((a >> shift).value(), 0b0110);
    }

    #[test]
    fn test_no_std_bool_operators() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        struct Logic(bool);

        let t = Logic::new(true);
        let f = Logic::new(false);

        // core::ops::BitAnd (logical AND)
        assert!(!(t & f).value());
        assert!((t & t).value());

        // core::ops::BitOr (logical OR)
        assert!((t | f).value());
        assert!(!(f | f).value());

        // core::ops::BitXor (logical XOR)
        assert!((t ^ f).value());
        assert!(!(t ^ t).value());

        // core::ops::Not
        assert!(!(!t).value());
        assert!((!f).value());
    }

    // ========== Iterator trait tests (use core::iter::*) ==========

    #[test]
    fn test_no_std_sum_iterator() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        struct Amount(i32);

        let values: Vec<Amount> = vec![Amount::new(1), Amount::new(2), Amount::new(3)];

        // core::iter::Sum
        let sum: Amount = values.into_iter().sum();
        assert_eq!(sum.value(), 6);
    }

    #[test]
    fn test_no_std_product_iterator() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        struct Factor(i32);

        let values: Vec<Factor> = vec![Factor::new(2), Factor::new(3), Factor::new(4)];

        // core::iter::Product
        let product: Factor = values.into_iter().product();
        assert_eq!(product.value(), 24);
    }

    // ========== AsRef/AsMut tests (use core::convert::*) ==========

    #[test]
    fn test_no_std_as_ref_as_mut() {
        #[derive(StrongType)]
        struct Counter(i32);

        let mut counter = Counter::new(5);

        // core::convert::AsRef
        let r: &i32 = counter.as_ref();
        assert_eq!(*r, 5);

        // core::convert::AsMut
        let m: &mut i32 = counter.as_mut();
        *m = 10;
        assert_eq!(counter.value(), 10);
    }

    // ========== Display test (uses core::fmt::Display) ==========

    #[test]
    fn test_no_std_display() {
        #[derive(StrongType)]
        struct Id(u64);

        let id = Id::new(42u64);

        // core::fmt::Display
        let display_str = alloc::format!("{}", id);
        assert_eq!(display_str, "Id(42)");
    }
}
