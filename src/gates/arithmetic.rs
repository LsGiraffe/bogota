use super::bus::{and16, mux16, not16};
use super::elementary::{and, or, xor};
use super::mux_multi::or8way;

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    let sum = xor(a, b);
    let carry = and(a, b);
    (sum, carry)
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, c);
    (sum2, or(carry1, carry2))
}

pub fn add16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut out = [false; 16];
    let (sum, mut carry) = half_adder(a[0], b[0]);
    out[0] = sum;
    for i in 1..16 {
        let (sum, next_carry) = full_adder(a[i], b[i], carry);
        out[i] = sum;
        carry = next_carry;
    }
    out
}

pub fn inc16(a: [bool; 16]) -> [bool; 16] {
    let mut one = [false; 16];
    one[0] = true;
    add16(a, one)
}

#[allow(clippy::too_many_arguments)]
pub fn alu(
    x: [bool; 16],
    y: [bool; 16],
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> ([bool; 16], bool, bool) {
    let x = mux16(x, [false; 16], zx);
    let x = mux16(x, not16(x), nx);

    let y = mux16(y, [false; 16], zy);
    let y = mux16(y, not16(y), ny);

    let fout = mux16(and16(x, y), add16(x, y), f);
    let out = mux16(fout, not16(fout), no);

    let ng = out[15];
    let zr = !or(
        or8way([
            out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7],
        ]),
        or8way([
            out[8], out[9], out[10], out[11], out[12], out[13], out[14], out[15],
        ]),
    );

    (out, zr, ng)
}

#[cfg(test)]
mod tests {
    use super::*;

    // All 18 canonical ALU operations from nand2tetris Figure 2.6
    // x=3, y=5 throughout so arithmetic is easy to verify by hand
    fn x() -> [bool; 16] {
        from_i16(3)
    }
    fn y() -> [bool; 16] {
        from_i16(5)
    }

    #[test]
    fn test_alu_zero() {
        let (out, zr, ng) = alu(x(), y(), true, false, true, false, true, false);
        assert_eq!(to_i16(out), 0);
        assert_eq!(zr, true);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_one() {
        let (out, zr, ng) = alu(x(), y(), true, true, true, true, true, true);
        assert_eq!(to_i16(out), 1);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_neg_one() {
        let (out, zr, ng) = alu(x(), y(), true, true, true, false, true, false);
        assert_eq!(to_i16(out), -1);
        assert_eq!(zr, false);
        assert_eq!(ng, true);
    }

    #[test]
    fn test_alu_x() {
        let (out, zr, ng) = alu(x(), y(), false, false, true, true, false, false);
        assert_eq!(to_i16(out), 3);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_y() {
        let (out, zr, ng) = alu(x(), y(), true, true, false, false, false, false);
        assert_eq!(to_i16(out), 5);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_not_x() {
        let (out, zr, ng) = alu(x(), y(), false, false, true, true, false, true);
        assert_eq!(to_i16(out), !3i16);
        assert_eq!(zr, false);
        assert_eq!(ng, true);
    }

    #[test]
    fn test_alu_not_y() {
        let (out, zr, ng) = alu(x(), y(), true, true, false, false, false, true);
        assert_eq!(to_i16(out), !5i16);
        assert_eq!(zr, false);
        assert_eq!(ng, true);
    }

    #[test]
    fn test_alu_neg_x() {
        let (out, zr, ng) = alu(x(), y(), false, false, true, true, true, true);
        assert_eq!(to_i16(out), -3);
        assert_eq!(zr, false);
        assert_eq!(ng, true);
    }

    #[test]
    fn test_alu_neg_y() {
        let (out, zr, ng) = alu(x(), y(), true, true, false, false, true, true);
        assert_eq!(to_i16(out), -5);
        assert_eq!(zr, false);
        assert_eq!(ng, true);
    }

    #[test]
    fn test_alu_x_plus_1() {
        let (out, zr, ng) = alu(x(), y(), false, true, true, true, true, true);
        assert_eq!(to_i16(out), 4);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_y_plus_1() {
        let (out, zr, ng) = alu(x(), y(), true, true, false, true, true, true);
        assert_eq!(to_i16(out), 6);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_x_minus_1() {
        let (out, zr, ng) = alu(x(), y(), false, false, true, true, true, false);
        assert_eq!(to_i16(out), 2);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_y_minus_1() {
        let (out, zr, ng) = alu(x(), y(), true, true, false, false, true, false);
        assert_eq!(to_i16(out), 4);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_x_plus_y() {
        let (out, zr, ng) = alu(x(), y(), false, false, false, false, true, false);
        assert_eq!(to_i16(out), 8);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_x_minus_y() {
        let (out, zr, ng) = alu(x(), y(), false, true, false, false, true, true);
        assert_eq!(to_i16(out), -2);
        assert_eq!(zr, false);
        assert_eq!(ng, true);
    }

    #[test]
    fn test_alu_y_minus_x() {
        let (out, zr, ng) = alu(x(), y(), false, false, false, true, true, true);
        assert_eq!(to_i16(out), 2);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_x_and_y() {
        let (out, zr, ng) = alu(x(), y(), false, false, false, false, false, false);
        assert_eq!(to_i16(out), 3 & 5);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_alu_x_or_y() {
        let (out, zr, ng) = alu(x(), y(), false, true, false, true, false, true);
        assert_eq!(to_i16(out), 3 | 5);
        assert_eq!(zr, false);
        assert_eq!(ng, false);
    }

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(false, false), (false, false));
        assert_eq!(half_adder(false, true), (true, false));
        assert_eq!(half_adder(true, false), (true, false));
        assert_eq!(half_adder(true, true), (false, true));
    }

    // Useful to write understandable numbers instead
    // of 16 bools in tests below
    fn from_i16(n: i16) -> [bool; 16] {
        let mut bits = [false; 16];
        for i in 0..16 {
            bits[i] = (n >> i) & 1 == 1;
        }
        bits
    }

    fn to_i16(bits: [bool; 16]) -> i16 {
        let mut n: i16 = 0;
        for i in 0..16 {
            if bits[i] {
                n |= 1 << i;
            }
        }
        n
    }

    #[test]
    fn test_add16() {
        assert_eq!(to_i16(add16(from_i16(0), from_i16(0))), 0);
        assert_eq!(to_i16(add16(from_i16(1), from_i16(1))), 2);
        assert_eq!(to_i16(add16(from_i16(100), from_i16(200))), 300);
        assert_eq!(to_i16(add16(from_i16(-1), from_i16(1))), 0);
        assert_eq!(to_i16(add16(from_i16(-1), from_i16(-1))), -2);
    }

    #[test]
    fn test_inc16() {
        assert_eq!(to_i16(inc16(from_i16(0))), 1);
        assert_eq!(to_i16(inc16(from_i16(1))), 2);
        assert_eq!(to_i16(inc16(from_i16(100))), 101);
        assert_eq!(to_i16(inc16(from_i16(-1))), 0);
    }

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(false, false, false), (false, false));
        assert_eq!(full_adder(false, false, true), (true, false));
        assert_eq!(full_adder(false, true, false), (true, false));
        assert_eq!(full_adder(false, true, true), (false, true));
        assert_eq!(full_adder(true, false, false), (true, false));
        assert_eq!(full_adder(true, false, true), (false, true));
        assert_eq!(full_adder(true, true, false), (false, true));
        assert_eq!(full_adder(true, true, true), (true, true));
    }
}
