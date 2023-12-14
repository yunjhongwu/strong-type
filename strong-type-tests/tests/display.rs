#[cfg(test)]
mod tests {
    use std::fmt::Display;
    use strong_type::StrongType;

    #[test]
    fn test_display() {
        #[derive(StrongType)]
        struct Meter(i32);
        assert_eq!(format!("{}", Meter::new(-2)), "Meter(-2)");
        assert_eq!(format!("{:?}", Meter::new(-2)), "Meter { value: -2 }");

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
            format!("{:?}", Mile::new(std::f64::consts::E)),
            format!("Mile {{ value: {} }}", std::f64::consts::E)
        );
    }
}
