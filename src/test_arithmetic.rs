use crate::arithmetic::*;


#[test]
fn test_alu() {
    // テストケース1: 基本的なAND演算
    let x = [true; 16];
    let y = [false; 16];
    let (result, zr, ng) = alu(x, y, false, false, false, false, false, false);
    assert_eq!((result, zr, ng), ([false; 16], true, false), "AND operation failed");

    // テストケース2: 基本的なADD演算
    let mut x = [false; 16];
    x[0] = true; // 1を表す
    let mut y = [false; 16];
    y[1] = true; // 2を表す
    let (result, zr, ng) = alu(x, y, false, false, false, false, true, false);
    assert_eq!((result, zr, ng), ([true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false], false, false));

    // テストケース3: 出力の否定
    let x = [true; 16];
    let y = [false; 16];
    let (result, zr, ng) = alu(x, y, false, false, false, false, false, true);
    assert_eq!((result, zr, ng), ([true; 16], false, true), "NOT operation failed");

}
#[test]
fn test_inc16(){
    let a =         [true,true,true,false,false,false,true,true,false,false,false,true,true,true,false,false];
    let output =    [false,false,false,true,false,false,true,true,false,false,false,true,true,true,false,false];
    assert_eq!(inc16(a), output);
}

#[test]
fn test_add16() {
    let a =         [false,false,true,false,false,false,true,true,false,false,false,true,true,true,false,true];
    let b =         [true,false,true,false,false,false,true,false,false,false,false,true,true,false,false,false];
    let output =    [true,false,false,true,false,false,false,false,true,false,false,false,true,false,true,true];
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
    assert_eq!(full_adder(false, false, true),  [true, false]);
    assert_eq!(full_adder(false, true, false),  [true, false]);
    assert_eq!(full_adder(false, true, true),   [false, true]);
    assert_eq!(full_adder(true, false, false),  [true, false]);
    assert_eq!(full_adder(true, false, true),   [false, true]);
    assert_eq!(full_adder(true, true, false),   [false, true]);
    assert_eq!(full_adder(true, true, true),    [true, true]);
}
