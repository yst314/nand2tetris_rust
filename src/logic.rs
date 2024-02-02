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

fn bool_2_uint(a: bool) -> u32 {
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
