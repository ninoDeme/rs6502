use std::fmt;
use std::io;

mod asm;
mod instruct;
mod m6502;
mod memory;

use crate::asm::{assemble, read_lines};
use crate::m6502::{step, State};
use crate::memory::{DefaultMemory, Memory};

fn main() -> io::Result<()> {
    let mut state = State::new();

    let mut memory = DefaultMemory::new();

    let lines: Vec<String> = read_lines("example2.asm")
        .unwrap()
        .map(|l| l.unwrap())
        .collect();
    let res = assemble(lines, 0x0600);

    memory.set(0xFFFC, 0x00);
    memory.set(0xFFFD, 0x00);

    let mut i = 0;
    for val in res {
        memory.set(0x0600 + i, val);
        i += 1;
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
