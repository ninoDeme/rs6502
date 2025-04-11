use crate::instruct::{Instruct, AddressType, InstructionInfo};
use std::fmt;

const NEGATIVE : u8 = 0b10000000;
const OVERFLOW : u8 = 0b01000000;
const _IGNORED : u8 = 0b00100000;
const BREAK    : u8 = 0b00010000;
const DECIMAL  : u8 = 0b00001000;
const INTERRUPT: u8 = 0b00000100;
const ZERO     : u8 = 0b00000010;
const CARRY    : u8 = 0b00000001;

#[derive(fmt::Debug)]
pub struct Registers {
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
pub struct TimingState {
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
            t0: true,
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
        if self.t5 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "T5")?;
            first = false;
        }
        if self.t6 {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "T6")?;
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
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct State {
    pub registers: Registers,
    pub total_cycles: i32,

    pub ab: u16,
    pub db: u8,
    pub rw: bool,

    // current Instruction
    pub ir: u8,
    // fetched instruction
    pub pd: u8,

    pub clock1: bool,
    pub clock2: bool,

    pub timing: TimingState,

    // sync output
    pub sync: bool,

    // interrupt inputs
    pub res: bool,
    pub irq: bool,
    pub nmi: bool,

}

impl State {
    pub fn new() -> State {
        State {
            registers: Registers::new(),
            total_cycles: 0,

            ir: 0,
            pd: 0,

            ab: 0,
            db: 0,
            rw: false,

            clock1: false,
            clock2: false,

            timing: TimingState::new(),

            sync: false,

            res: false,
            irq: false,
            nmi: false,
        }
    }
}


pub fn step(state: &mut State) {
    if state.res {
        state.timing.clear();
        state.timing.t2 = true;
        state.ir = 0;
        state.registers.sr &= !BREAK;
        state.clock1 = true;
        state.clock2 = false;
        return;
    };
    if state.irq || state.nmi {
        state.registers.sr &= INTERRUPT;
    };
    if state.clock1 {
        state.clock1 = false;
        state.clock2 = true;
        step1(state);
    } else if state.clock2 {
        state.clock1 = true;
        state.clock2 = false;
        step2(state);
        state.total_cycles += 1;
    };
}

fn step1(state: &mut State) {
    if state.timing.t2 {
        if state.res {
            state.timing.clear();
            state.timing.t2 = true;
            state.registers.sr &= !BREAK;
            state.ir = 0;
            return;
        } else {
            state.ir = state.pd;
        }
    };
    let op_code = state.ir; 
    let InstructionInfo {instruction, mode, cycles, extra_cycles} = Instruct::from_op_code(op_code).unwrap();
    match mode {
        AddressType::Impl => {
            match instruction {
                Instruct::BRK => {
                    if state.timing.t6 {
                        state.ab = 0xFFFC;
                        state.rw = true;
                    };
                    if state.timing.t0 {
                        state.ab = 0xFFFD;
                        state.rw = true;
                    };
                }
                _ => unimplemented!()
            }
        },
        AddressType::Immediate => {
            match instruction {
                Instruct::ADC => {
                    if state.timing.t1 {
                        state.ab = state.registers.pc + 2;
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
}

fn step2(state: &mut State) {
    state.pd = state.db;
    if state.timing.t2 {
        state.ir = state.pd;
    };
    if state.ir == 0 {
        if state.timing.t2 {
            if state.registers.sp & BREAK != BREAK {
                state.timing.clear();
                state.timing.t3 = true;
            } else {
                state.timing.clear();
                // state.timing.t0 = true;
                state.timing.t3 = true;
            };
        } else if state.timing.t3 {
            state.ir = 0;
            state.registers.sp = 0;
            state.registers.pc = 0;
            state.timing.clear();
            state.timing.t4 = true;
        } else if state.timing.t4 {
            state.timing.clear();
            state.timing.t5 = true;
        } else if state.timing.t5 {
            state.timing.clear();
            state.timing.t6 = true;
        } else if state.timing.t6 {
            state.registers.pc = state.db as u16;
            state.timing.clear();
            state.timing.t0 = true;
        } else if state.timing.t0 {
            state.registers.pc = (state.db as u16) << 8;
            // state.registers.sp |= BREAK;
            state.timing.clear();
            state.timing.t1 = true;
        } else if state.timing.t1 {
            state.timing.clear();
            state.timing.t2 = true;
        } else {
            panic!("Invalid Timing State");
        };
    } else {
        let op_code = state.ir; 
        let InstructionInfo {instruction, mode, cycles, extra_cycles} = Instruct::from_op_code(op_code).unwrap();
        match mode {
            AddressType::Immediate => {
                match instruction {
                    Instruct::ADC => {
                        if state.timing.t1 {
                            state.timing.clear();
                            state.timing.t2 = true;
                        };
                        if state.timing.t2 {
                            let value = state.db;
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
}

