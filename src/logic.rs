pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(a: bool) -> bool {
    nand(a, a)
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub fn xor(a: bool, b: bool) -> bool {
    nand(nand(not(a), b), nand(a, not(b)))
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(a, not(sel)), and(b, sel))
}

pub fn dmux(input: bool, sel: bool) -> [bool; 2] {
    // in sel | a  b
    // 0  0   | 0  0
    // 0  1   | 0  0
    // 1  0   | 1  0
    // 0  1   | 0  1
    [and(not(sel), input), and(sel, input)]
}

pub fn not16(a: [bool; 16]) -> [bool; 16] {
    let mut output = [false; 16];
    for i in 0..16 {
        output[i] = not(a[i]);
    }
    output
}

pub fn and16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut output = [false; 16];
    for i in 0..16 {
        output[i] = and(a[i], b[i]);
    }
    output
}

pub fn or16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut output = [false; 16];
    for i in 0..16 {
        output[i] = or(a[i], b[i]);
    }
    output
}

pub fn mux16(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
    let mut output = [false; 16];
    for i in 0..16 {
        output[i] = mux(a[i], b[i], sel);
    }
    output
}

pub fn or_8_way(input: [bool; 8]) -> bool {
    let mut output = false;
    for i in 0..7 {
        match or(input[i], input[i + 1]) {
            false => continue,
            true => {
                output = true;
                break;
            }
        }
    }
    output
}

pub fn bool_2_uint(a: bool) -> u32 {
    let output: u32;
    match a == true {
        false => output = 0,
        true => output = 1,
    }
    output
}

pub fn mux_4_way_16(input: [[bool; 16]; 4], sel: [bool; 2]) -> [bool; 16] {
    let i = 1 * bool_2_uint(sel[0]) + 2 * bool_2_uint(sel[1]);

    input[i as usize]
}

// pub fn mux_8_way_16(
//     a: [bool; 16],
//     b: [bool; 16],
//     d: [bool; 16],
//     e: [bool; 16],
//     f: [bool; 16],
//     g: [bool; 16],
//     h: [bool; 16],
//     i: [bool; 16],
//     sel:[bool; 4]) -> [bool; 16] {
//     for i in 0..16{
//         output[i] = and(and(and(sel[0], not(sel[1])), not(sel[2])), not(sel[3]));
//     }
// }

pub fn mux_8_way_16(input: [[bool; 16]; 8], sel: [bool; 3]) -> [bool; 16] {
    let mut ind: i32 = 0;
    for i in 0..3 {
        ind += 2_i32.pow(i) * bool_2_uint(sel[i as usize]) as i32;
    }

    input[ind as usize]
}

// pub fn dmux_8_way(input: bool, sel: [bool; 3]) -> [bool:8] {

// }

#[cfg(test)]
mod test_logic{
    use crate::logic::*;

    #[test]
    fn test_mux_8_way_16() {
        let mut input = [[false; 16]; 8];
        let sel = [false, false, true];
        input[4][7] = true;
        let mut output = [false; 16];
        output[7] = true;
        assert_eq!(mux_8_way_16(input, sel), output);
    }

    #[test]
    fn test_or_8_way() {
        let input = [false, false, false, false, false, true, false, true];
        assert_eq!(or_8_way(input), true);

        let input = [false, false, false, false, false, false, false, false];
        assert_eq!(or_8_way(input), false);
    }

    #[test]
    fn test_not16() {
        let input = [
            false, false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ];

        let output = [
            true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true,
        ];
        assert_eq!(not16(input), output);
    }

    #[test]
    fn test_nand() {
        assert_eq!(nand(false, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(true, true), false);
    }

    #[test]
    fn test_not() {
        assert_eq!(not(false), true);
        assert_eq!(not(true), false);
    }

    #[test]
    fn test_and() {
        assert_eq!(and(false, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(true, false), false);
        assert_eq!(and(true, true), true);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(false, false), false);
        assert_eq!(or(false, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(true, true), true);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(true, true), false);
    }

    #[test]
    fn test_mux() {
        assert_eq!(mux(false, false, false), false);
        assert_eq!(mux(false, true, false), false);
        assert_eq!(mux(true, false, false), true);
        assert_eq!(mux(true, true, false), true);
        assert_eq!(mux(false, false, true), false);
        assert_eq!(mux(false, true, true), true);
        assert_eq!(mux(true, false, true), false);
        assert_eq!(mux(true, true, true), true);
    }

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(false, false), [false, false]);
        assert_eq!(dmux(false, true), [false, false]);
        assert_eq!(dmux(true, false), [true, false]);
        assert_eq!(dmux(true, true), [false, true]);
    }
}