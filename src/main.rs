use std::io;

use rs6502::asm::{assemble, read_lines};
use rs6502::m6502::{step, State};
use rs6502::memory::{DefaultMemory, Memory};

fn main() -> io::Result<()> {
    let mut state = State::new();

    let mut memory = DefaultMemory::new();

    let lines: Vec<String> = read_lines("example.asm")
        .unwrap()
        .map(|l| l.unwrap())
        .collect();
    let res = assemble(lines);

    for (key, val) in res {
        memory.set(key, val);
    }

    step(&mut state);
    if state.rw {
        state.db = memory.get(state.ab);
    } else {
        memory.set(state.ab, state.db);
    }
    step(&mut state);

    println!("{state:?}");
    Ok(())
}

//
// fn adc()
