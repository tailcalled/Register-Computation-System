use {Pin, Config};
use gate::AndGate;

pub struct HalfAdder {
    or: Pin,
    low: Pin,
    high: AndGate,
}
impl HalfAdder {
    pub fn create(config: &mut Config, p1: Pin, p2: Pin) -> HalfAdder {
        let high = config.and(p1, p2);
        let nand = high.pin().negate();
        let or = config.or(p1, p2);
        HalfAdder {
            high: high,
            or: or,
            low: config.and(nand, or).pin()
        }
    }
    pub fn modify_pins(&self, config: &mut Config, p1: Pin, p2: Pin) {
        config.modify_or_pins(self.or, p1, p2);
        config.modify_and_pins(self.high, p1, p2);
    }
    pub fn low(&self) -> Pin {
        self.low
    }
    pub fn high(&self) -> Pin {
        self.high.pin()
    }
}

pub struct FullAdder {
    a_and_b: AndGate,
    a_and_c: AndGate,
    b_and_c: AndGate,
    a_and_b_and_c: AndGate,
    nota_and_notb: AndGate,
    nota_and_notb_and_c: AndGate,
    nota_and_b: AndGate,
    nota_and_b_and_notc: AndGate,
    a_and_notb: AndGate,
    a_and_notb_and_notc: AndGate,

    carry: Pin,
    sum: Pin
}
impl FullAdder {
    pub fn create(config: &mut Config, a: Pin, b: Pin, c: Pin) -> FullAdder {
        let a_and_b = config.and(a, b);
        let a_and_c = config.and(a, c);
        let b_and_c = config.and(b, c);
        let a_and_b_and_c = config.and(a_and_b.pin(), c);
        let nota_and_notb = config.and(a.negate(), b.negate());
        let nota_and_notb_and_c = config.and(nota_and_notb.pin(), c);
        let nota_and_b = config.and(a.negate(), b);
        let nota_and_b_and_notc = config.and(nota_and_b.pin(), c.negate());
        let a_and_notb = config.and(a, b.negate());
        let a_and_notb_and_notc = config.and(a_and_notb.pin(), c.negate());

        let a_and_b_or_a_and_c = config.or(a_and_b.pin(), a_and_c.pin());
        let carry = config.or(a_and_b_or_a_and_c, b_and_c.pin());
        let u1_or_u2 = config.or(nota_and_notb_and_c.pin(), nota_and_b_and_notc.pin());
        let u3_or_u4 = config.or(a_and_notb_and_notc.pin(), a_and_b_and_c.pin());
        let sum = config.or(u1_or_u2, u3_or_u4);

        FullAdder {
            a_and_b: a_and_b,
            a_and_c: a_and_c,
            b_and_c: b_and_c,
            a_and_b_and_c: a_and_b_and_c,
            nota_and_notb: nota_and_notb,
            nota_and_notb_and_c: nota_and_notb_and_c,
            nota_and_b: nota_and_b,
            nota_and_b_and_notc: nota_and_b_and_notc,
            a_and_notb: a_and_notb,
            a_and_notb_and_notc: a_and_notb_and_notc,
            carry: carry,
            sum: sum
        }
    }
    pub fn modify_pins(&self, config: &mut Config, a: Pin, b: Pin, c: Pin) {
        config.modify_and_pins(self.a_and_b, a, b);
        config.modify_and_pins(self.a_and_c, a, c);
        config.modify_and_pins(self.b_and_c, b, c);
        config.modify_and_pins(self.a_and_b_and_c, self.a_and_b.pin(), c);
        config.modify_and_pins(self.nota_and_notb, a.negate(), b.negate());
        config.modify_and_pins(self.nota_and_notb_and_c, self.nota_and_notb.pin(), c);
        config.modify_and_pins(self.nota_and_b, a.negate(), b);
        config.modify_and_pins(self.nota_and_b_and_notc, self.nota_and_b.pin(), c.negate());
        config.modify_and_pins(self.a_and_notb, a, b.negate());
        config.modify_and_pins(self.a_and_notb_and_notc, self.a_and_notb.pin(), c.negate());
    }
    pub fn carry(&self) -> Pin {
        self.carry
    }
    pub fn sum(&self) -> Pin {
        self.sum
    }
}

#[cfg(test)]
mod tests {

    use adder::{FullAdder, HalfAdder};
    use {Config, Pin};

    fn half_adder_add(config: &mut Config, a: Pin, b: Pin, ha: &HalfAdder, av: bool, bv: bool) -> u32 {
        let mut state1 = config.empty_state();
        let mut state2 = config.empty_state();

        config.set_dummy_value(a, av);
        config.set_dummy_value(b, bv);

        config.step(&state1, &mut state2);
        config.step(&state2, &mut state1);
        config.step(&state1, &mut state2);
        config.step(&state2, &mut state1);
        config.step(&state1, &mut state2);

        let mut res = 0;
        if state2.read(ha.low()) { res += 1; }
        if state2.read(ha.high()) { res += 2; }
        res
    }
    #[test]
    fn half_adder() {
        let mut config = Config::new();

        let a = config.dummy_gate();
        let b = config.dummy_gate();
        let half_adder = HalfAdder::create(&mut config, a, b);

        assert_eq!(half_adder_add(&mut config, a, b, &half_adder, false, false), 0);
        assert_eq!(half_adder_add(&mut config, a, b, &half_adder, false, true ), 1);
        assert_eq!(half_adder_add(&mut config, a, b, &half_adder, true , false), 1);
        assert_eq!(half_adder_add(&mut config, a, b, &half_adder, true , true ), 2);
    }

    fn full_adder_add(config: &mut Config, a: Pin, b: Pin, c: Pin, fa: &FullAdder, av: bool, bv: bool, cv: bool) -> u32 {
        let mut state1 = config.empty_state();
        let mut state2 = config.empty_state();

        config.set_dummy_value(a, av);
        config.set_dummy_value(b, bv);
        config.set_dummy_value(c, cv);

        config.step(&state1, &mut state2);
        config.step(&state2, &mut state1);
        config.step(&state1, &mut state2);
        config.step(&state2, &mut state1);
        config.step(&state1, &mut state2);

        let mut res = 0;
        if state2.read(fa.sum()) { res += 1; }
        if state2.read(fa.carry()) { res += 2; }
        res
    }
    #[test]
    fn full_adder() {
        let mut config = Config::new();

        let a = config.dummy_gate();
        let b = config.dummy_gate();
        let c = config.dummy_gate();
        let full_adder = FullAdder::create(&mut config, a, b, c);

        assert_eq!(full_adder_add(&mut config, a, b, c, &full_adder, false, false, false), 0);
        assert_eq!(full_adder_add(&mut config, a, b, c, &full_adder, false, false, true ), 1);
        assert_eq!(full_adder_add(&mut config, a, b, c, &full_adder, false, true , false), 1);
        assert_eq!(full_adder_add(&mut config, a, b, c, &full_adder, true , false, false), 1);
        assert_eq!(full_adder_add(&mut config, a, b, c, &full_adder, false, true , true ), 2);
        assert_eq!(full_adder_add(&mut config, a, b, c, &full_adder, true , false, true ), 2);
        assert_eq!(full_adder_add(&mut config, a, b, c, &full_adder, true , true , false), 2);
        assert_eq!(full_adder_add(&mut config, a, b, c, &full_adder, true , true , true ), 3);
    }

}
