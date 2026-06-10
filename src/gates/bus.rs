use super::elementary::{and, not};

pub fn not16(input: [bool; 16]) -> [bool; 16] {
    input.map(not)
}

pub fn and16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    std::array::from_fn(|i| and(a[i], b[i]))
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

    #[test]
    fn test_and16() {
        assert_eq!(and16([false; 16], [false; 16]), [false; 16]);
        assert_eq!(and16([true; 16], [false; 16]), [false; 16]);
        assert_eq!(and16([true; 16], [true; 16]), [true; 16]);
        assert_eq!(
            and16(
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true
                ],
                [
                    true, true, false, false, true, true, false, false, true, true, false, false,
                    true, true, false, false
                ],
            ),
            [
                false, true, false, false, false, true, false, false, false, true, false, false,
                false, true, false, false
            ],
        );
    }
}
