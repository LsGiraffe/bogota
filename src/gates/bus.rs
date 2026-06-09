use super::elementary::not;

pub fn not16(input: [bool; 16]) -> [bool; 16] {
    input.map(not)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not16() {
        assert_eq!(not16([false; 16]), [true; 16]);
        assert_eq!(not16([true; 16]), [false; 16]);
        assert_eq!(
            not16([
                false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false, true
            ]),
            [
                true, false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false
            ]
        );
    }
}
