use super::elementary::or;

pub fn or8way(input: [bool; 8]) -> bool {
    input.iter().copied().reduce(or).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or8way() {
        assert_eq!(or8way([false; 8]), false);
        assert_eq!(or8way([true; 8]), true);
        assert_eq!(
            or8way([false, false, false, false, false, false, false, true]),
            true
        );
        assert_eq!(
            or8way([true, false, false, false, false, false, false, false]),
            true
        );
        assert_eq!(
            or8way([false, false, false, true, false, false, false, false]),
            true
        );
    }
}
