use super::bus::mux16;
use super::elementary::or;

pub fn or8way(input: [bool; 8]) -> bool {
    input.iter().copied().reduce(or).unwrap()
}

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
