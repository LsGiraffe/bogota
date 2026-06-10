use super::bus::mux16;
use super::elementary::or;
use super::mux::dmux;

pub fn or8way(input: [bool; 8]) -> bool {
    input.iter().copied().reduce(or).unwrap()
}

pub fn dmux4way(input: bool, sel: [bool; 2]) -> (bool, bool, bool, bool) {
    let (lo, hi) = dmux(input, sel[1]);
    let (a, b) = dmux(lo, sel[0]);
    let (c, d) = dmux(hi, sel[0]);
    (a, b, c, d)
}

pub fn dmux8way(input: bool, sel: [bool; 3]) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    let (lo, hi) = dmux(input, sel[2]);
    let (a, b, c, d) = dmux4way(lo, [sel[0], sel[1]]);
    let (e, f, g, h) = dmux4way(hi, [sel[0], sel[1]]);
    (a, b, c, d, e, f, g, h)
}

#[allow(clippy::too_many_arguments)]
pub fn mux4way16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    sel: [bool; 2],
) -> [bool; 16] {
    let ab = mux16(a, b, sel[0]);
    let cd = mux16(c, d, sel[0]);
    mux16(ab, cd, sel[1])
}

#[allow(clippy::too_many_arguments)]
pub fn mux8way16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    e: [bool; 16],
    f: [bool; 16],
    g: [bool; 16],
    h: [bool; 16],
    sel: [bool; 3],
) -> [bool; 16] {
    let abcd = mux4way16(a, b, c, d, [sel[0], sel[1]]);
    let efgh = mux4way16(e, f, g, h, [sel[0], sel[1]]);
    mux16(abcd, efgh, sel[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmux4way() {
        assert_eq!(
            dmux4way(false, [false, false]),
            (false, false, false, false)
        );
        assert_eq!(dmux4way(true, [false, false]), (true, false, false, false));
        assert_eq!(dmux4way(true, [true, false]), (false, true, false, false));
        assert_eq!(dmux4way(true, [false, true]), (false, false, true, false));
        assert_eq!(dmux4way(true, [true, true]), (false, false, false, true));
    }

    #[test]
    fn test_dmux8way() {
        assert_eq!(
            dmux8way(false, [false, false, false]),
            (false, false, false, false, false, false, false, false)
        );
        assert_eq!(
            dmux8way(true, [false, false, false]),
            (true, false, false, false, false, false, false, false)
        );
        assert_eq!(
            dmux8way(true, [true, false, false]),
            (false, true, false, false, false, false, false, false)
        );
        assert_eq!(
            dmux8way(true, [false, true, false]),
            (false, false, true, false, false, false, false, false)
        );
        assert_eq!(
            dmux8way(true, [true, true, false]),
            (false, false, false, true, false, false, false, false)
        );
        assert_eq!(
            dmux8way(true, [false, false, true]),
            (false, false, false, false, true, false, false, false)
        );
        assert_eq!(
            dmux8way(true, [true, false, true]),
            (false, false, false, false, false, true, false, false)
        );
        assert_eq!(
            dmux8way(true, [false, true, true]),
            (false, false, false, false, false, false, true, false)
        );
        assert_eq!(
            dmux8way(true, [true, true, true]),
            (false, false, false, false, false, false, false, true)
        );
    }

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

    #[test]
    fn test_mux4way16() {
        let a = [false; 16];
        let b = [true; 16];
        let c = [
            false, true, false, true, false, true, false, true, false, true, false, true, false,
            true, false, true,
        ];
        let d = [
            true, false, true, false, true, false, true, false, true, false, true, false, true,
            false, true, false,
        ];

        assert_eq!(mux4way16(a, b, c, d, [false, false]), a);
        assert_eq!(mux4way16(a, b, c, d, [true, false]), b);
        assert_eq!(mux4way16(a, b, c, d, [false, true]), c);
        assert_eq!(mux4way16(a, b, c, d, [true, true]), d);
    }

    #[test]
    fn test_mux8way16() {
        let a = [false; 16];
        let b = [true; 16];
        let c = [
            false, true, false, true, false, true, false, true, false, true, false, true, false,
            true, false, true,
        ];
        let d = [
            true, false, true, false, true, false, true, false, true, false, true, false, true,
            false, true, false,
        ];
        let e = [
            false, false, false, false, false, false, false, false, true, true, true, true, true,
            true, true, true,
        ];
        let f = [
            true, true, true, true, true, true, true, true, false, false, false, false, false,
            false, false, false,
        ];
        let g = [
            false, false, false, false, true, true, true, true, false, false, false, false, true,
            true, true, true,
        ];
        let h = [
            true, true, true, true, false, false, false, false, true, true, true, true, false,
            false, false, false,
        ];

        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, false, false]), a);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, false, false]), b);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, true, false]), c);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, true, false]), d);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, false, true]), e);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, false, true]), f);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, true, true]), g);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, true, true]), h);
    }
}
