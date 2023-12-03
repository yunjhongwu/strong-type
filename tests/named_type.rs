#[cfg(test)]
mod tests {
    use named_type::{NamedNumeric, NamedType};
    use std::any::Any;
    use std::collections::HashSet;

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
    fn test_hash() {
        #[derive(NamedType)]
        struct Sign(bool);

        let mut map = HashSet::<Sign>::new();
        map.insert(Sign(true));
        map.insert(Sign(false));
        map.insert(Sign(false));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_string() {
        #[derive(NamedType)]
        struct Tag(String);

        let mut map = HashSet::<Tag>::new();
        map.insert(Tag("dev".to_string()));
        map.insert(Tag("prod".to_string()));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_display() {
        #[derive(NamedNumeric)]
        #[default_display]
        struct Meter(i32);
        assert_eq!(format!("{}", Meter(-2)), "Meter(-2)");
        assert_eq!(format!("{:?}", Meter(-2)), "Meter { value: -2 }");
    }
}
