#[cfg(test)]
mod tests {
    use trybuild::TestCases;

    #[test]
    fn test_unknown_attribute_has_clear_error() {
        let t = TestCases::new();
        t.compile_fail("tests/ui/invalid_attribute.rs");
    }

    #[test]
    fn test_invalid_underlying_path_is_reported() {
        let t = TestCases::new();
        t.compile_fail("tests/ui/unsupported_underlying.rs");
    }
}
