use crate::logic::{mux};

#[derive(Clone, Copy)]
pub struct Flipflop { bit: bool }
impl Flipflop {
    pub fn new() -> Self {
        Flipflop{ bit: false }
    }
    pub fn out(&self) -> bool {
        self.bit
    }
    pub fn clock(&mut self, a: bool){
        self.bit = a;
    }
}

#[derive(Clone, Copy)]
pub struct Bit { flipflop: Flipflop}
impl Bit {
    pub fn new() -> Self {
        Bit {flipflop: Flipflop::new()}
    }
    pub fn out(&self) -> bool {
        // if load(t-1) then out(t)=in(t-1)
        // else out(t)=out(t-1)
        self.flipflop.out()
    }
    pub fn clock(&mut self, input: bool, load: bool) {
        self.flipflop.clock(mux(self.out(), input, load))
    }
}

#[derive(Clone, Copy)]
pub struct Register { bits: [Bit; 16] }
impl Register {
    pub fn new() -> Self {
        Register {bits: [Bit::new(); 16]}
    }
    pub fn out(&self) -> [bool; 16] {
        let mut output = [false; 16];
        for i in 0..16{
            output[i] = self.bits[i].out();
        }
        output
    }
    pub fn clock(&mut self, input: [bool; 16], load: bool) {
        for i in 0..16{
            self.bits[i].clock(input[i], load)
        }
    }
}

// pub struct Ram8 {word: [Register; 8]}
// impl Ram8 {
//     pub fn new() -> Ram8{
//         Ram8 { word: [Register::new(); 8]}
//     }
//     pub fn out(address: [bool;3]) -> [bool; 16]{


//     }
//     pub fn clock(&mut self, input: [bool; 16], address: [bool; 3], load: bool){
//         let mut ind = 0;
//         for i in 0..3 {
//             ind += bool_2_uint(address[i]) * 2_u32.pow(i as u32);
//         }
//         self.word[ind as usize].clock(input, load);
//     }
// }

#[cfg(test)]
mod test_sequential {
    use crate::sequential::*;

    #[test]
    fn test_register() {
        let mut register = Register::new();
        register.clock([false; 16], false);
        assert_eq!(register.out(), [false; 16]);
        register.clock([true; 16], true);
        assert_eq!(register.out(), [true; 16]);
        register.clock([false; 16], false);
        assert_eq!(register.out(), [true; 16]);
        register.clock([false; 16], true);
        assert_eq!(register.out(), [false; 16]);
        register.clock([true; 16], false);
        assert_eq!(register.out(), [false; 16]);
    }

    #[test]
    fn test_bit() {
        let mut bit = Bit::new();
        bit.clock(false, false);
        assert_eq!(bit.out(), false);
        bit.clock(true, true);
        assert_eq!(bit.out(), true);
        bit.clock(false, false);
        assert_eq!(bit.out(), true);
        bit.clock(false, true);
        assert_eq!(bit.out(), false);
        bit.clock(true, false);
        assert_eq!(bit.out(), false);
    }
}