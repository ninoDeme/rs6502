use std::io;
use std::fmt;

mod instruct;
mod asm;

use crate::asm::{assemble, read_lines};
use crate::instruct::{Instruct, AddressType};

const NEGATIVE : u8 = 0b10000000;
const OVERFLOW : u8 = 0b01000000;
const _IGNORED : u8 = 0b00100000;
const BREAK    : u8 = 0b00010000;
const DECIMAL  : u8 = 0b00001000;
const INTERRUPT: u8 = 0b00000100;
const ZERO     : u8 = 0b00000010;
const CARRY    : u8 = 0b00000001;

#[derive(fmt::Debug)]
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
            //    nv-bdizc
            sr: 0b00000110,
            sp: 0xFD
        }
    }
}

#[derive(Clone)]
struct TimingState {
    t0: bool,
    tp: bool,
    t2: bool,
    t3: bool,
    t4: bool,
    t5: bool,
    t1: bool,
    t6: bool,
    v0: bool,
    sd1: bool,
    sd2: bool,
}

impl TimingState {
    pub fn new() -> TimingState {
        TimingState {
            t0: false,
            tp: false,
            t2: false,
            t3: false,
            t4: false,
            t5: false,
            t1: false,
            t6: false,
            v0: false,
            sd1: false,
            sd2: false,
        }
    }

    pub fn clear(&mut self) -> () {
        self.t0 = false;
        self.tp = false;
        self.t2 = false;
        self.t3 = false;
        self.t4 = false;
        self.t5 = false;
        self.t1 = false;
        self.t6 = false;
        self.v0 = false;
        self.sd1 = false;
        self.sd2 = false;
    }
}

impl fmt::Debug for TimingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        if self.t0 {
            write!(f, "T0")?;
            first = false;
        }
        if self.t1 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "T1")?;
            first = false;
        }
        if self.t2 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "T2")?;
            first = false;
        }
        if self.t3 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "T3")?;
            first = false;
        }
        if self.t4 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "T4")?;
            first = false;
        }
        if self.t6 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "T5")?;
            first = false;
        }
        if self.v0 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "VEC0")?;
            first = false;
        }
        if self.sd1 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "SD1")?;
            first = false;
        }
        if self.sd2 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "SD2")?;
            first = false;
        }
        Ok(())
    }
}

trait Memory: fmt::Debug {
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
    pub total_cycles: i32,

    pub ab: u16,
    pub rw: bool,

    // current Instruction
    pub ir: u8,
    // fetched instruction
    pub pd: u8,

    clock1: bool,
    clock2: bool,

    pub timing: TimingState,

    // sync output
    pub sync: bool,

    // interrupt inputs
    pub res: bool,
    pub irq: bool,
    pub nmi: bool,

}

fn main() -> io::Result<()> {
    let mut state = State {
        registers: Registers::new(),
        total_cycles: 0,

        ir: 0,
        pd: 0,

        ab: 0,
        db: 0,

        clock1: false,
        clock2: false,

        timing: TimingState::new(),

        sync: false,

        res: false,
        irq: false,
        nmi: false,
    };

    let memory = DefaultMemory::new();
    
    let lines: Vec<String> = read_lines("example2.asm").unwrap().map(|l| l.unwrap()).collect();
    let res = assemble(lines);

    memory.set(0xFFFC, 0x00);
    memory.set(0xFFFD, 0x00);

    let mut i = 0;
    for val in res {
        memory.set(0x0600 + i, val);
        i += 1;
    }


    step1(&mut state);
    if state.rw {
        state.db = memory.get(state.ab);
    } else {
        memory.set(state.ab, state.db);
    }
    step2(&mut state);

    println!("{state:?}");
    Ok(())
}


fn check_res(state: &mut State) -> bool {
    if state.res {
        state.timing.clear();
        state.timing.t0 = true;
        state.registers.sr &= !BREAK
        return true;
    };
    return false;
}

fn step1(state: &mut State) {
    if state.registers.sp & BREAK != BREAK {
        if state.timing.t0 {
        } else if state.timing.t1 {
        } else if state.timing.t2 {
        } else if state.timing.t3 {
        } else if state.timing.t4 {
        } else if state.timing.t5 {
        } else if state.timing.t6 {
            state.db = 0xFFFC;
            state.rw = 1;
        } else {
            panic!("Invalid Timing State");
        };
    } else {
        if state.timing.t2 {
            state.ir = state.pd;
        };
        let op_code = state.ir; 
        let (ins, addr_type) = Instruct::from_op_code(op_code).unwrap();
        if state.timin.t0 && check_res(&mut state) {
            return;
        }
        match addr_type {
            AddressType::Immediate => {
                match ins {
                    Instruct::ADC => {
                        if state.timing.t1 {
                            state.ab = state.registers + 2;
                        };
                        if state.timing.t2 {
                            state.ab = state.registers.pc + 1;
                        };
                    },
                    _ => unimplemented!()
                };
            },
            _ => unimplemented!()
        };
    };
}

fn step2(state: &mut State) {
    state.pd = state.db
    if state.registers.sp & BREAK != BREAK {
        if state.timing.t0 {
            if state.res {
                state.timing.clear();
                state.timing.t1 = true;
            } else {
                state.timing.clear();
                state.timing.t0 = true;
                state.timing.t1 = true;
            };
        } else if state.timing.t1 {
            state.ir = 0;
            state.registers.sp = 0;
            state.timing.clear();
            state.timing.t2 = true;
        } else if state.timing.t2 {
            state.timing.clear();
            state.timing.t3 = true;
        } else if state.timing.t3 {
            state.timing.clear();
            state.timing.t4 = true;
        } else if state.timing.t4 {
            state.timing.clear();
            state.timing.t5 = true;
        } else if state.timing.t5 {
            state.timing.clear();
            state.timing.t6 = true;
        } else if state.timing.t6 {
            state.registers.sp |= BREAK;
            state.timing.clear();
            state.timing.t0 = true;
        } else {
            panic!("Invalid Timing State");
        };
    } else {
        if (state.timing.t0 || state.timing.t1) && state.res {
            state.timing.clear();
            state.registers.sr &= !BREAK
        }
        if state.timing.t2 {
            state.ir = state.pd;
        };
        let op_code = state.ir; 
        let (ins, addr_type) = Instruct::from_op_code(op_code).unwrap();
        match addr_type {
            AddressType::Immediate => {
                match ins {
                    Instruct::ADC => {
                        if state.timing.t1 {
                            state.timing
                        };
                        if state.timing.t2 {
                            let value = db;
                            state.registers.pc += 1;
                            if state.registers.sr & CARRY == CARRY {
                                let (value, carry) = u8::overflowing_add(state.registers.ac, 1);
                                state.registers.ac = value;
                                if carry {
                                    state.registers.sr |= CARRY;
                                } else {
                                    state.registers.sr &= !CARRY;
                                };
                            }
                            let (result, carry) = u8::overflowing_add(state.registers.ac, value);
                            state.registers.ac = result;
                            if carry {
                                state.registers.sr |= CARRY;
                            };
                            state.registers.pc += 1;
                        };
                        if state.timing.t0 {
                            state.timing.clear();
                            state.timing.t1 = true;
                        };
                    },
                    _ => unimplemented!()
                };
            },
            _ => unimplemented!()
        };
    };
    state.total_cycles += 1;
}
//
// fn adc()
