use crate::logic;

#[test]
fn test_mux_8_way_16() {
    let mut input = [[false; 16]; 8];
    let sel = [false, false, true];
    input[4][7] = true;
    let mut output = [false; 16];
    output[7] = true;
    assert_eq!(logic::mux_8_way_16(input, sel), output);
}

#[test]
fn test_or_8_way() {
    let input = [false, false, false, false, false, true, false, true];
    assert_eq!(logic::or_8_way(input), true);

    let input = [false, false, false, false, false, false, false, false];
    assert_eq!(logic::or_8_way(input), false);
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
    assert_eq!(logic::not16(input), output);
}

#[test]
fn test_nand() {
    assert_eq!(logic::nand(false, false), true);
    assert_eq!(logic::nand(false, true), true);
    assert_eq!(logic::nand(true, false), true);
    assert_eq!(logic::nand(true, true), false);
}

#[test]
fn test_not() {
    assert_eq!(logic::not(false), true);
    assert_eq!(logic::not(true), false);
}

#[test]
fn test_and() {
    assert_eq!(logic::and(false, false), false);
    assert_eq!(logic::and(false, true), false);
    assert_eq!(logic::and(true, false), false);
    assert_eq!(logic::and(true, true), true);
}

#[test]
fn test_or() {
    assert_eq!(logic::or(false, false), false);
    assert_eq!(logic::or(false, true), true);
    assert_eq!(logic::or(true, false), true);
    assert_eq!(logic::or(true, true), true);
}

#[test]
fn test_xor() {
    assert_eq!(logic::xor(false, false), false);
    assert_eq!(logic::xor(false, true), true);
    assert_eq!(logic::xor(true, false), true);
    assert_eq!(logic::xor(true, true), false);
}

#[test]
fn test_mux() {
    assert_eq!(logic::mux(false, false, false), false);
    assert_eq!(logic::mux(false, true, false), false);
    assert_eq!(logic::mux(true, false, false), true);
    assert_eq!(logic::mux(true, true, false), true);
    assert_eq!(logic::mux(false, false, true), false);
    assert_eq!(logic::mux(false, true, true), true);
    assert_eq!(logic::mux(true, false, true), false);
    assert_eq!(logic::mux(true, true, true), true);
}

#[test]
fn test_dmux() {
    assert_eq!(logic::dmux(false, false), [false, false]);
    assert_eq!(logic::dmux(false, true), [false, false]);
    assert_eq!(logic::dmux(true, false), [true, false]);
    assert_eq!(logic::dmux(true, true), [false, true]);
}
