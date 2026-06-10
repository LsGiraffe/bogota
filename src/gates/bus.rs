use super::elementary::{and, not, or};
use super::mux::mux;

pub fn not16(input: [bool; 16]) -> [bool; 16] {
    input.map(not)
}

pub fn and16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    std::array::from_fn(|i| and(a[i], b[i]))
}

pub fn or16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    std::array::from_fn(|i| or(a[i], b[i]))
}

pub fn mux16(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
    std::array::from_fn(|i| mux(a[i], b[i], sel))
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

    #[test]
    fn test_or16() {
        assert_eq!(or16([false; 16], [false; 16]), [false; 16]);
        assert_eq!(or16([false; 16], [true; 16]), [true; 16]);
        assert_eq!(or16([true; 16], [true; 16]), [true; 16]);
        assert_eq!(
            or16(
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
                true, true, false, true, true, true, false, true, true, true, false, true, true,
                true, false, true
            ],
        );
    }

    #[test]
    fn test_mux16() {
        assert_eq!(mux16([false; 16], [false; 16], false), [false; 16]);
        assert_eq!(mux16([false; 16], [false; 16], true), [false; 16]);
        assert_eq!(mux16([true; 16], [false; 16], false), [true; 16]);
        assert_eq!(mux16([true; 16], [false; 16], true), [false; 16]);
        assert_eq!(mux16([false; 16], [true; 16], false), [false; 16]);
        assert_eq!(mux16([false; 16], [true; 16], true), [true; 16]);
        assert_eq!(
            mux16(
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true
                ],
                [
                    true, true, false, false, true, true, false, false, true, true, false, false,
                    true, true, false, false
                ],
                false,
            ),
            [
                false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false, true
            ],
        );
        assert_eq!(
            mux16(
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true
                ],
                [
                    true, true, false, false, true, true, false, false, true, true, false, false,
                    true, true, false, false
                ],
                true,
            ),
            [
                true, true, false, false, true, true, false, false, true, true, false, false, true,
                true, false, false
            ],
        );
    }
}
