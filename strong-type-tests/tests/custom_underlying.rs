#[cfg(test)]
mod tests {
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
        assert_eq!(size_of::<Cash>(), size_of::<i32>());

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
        assert_eq!(size_of::<Coin>(), size_of::<i32>());
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
        assert_eq!(size_of::<Name>(), size_of::<String>());
        assert_eq!(
            format!("{}", Name::new(Tag::new("tag".to_string()))),
            "Name(Tag(tag))"
        );

        #[derive(StrongType)]
        #[strong_type(underlying=String)]
        struct Surname(Name);
        assert_eq!(size_of::<Surname>(), size_of::<String>());
        assert_eq!(
            format!("{}", Surname::new(Name::new(Tag::new("tag".to_string())))),
            "Surname(Name(Tag(tag)))"
        );
    }

    #[test]
    fn test_custom_underlying_accepts_primitive_paths() {
        #[derive(StrongType)]
        struct Credits(core::primitive::i64);

        #[derive(StrongType)]
        #[strong_type(underlying=core::primitive::i64)]
        struct Ledger(Credits);

        assert_eq!(
            core::mem::size_of::<Ledger>(),
            core::mem::size_of::<core::primitive::i64>()
        );
        assert_eq!(Ledger::new(Credits::new(42)).primitive(), 42);
    }
}
