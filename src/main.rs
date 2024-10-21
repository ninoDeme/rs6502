mod instruct;
use crate::instruct::{Instruct, AddressType};
use std::fmt::Debug;

const NEGATIVE : u8 = 0b10000000;
const OVERFLOW : u8 = 0b01000000;
const _IGNORED  : u8 = 0b00100000;
const BREAK    : u8 = 0b00010000;
const DECIMAL  : u8 = 0b00001000;
const INTERRUPT: u8 = 0b00000100;
const ZERO     : u8 = 0b00000010;
const CARRY    : u8 = 0b00000001;

#[derive(Debug)]
struct Registers {
    // Program Counter
    pub pc: u16,
    // Accumulator
    pub ac: u8,
    // X Register
    pub xr: u8,
    // Y Register
    pub yr: u8,
    // Status register [NV-BDIZC]
    pub sr: u8,
    // Stack pointer
    pub sp: u8
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            pc: 0x00,
            ac: 0x00,
            xr: 0x00,
            yr: 0x00,
            sr: 0x00,
            sp: 0xFD
        }
    }
}

trait Memory: Debug {
    fn get(&mut self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, value: u8) -> ();
}

#[derive(Debug)]
struct DefaultMemory {
    memory: [u8; 65536],
}

impl DefaultMemory {
    pub fn new() -> Self {
        DefaultMemory {
            memory: [0; 65536]
        }
    }
}

impl Memory for DefaultMemory {
    fn get(&mut self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }
    fn set(&mut self, addr: u16, value: u8) -> () {
        self.memory[addr as usize] = value;
    }
}

#[derive(Debug)]
struct State {
    pub registers: Registers,
    pub current_cycle: i32,
    pub total_cycles: i32,
    pub current_op: u8,
    pub memory: Box<dyn Memory>,
}

fn main() {
    let mut state = State {
        registers: Registers::new(),
        current_cycle: 0,
        total_cycles: 0,
        current_op: 0,
        memory: Box::new(DefaultMemory::new())
    };

    
    state.memory.set(0x0600, 0x69);
    state.memory.set(0x0601, 0x55);

    state.registers.pc = 0x0600;

    step(&mut state);
    step(&mut state);

    // println!("{state:?}");
}

fn step(state: &mut State) {
    match state.total_cycles {
        0 => {
            state.registers.sr = 0b00010110;
        },
        1..6 => {
            ();
        },
        7 => {
            let high: u16 = (state.memory.get(0xfffd) as u16) << 8;
            let low = state.memory.get(0xfffc) as u16;
            state.registers.pc = low + high
        }
        _ => {
            let op_code = if state.current_cycle == 0 {
                let op = state.memory.get(state.registers.pc);
                state.current_op = op;
                op
            } else {
                state.current_op
            };
            let (ins, addr_type) = Instruct::from_op_code(op_code).unwrap();
            match addr_type {
                AddressType::Immediate => {
                    match state.current_cycle {
                        0 => {
                            match ins {
                                Instruct::ADC => {
                                    if state.registers.sr & CARRY == CARRY {
                                        let (result, carry) = u8::overflowing_add(state.registers.ac, 1);
                                        state.registers.ac = result;
                                        if carry {
                                            state.registers.sr |= CARRY;
                                        } else {
                                            state.registers.sr &= !CARRY;
                                        };
                                    }
                                },
                                _ => unimplemented!()
                            }
                        },
                        1 => {
                            let value = state.memory.get(state.registers.pc + 1);
                            match ins {
                                Instruct::ADC => {
                                    let (result, carry) = u8::overflowing_add(state.registers.ac, value);
                                    println!("{result}");
                                    state.registers.ac = result;
                                    if carry {
                                        state.registers.sr |= CARRY;
                                    };
                                    state.registers.pc += 2;
                                    state.current_cycle = 0;
                                },
                                _ => unimplemented!()
                            }
                        },
                        _ => unreachable!()
                    };
                },
                _ => unimplemented!()
            };
        }
    };
    state.total_cycles += 1;
    state.current_cycle += 1;
}
//
// fn adc()
