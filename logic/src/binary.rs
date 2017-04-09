use {Pin, Config};
use gate::{AndGate, XorGate};

struct BinaryPin {
    pins: Vec<Pin>
}

impl BinaryPin {
    fn wrap(pins: Vec<Pin>) -> BinaryPin {
        BinaryPin {
            pins: pins
        }
    }
    fn size(&self) -> usize {
        self.pins.len()
    }
}

struct HalfAdder {
    low: XorGate,
    high: AndGate
}
impl HalfAdder {
    fn create(config: &mut Config, p1: Pin, p2: Pin) -> HalfAdder {
        let low = config.xor(p1, p2);
        let high = config.and(p1, p2);
        HalfAdder {
            low: low, high: high
        }
    }
    fn modify_pins(&self, config: &mut config, p1: Pin, p2: Pin) {
        config.modify_xor_pins(self.low, p1, p2);
        config.modify_and_pins(self.high, p1, p2);
    }
    fn low(&self) -> Pin {
        self.low.pin()
    }
    fn high(&self) -> Pin {
        self.high.pin()
    }
}

struct FullAdder {
    low: HalfAdder
}
impl FullAdder {
    fn create(config: &mut Config, p1: Pin, p2: Pin, p3: Pin) -> HalfAdder {
        let low = config.xor(p1, p2);
        let high = config.and(p1, p2);
        HalfAdder {
            low: low, high: high
        }
    }
    fn modify_pins(&self, config: &mut config, p1: Pin, p2: Pin) {
        config.modify_xor_pins(self.low, p1, p2);
        config.modify_and_pins(self.high, p1, p2);
    }
    fn low(&self) -> Pin {
        self.low.pin()
    }
    fn high(&self) -> Pin {
        self.high.pin()
    }
}

