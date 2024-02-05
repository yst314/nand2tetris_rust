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