use {Pin, Config};
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
    pub fn size(&self) -> usize {
        self.pins.len()
    }
}

pub struct BinaryAdder {
    output: BinaryPin
}
impl BinaryAdder {
    pub fn create(config: &mut Config, n1: &BinaryPin, n2: &BinaryPin) -> BinaryAdder {
        if n1.size() == 0 { return BinaryAdder { output: n2.clone() }; }
        if n2.size() == 0 { return BinaryAdder { output: n1.clone() }; }
        let mut iter = n1.pins.iter().zip(n2.pins.iter());
        let mut out = Vec::with_capacity(if n1.size() > n2.size() { n1.size() + 1 } else { n2.size() + 1 });
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



