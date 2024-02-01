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
pub fn dmux(input:bool, sel:bool) -> [bool; 2]{
    [and(not(sel), input), and(sel, input)]
}

// pub fn not16(a:[bool; 16]) -> [bool; 16]{
//     not(a[0])
// }

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(mux(false, false,  false), false);
        assert_eq!(mux(false, true ,  false), false);
        assert_eq!(mux(true,  false , false), true);
        assert_eq!(mux(true,  true  , false), true);
        assert_eq!(mux(false, false,  true),  false);
        assert_eq!(mux(false, true ,  true),  true);
        assert_eq!(mux(true,  false , true),  false);
        assert_eq!(mux(true,  true  , true),  true);
    }

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(false, false), [false, false]);
        assert_eq!(dmux(false, true),  [false, false]);
        assert_eq!(dmux(true, false),  [true, false]);
        assert_eq!(dmux(true, true),   [false, true]);
    }
}
