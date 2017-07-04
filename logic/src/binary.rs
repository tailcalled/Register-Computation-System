use {Pin, Config, State};
//use gate::AndGate;
use adder::{HalfAdder, FullAdder};

#[derive(Clone)]
pub struct BinaryPin {
    pub pins: Vec<Pin>
}

impl BinaryPin {
    pub fn wrap(pins: Vec<Pin>) -> BinaryPin {
        BinaryPin {
            pins: pins
        }
    }
    pub fn create_dummy_and_wrap(size: usize, config: &mut Config) -> BinaryPin {
        let mut pins = Vec::new();
        for _ in 0..size {
            pins.push(config.dummy_gate());
        }
        BinaryPin::wrap(pins)
    }
    pub fn size(&self) -> usize {
        self.pins.len()
    }
    pub fn negate(&mut self) {
        for pin in &mut self.pins {
            *pin = pin.negate();
        }
    }
    pub fn read_u8_le(&self, state: &State) -> u8 {
        let mut res = 0u8;
        let mut i = 0;
        for pin in &self.pins {
            if i == 8 { return res; }
            if state.read(*pin) { res += 1 << i; }
            i += 1;
        }
        res
    }
    pub fn read_u16_le(&self, state: &State) -> u16 {
        let mut res = 0u16;
        let mut i = 0;
        for pin in &self.pins {
            if i == 16 { return res; }
            if state.read(*pin) { res += 1 << i; }
            i += 1;
        }
        res
    }
    pub fn read_u32_le(&self, state: &State) -> u32 {
        let mut res = 0u32;
        let mut i = 0;
        for pin in &self.pins {
            if i == 32 { return res; }
            if state.read(*pin) { res += 1 << i; }
            i += 1;
        }
        res
    }
    pub fn read_u64_le(&self, state: &State) -> u64 {
        let mut res = 0u64;
        let mut i = 0;
        for pin in &self.pins {
            if i == 64 { return res; }
            if state.read(*pin) { res += 1 << i; }
            i += 1;
        }
        res
    }
    pub fn dummy_set_le(&self, config: &mut Config, mut v: u64) {
        let mut pins = self.pins.iter();
        while v != 0 {
            if let Some(pin) = pins.next() {
                config.set_dummy_value(*pin, v & 1 == 1);
                v = v >> 1;
            } else { return; }
        }
    }
}

pub struct BinaryAdder {
    output: BinaryPin
}
impl BinaryAdder {
    pub fn create(config: &mut Config, n1: &BinaryPin, n2: &BinaryPin) -> BinaryAdder {
        if n1.size() == 0 { return BinaryAdder { output: n2.clone() }; }
        if n2.size() == 0 { return BinaryAdder { output: n1.clone() }; }
        let mut out = Vec::with_capacity(if n1.size() > n2.size() { n1.size() + 1 } else { n2.size() + 1 });
        let mut iter = n1.pins.iter().zip(n2.pins.iter());
        if let Some((a, b)) = iter.next() {
            let ha = HalfAdder::create(config, *a, *b);
            out.push(ha.low());
            let mut carry = ha.high();
            for (x, y) in iter {
                let fa = FullAdder::create(config, *x, *y, carry);
                carry = fa.carry();
                out.push(fa.sum());
            }
            let skip = if n1.size() > n2.size() {
                n1.pins.iter().skip(n2.size())
            } else {
                n2.pins.iter().skip(n1.size())
            };
            for pin in skip {
                let ha = HalfAdder::create(config, *pin, carry);
                carry = ha.high();
                out.push(ha.low());
            }
            out.push(carry);
        }
        BinaryAdder { output: BinaryPin::wrap(out) }
    }
    pub fn pins(&self) -> &BinaryPin {
        &self.output
    }
}

#[cfg(test)]
mod tests {

    use binary::{BinaryPin, BinaryAdder};
    use {Config, Pin};
    extern crate rand;
    use self::rand::Rng;

    #[test]
    fn binary_adder_small() {
        let mut config = Config::new();
        let bin1 = BinaryPin::create_dummy_and_wrap(2, &mut config);
        let bin2 = BinaryPin::create_dummy_and_wrap(3, &mut config);
        let binadd = BinaryAdder::create(&mut config, &bin1, &bin2);
        assert_eq!(binadd.pins().size(), 4);

        let mut state1 = config.empty_state();
        let mut state2 = config.empty_state();

        let mut rng = rand::thread_rng();

        for _ in 0..512 {
            let r1: u8 = rng.gen();
            let r2: u8 = rng.gen();
            let r1: u8 = r1 & 3;
            let r2: u8 = r2 & 7;
            println!("testing {} + {}", r1, r2);
            bin1.dummy_set_le(&mut config, r1 as u64);
            bin2.dummy_set_le(&mut config, r2 as u64);
            while config.step_check_changes(&state1, &mut state2) {
                config.step(&state2, &mut state1);
            }
            let gate = binadd.pins().read_u32_le(&state2);
            if gate != r1 as u32 + r2 as u32 {
                panic!("addition failed:\n      {:04b}\n    + {:04b}\nreal: {:04b}\ngate: {:04b}",
                       r1, r2, r1 as u32 + r2 as u32, gate);
            }
        }
    }
    #[test]
    fn binary_adder() {
        let mut config = Config::new();
        let bin1 = BinaryPin::create_dummy_and_wrap(16, &mut config);
        let bin2 = BinaryPin::create_dummy_and_wrap(31, &mut config);
        let binadd = BinaryAdder::create(&mut config, &bin1, &bin2);
        assert_eq!(binadd.pins().size(), 32);

        let mut state1 = config.empty_state();
        let mut state2 = config.empty_state();

        let mut rng = rand::thread_rng();

        for _ in 0..512 {
            let r1: u16 = rng.gen();
            let r2: u32 = rng.gen();
            let r2: u32 = r2 & 0x7fffffff;
            println!("testing {} + {}", r1, r2);
            bin1.dummy_set_le(&mut config, r1 as u64);
            bin2.dummy_set_le(&mut config, r2 as u64);
            while config.step_check_changes(&state1, &mut state2) {
                config.step(&state2, &mut state1);
            }
            let gate = binadd.pins().read_u32_le(&state2);
            if gate != r1 as u32 + r2 {
                panic!("addition failed:\n      {:032b}\n    + {:032b}\nreal: {:032b}\ngate: {:032b}",
                       r1, r2, r1 as u32 + r2, gate);
            }
        }
    }
}

