use std::fmt;
pub mod gate;
pub mod binary;
pub mod adder;

struct Gate {
    in1: usize,
    negate1: bool,
    in2: usize,
    negate2: bool
}
pub struct Config {
    config: Vec<Gate>
}
pub struct State {
    state: Vec<bool>,
}

#[derive(Clone,Copy)]
pub struct Pin(usize, bool);

impl Config {
    pub fn new() -> Config {
        Config {
            config: Vec::new()
        }
    }
    pub fn step(&self, state: &State, store: &mut State) {
        store.state[0] = false;
        for (gate, out) in self.config.iter().zip(store.state.iter_mut().skip(1)) {
            *out = (state.state[gate.in1] ^ gate.negate1)
                 | (state.state[gate.in2] ^ gate.negate2);
        }
    }
    pub fn step_check_changes(&self, state: &State, store: &mut State) -> bool {
        store.state[0] = false;
        let mut changed = false;
        for (gate, out) in self.config.iter().zip(store.state.iter_mut().skip(1)) {
            let prev_out = *out;
            let result = (state.state[gate.in1] ^ gate.negate1)
                       | (state.state[gate.in2] ^ gate.negate2);
            if prev_out != result { changed = true; }
            *out = result;
        }
        changed
    }
    pub fn empty_state(&self) -> State {
        State { state: vec![false; self.config.len() + 1] }
    }
    pub fn true_pin(&self) -> Pin {
        Pin(0, true)
    }
    pub fn false_pin(&self) -> Pin {
        Pin(0, false)
    }
    pub fn or(&mut self, p1: Pin, p2: Pin) -> Pin {
        self.config.push(Gate {
            in1: p1.0,
            negate1: p1.1,
            in2: p2.0,
            negate2: p2.1
        });
        Pin(self.config.len(), false)
    }
    pub fn nor(&mut self, p1: Pin, p2: Pin) -> Pin {
        self.or(p1,p2).negate()
    }
    pub fn modify_or_pins(&mut self, gate: Pin, p1: Pin, p2: Pin) {
        self.config[gate.config_index()] = Gate {
            in1: p1.0,
            negate1: p1.1,
            in2: p2.0,
            negate2: p2.1
        };
    }
    pub fn dummy_gate(&mut self) -> Pin {
        let p1 = self.false_pin();
        let p2 = self.false_pin();
        self.or(p1, p2)
    }
    pub fn set_dummy_value(&mut self, pin: Pin, value: bool) {
        self.modify_or_pins(pin, Pin(0, value), Pin(0, value));
    }
}
impl Pin {
    pub fn negate(&self) -> Pin {
        Pin(self.0, !self.1)
    }
    pub fn config_index(&self) -> usize {
        self.0 - 1
    }
}
impl State {
    pub fn read(&self, pin: Pin) -> bool {
        self.state[pin.0] ^ pin.1
    }
}

impl fmt::Debug for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.negate1 {
            try!(write!(f, "!"));
        }
        try!(write!(f, "{} | ", self.in1));
        if self.negate2 {
            try!(write!(f, "!"));
        }
        write!(f, "{}", self.in2)
    }
}
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.config)
    }
}

