#[cfg(test)]
mod tests {
    use std::mem;
    use strong_type::StrongType;

    fn test_type<T: std::fmt::Debug + Clone + Send + Sync + Default + PartialEq>() {}

    #[test]
    fn test_custom_underlying() {
        #[derive(StrongType)]
        #[strong_type(auto_operators)]
        struct Dollar(i32);

        #[derive(StrongType)]
        #[strong_type(auto_operators, underlying=i32)]
        struct Cash(Dollar);
        test_type::<Cash>();
        assert_eq!(mem::size_of::<Cash>(), mem::size_of::<i32>());

        assert_eq!(
            Cash::new(Dollar::new(10)),
            Cash::new(Dollar::new(2)) + Cash::new(Dollar::new(8))
        );
        assert_eq!(Cash::new(Dollar::new(10)).primitive(), 10);

        assert_eq!(
            format!("{}", Cash::new(Dollar::new(10))),
            "Cash(Dollar(10))"
        );

        #[derive(StrongType)]
        #[strong_type(underlying=i32)]
        struct Coin(Cash);
        test_type::<Coin>();
        assert_eq!(mem::size_of::<Coin>(), mem::size_of::<i32>());
        assert_eq!(
            format!("{}", Coin::new(Cash::new(Dollar::new(10)))),
            "Coin(Cash(Dollar(10)))"
        );
        assert_eq!(
            Coin::new(Cash::new(Dollar::new(10))).value(),
            Cash::new(Dollar::new(10))
        );
        assert_eq!(Coin::new(Cash::new(Dollar::new(10))).primitive(), 10);
    }

    #[test]
    fn test_custom_string_underlying_with() {
        #[derive(StrongType)]
        struct Tag(String);

        #[derive(StrongType)]
        #[strong_type(underlying=String)]
        struct Name(Tag);

        test_type::<Name>();
        assert_eq!(mem::size_of::<Name>(), mem::size_of::<String>());
        assert_eq!(
            format!("{}", Name::new(Tag::new("tag".to_string()))),
            "Name(Tag(tag))"
        );

        #[derive(StrongType)]
        #[strong_type(underlying=String)]
        struct Surname(Name);
        assert_eq!(mem::size_of::<Surname>(), mem::size_of::<String>());
        assert_eq!(
            format!("{}", Surname::new(Name::new(Tag::new("tag".to_string())))),
            "Surname(Name(Tag(tag)))"
        );
    }
}
