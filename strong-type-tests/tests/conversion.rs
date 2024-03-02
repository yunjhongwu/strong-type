#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use strong_type::StrongType;

    fn test<T: Debug + Eq>(value: T, underlying: T) {
        assert_eq!(value, underlying);
    }

    #[test]
    fn test_conversion() {
        #[derive(StrongType)]
        #[strong_type(conversion)]
        struct NamedI32(i64);

        let i64_value = 64;
        let value = NamedI32::new(i64_value);

        test(value, i64_value.into());
        test(value.into(), i64_value);

        #[derive(StrongType)]
        #[strong_type(conversion)]
        struct NamedString(String);
        let str_value = "test";
        let string_value = String::from(str_value);
        let value = NamedString::new(string_value.clone());

        test(value.clone(), str_value.into());
        test(value.clone(), string_value.clone().into());
        test(value.into(), string_value);
    }
}
