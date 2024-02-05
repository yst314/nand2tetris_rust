use crate::logic::{and, and16, not16, or, xor};

pub fn half_adder(a: bool, b: bool) -> [bool; 2] {
    let sum = xor(a, b);
    let carry = and(a, b);
    [sum, carry]
}

pub fn full_adder(a: bool, b: bool, c: bool) -> [bool; 2] {
    let sum = xor(xor(a,b), c);
    let carry = or(or(and(a,b), and(a,c)), and(b,c));
    [sum, carry]
}

pub fn add16(a:[bool;16], b:[bool;16]) -> [bool; 16]{
    let mut output = [false; 16];
    let mut result = half_adder(a[0], b[0]);
    output[0] = result[0];
    for i in 1..16{
        result = full_adder(a[i], b[i], result[1]);
        output[i] = result[0];
    }
    output
}

pub fn inc16(input:[bool;16]) -> [bool;16]{
    let mut one = [false; 16];
    one[0] = true;
    add16(input, one)
}

pub fn alu(mut x:[bool;16], mut y:[bool;16], zx:bool, nx:bool, zy:bool, ny:bool, f:bool, no:bool)->([bool;16], bool, bool){
    let mut output:[bool;16];
    if zx{
        x = [false; 16];
    } 
    if nx{
        x = not16(x);
    } 
    if zy{
        y = [false; 16];
    } 
    if ny{
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
    let zr = !output.iter().any(|&i| i ==true);
    
    let ng = output[15];
    (output, zr, ng)
}


