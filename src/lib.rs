pub fn greeting() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        assert_eq!(greeting(), "Hello, world!");
    }
}
