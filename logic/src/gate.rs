use {Pin,Config};

#[derive(Clone,Copy)]
pub struct AndGate(Pin);

impl Config {
    pub fn and(&mut self, p1: Pin, p2: Pin) -> AndGate {
        AndGate(self.or(p1.negate(), p2.negate()).negate())
    }
    pub fn nand(&mut self, p1: Pin, p2: Pin) -> AndGate {
        self.and(p1, p2).negate()
    }
    pub fn modify_and_pins(&mut self, gate: AndGate, p1: Pin, p2: Pin) {
        self.modify_or_pins(gate.0, p1.negate(), p2.negate());
    }
}
impl AndGate {
    pub fn negate(&self) -> AndGate {
        AndGate(self.0.negate())
    }
    pub fn pin(&self) -> Pin {
        self.0
    }
}

#[derive(Clone,Copy)]
pub struct XorGate {
    out: Pin,
    nand: AndGate,
    or: Pin
}

impl Config {
    pub fn xor(&mut self, p1: Pin, p2: Pin) -> XorGate {
        let nand = self.nand(p1, p2);
        let or = self.or(p1, p2);
        let out = self.and(nand.pin(), or).pin();
        XorGate {
            out: out, nand: nand, or: or
        }
    }
    pub fn xnor(&mut self, p1: Pin, p2: Pin) -> XorGate {
        self.xor(p1, p2).negate()
    }
    pub fn modify_xor_pins(&mut self, gate: XorGate, p1: Pin, p2: Pin) {
        self.modify_or_pins(gate.or, p1, p2);
        self.modify_and_pins(gate.nand, p1, p2);
    }
}
impl XorGate {
    pub fn negate(&self) -> XorGate {
        XorGate {
            out: self.out.negate(),
            nand: self.nand,
            or: self.or
        }
    }
    pub fn pin(&self) -> Pin {
        self.out
    }
}


