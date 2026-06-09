use super::elementary::{and, not, or};

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(a, not(sel)), and(b, sel))
}

pub fn dmux(input: bool, sel: bool) -> (bool, bool) {
    (and(input, not(sel)), and(input, sel))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mux() {
        assert_eq!(mux(false, false, false), false);
        assert_eq!(mux(false, false, true), false);
        assert_eq!(mux(false, true, false), false);
        assert_eq!(mux(false, true, true), true);
        assert_eq!(mux(true, false, false), true);
        assert_eq!(mux(true, false, true), false);
        assert_eq!(mux(true, true, false), true);
        assert_eq!(mux(true, true, true), true);
    }

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(false, false), (false, false));
        assert_eq!(dmux(false, true), (false, false));
        assert_eq!(dmux(true, false), (true, false));
        assert_eq!(dmux(true, true), (false, true));
    }
}
