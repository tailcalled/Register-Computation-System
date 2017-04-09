extern crate logic;

use logic::*;

fn main() {
    let mut config = Config::new();

    let clock = config.dummy_gate();
    config.modify_pins(clock, clock.negate(), clock.negate());

    let mut state1 = config.empty_state();
    let mut state2 = config.empty_state();

    config.step(&state1, &mut state2);
    println!("{}", state2.read(clock));
    config.step(&state2, &mut state1);
    println!("{}", state1.read(clock));

    config.step(&state1, &mut state2);
    println!("{}", state2.read(clock));
    config.step(&state2, &mut state1);
    println!("{}", state1.read(clock));

    config.step(&state1, &mut state2);
    println!("{}", state2.read(clock));
    config.step(&state2, &mut state1);
    println!("{}", state1.read(clock));

    println!("{:?}", config);

}

