use crate::logic::{and, and16, not16, or, xor, Word};

pub fn half_adder(a: bool, b: bool) -> [bool; 2] {
    let sum = xor(a, b);
    let carry = and(a, b);
    [sum, carry]
}

pub fn full_adder(a: bool, b: bool, c: bool) -> [bool; 2] {
    let sum = xor(xor(a, b), c);
    let carry = or(or(and(a, b), and(a, c)), and(b, c));
    [sum, carry]
}

pub fn add16(a: Word, b: Word) -> Word {
    let mut output = Word::new([false; 16]);
    let mut result = half_adder(a[0], b[0]);
    output[0] = result[0];
    for i in 1..16 {
        result = full_adder(a[i], b[i], result[1]);
        output[i] = result[0];
    }
    output
}

pub fn inc16(input: Word) -> Word {
    let mut one = Word::new([false; 16]);
    one[0] = true;
    add16(input, one)
}

pub fn alu(
    mut x: Word,
    mut y: Word,
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> (Word, bool, bool) {
    let mut output: Word;
    if zx {
        x = Word::new([false; 16]);
    }
    if nx {
        x = not16(x);
    }
    if zy {
        y = Word::new([false; 16]);
    }
    if ny {
        y = not16(y);
    }

    if f {
        output = add16(x, y);
    } else {
        output = and16(x, y);
    }

    if no {
        output = not16(output);
    }
    let zr = !output.iter().any(|i| i == true);

    let ng = output[15];
    (output, zr, ng)
}

#[cfg(test)]
mod test_arithmetic {
    use crate::arithmetic::*;

    #[test]
    fn test_alu() {
        // テストケース1: 基本的なAND演算
        let x = Word::new([true; 16]);
        let y = Word::new([false; 16]);
        let (result, zr, ng) = alu(x, y, false, false, false, false, false, false);
        assert_eq!(
            (result, zr, ng),
            (Word::new([false; 16]), true, false),
            "AND operation failed"
        );

        // テストケース2: 基本的なADD演算
        let mut x = Word::new([false; 16]);
        x[0] = true; // 1を表す
        let mut y = Word::new([false; 16]);
        y[1] = true; // 2を表す
        let (result, zr, ng) = alu(x, y, false, false, false, false, true, false);
        assert_eq!(
            (result, zr, ng),
            (
                Word::new([
                    true, true, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false
                ]),
                false,
                false
            )
        );

        // テストケース3: 出力の否定
        let x = Word::new([true; 16]);
        let y = Word::new([false; 16]);
        let (result, zr, ng) = alu(x, y, false, false, false, false, false, true);
        assert_eq!(
            (result, zr, ng),
            (Word::new([true; 16]), false, true),
            "NOT operation failed"
        );
    }
    #[test]
    fn test_inc16() {
        let a = Word::new([
            true, true, true, false, false, false, true, true, false, false, false, true, true,
            true, false, false,
        ]);
        let output = Word::new([
            false, false, false, true, false, false, true, true, false, false, false, true, true,
            true, false, false,
        ]);
        assert_eq!(inc16(a), output);
    }

    #[test]
    fn test_add16() {
        let a = Word::new([
            false, false, true, false, false, false, true, true, false, false, false, true, true,
            true, false, true,
        ]);
        let b = Word::new([
            true, false, true, false, false, false, true, false, false, false, false, true, true,
            false, false, false,
        ]);
        let output = Word::new([
            true, false, false, true, false, false, false, false, true, false, false, false, true,
            false, true, true,
        ]);
        assert_eq!(add16(a, b), output);
    }

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(false, false), [false, false]);
        assert_eq!(half_adder(false, true), [true, false]);
        assert_eq!(half_adder(true, false), [true, false]);
        assert_eq!(half_adder(true, true), [false, true]);
    }
    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(false, false, false), [false, false]);
        assert_eq!(full_adder(false, false, true), [true, false]);
        assert_eq!(full_adder(false, true, false), [true, false]);
        assert_eq!(full_adder(false, true, true), [false, true]);
        assert_eq!(full_adder(true, false, false), [true, false]);
        assert_eq!(full_adder(true, false, true), [false, true]);
        assert_eq!(full_adder(true, true, false), [false, true]);
        assert_eq!(full_adder(true, true, true), [true, true]);
    }
}
