use crate::instruct::{AddressType, Instruct, InstructionInfo};
use std::fmt;

const NEGATIVE: u8 = 0b10000000;
const OVERFLOW: u8 = 0b01000000;
const _IGNORED: u8 = 0b00100000;
const BREAK: u8 = 0b00010000;
const DECIMAL: u8 = 0b00001000;
const INTERRUPT: u8 = 0b00000100;
const ZERO: u8 = 0b00000010;
const CARRY: u8 = 0b00000001;

#[derive(fmt::Debug, Clone)]
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
    pub sp: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            pc: 0x0000,
            ac: 0x00,
            xr: 0x00,
            yr: 0x00,
            //    nv-bdizc
            sr: 0b00000110,
            sp: 0xFD,
        }
    }
    pub fn status_add(&mut self, flags: u8) -> () {
        self.sr |= flags;
    }
    pub fn status_remove(&mut self, flags: u8) -> () {
        self.sr &= !flags;
    }
    pub fn status_has(&self, flags: u8) -> bool {
        (self.sr & flags) == flags
    }
    pub fn fmt_status(&self) -> String {
        format!(
            "{}{}-{}{}{}{}{}",
            if self.status_has(NEGATIVE) { 'N' } else { 'n' },
            if self.status_has(OVERFLOW) { 'V' } else { 'v' },
            if self.status_has(BREAK) { 'b' } else { 'B' },
            if self.status_has(DECIMAL) { 'D' } else { 'd' },
            if self.status_has(INTERRUPT) { 'I' } else { 'i' },
            if self.status_has(ZERO) { 'Z' } else { 'z' },
            if self.status_has(CARRY) { 'C' } else { 'c' },
        )
    }
}

#[derive(Clone)]
pub struct TimingState {
    t0: bool,
    t1: bool,
    tp: bool,
    t2: bool,
    t3: bool,
    t4: bool,
    t5: bool,
    t6: bool,
    v0: bool,
    sd1: bool,
    sd2: bool,
}

impl TimingState {
    pub fn new() -> TimingState {
        TimingState {
            t0: true,
            t1: false,
            tp: false,
            t2: false,
            t3: false,
            t4: false,
            t5: false,
            t6: false,
            v0: false,
            sd1: false,
            sd2: false,
        }
    }

    pub fn clear() -> TimingState {
        TimingState {
            t0: false,
            t1: false,
            tp: false,
            t2: false,
            t3: false,
            t4: false,
            t5: false,
            t6: false,
            v0: false,
            sd1: false,
            sd2: false,
        }
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

#[derive(Debug, Clone)]
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
    pub next_timing: TimingState,

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
            next_timing: TimingState::new(),

            sync: false,

            res: false,
            irq: false,
            nmi: false,
        }
    }
}

pub fn step(state: &mut State) {
    if state.res {
        state.next_timing = TimingState::clear();
        state.next_timing.t2 = true;
        state.ir = 0;
        state.registers.sr &= !BREAK;
        state.clock1 = true;
        state.clock2 = false;
        state.total_cycles = 0;
        return;
    };
    if state.irq || state.nmi {
        state.registers.sr &= INTERRUPT;
    };
    if state.clock1 {
        state.total_cycles += 1;
        state.clock1 = false;
        state.clock2 = true;
        step1(state);
    } else if state.clock2 {
        state.clock1 = true;
        state.clock2 = false;
        step2(state);
    };
}

fn step1(state: &mut State) {
    state.timing = std::mem::replace(&mut state.next_timing, TimingState::clear());

    state.rw = true;
    if state.timing.t2 {
        if state.irq || state.nmi {
            state.next_timing = TimingState::clear();
            state.next_timing.t2 = true;
            state.registers.sr &= !BREAK;
            state.ir = 0;
            return;
        } else {
            state.ir = state.pd;
        }
    };
    let op_code = state.ir;
    let InstructionInfo {
        instruction,
        mode,
        ..
    } = Instruct::from_op_code(op_code).unwrap();
    match mode {
        AddressType::Impl => match instruction {
            Instruct::BRK => {
                if state.timing.t6 {
                    state.ab = 0xFFFC;
                };
                if state.timing.t0 {
                    state.ab = 0xFFFD;
                };
            }
            _ => unimplemented!(),
        },
        AddressType::Immediate => {
            match instruction {
                _ => {
                    if state.timing.t2 {
                        state.ab = state.registers.pc + 1;
                    };
                    if state.timing.t1 {
                        state.registers.pc = state.registers.pc + 2;
                    };
                },
            };
        }
        AddressType::ZeroPage => {
            match instruction {
                Instruct::STA => {
                    if state.timing.t2 {
                        state.ab = state.registers.pc + 1;
                    };
                    if state.timing.t0 {
                        state.ab = state.pd as u16;
                        state.rw = false;
                        state.db = state.registers.ac;
                    }
                    if state.timing.t1 {
                        state.registers.pc = state.registers.pc + 2;
                    };
                }
                _ => {
                    if state.timing.t2 {
                        state.ab = state.registers.pc + 1;
                    };
                    if state.timing.t0 {
                        state.ab = state.pd as u16;
                    };
                    if state.timing.t1 {
                        state.registers.pc = state.registers.pc + 2;
                    };
                },
            };
        }
        _ => unimplemented!(),
    };
    if state.timing.t1 {
        state.ab = state.registers.pc;
    }
}

fn step2(state: &mut State) {
    state.pd = state.db;

    let op_code = state.ir;
    let InstructionInfo {
        instruction,
        mode,
        ..
    } = Instruct::from_op_code(op_code).unwrap();
    match mode {
        AddressType::Impl => match instruction {
            Instruct::BRK => {
                if state.timing.t3 {
                    state.ir = 0;
                    state.registers.sp = 0;
                    state.registers.pc = 0;
                } else if state.timing.t6 {
                    state.registers.pc = state.db as u16;
                    state.next_timing.t0 = true;
                } else if state.timing.t0 {
                    state.registers.pc = (state.db as u16) << 8;
                }
            }
            _ => unimplemented!(),
        },
        AddressType::Immediate => {
            match instruction {
                Instruct::ADC => {
                    if state.timing.t2 {
                        ins_adc(state);
                    };
                }
                Instruct::LDA => {
                    if state.timing.t2 {
                        state.registers.ac = state.pd;
                    };
                }
                _ => unimplemented!(),
            };
        }
        AddressType::ZeroPage => {
            match instruction {
                Instruct::ADC => {
                    if state.timing.t2 {
                        ins_adc(state);
                        state.next_timing.t0 = true;
                    };
                },
                Instruct::LDA => {
                    if state.timing.t2 {
                        state.registers.ac = state.pd;
                        state.next_timing.t0 = true;
                    };
                }
                Instruct::STA => {
                    if state.timing.t2 {
                        state.next_timing.t0 = true;
                    }
                }
                _ => unimplemented!(),
            };
        }
        _ => unimplemented!(),
    };

    if state.timing.t1 {
        let op_code = state.pd;
        let next_instruct = Instruct::from_op_code(op_code).unwrap();
        state.next_timing = TimingState::clear();
        state.next_timing.t2 = true;
        if next_instruct.cycles == 2 {
            state.next_timing.t0 = true;
        }
    } else {
        if state.timing.t0 {
            state.next_timing.t1 = true;
        } else if state.timing.t2 {
            state.next_timing.t3 = true;
        } else if state.timing.t3 {
            state.next_timing.t4 = true;
        } else if state.timing.t4 {
            state.next_timing.t5 = true;
        } else if state.timing.t5 {
            state.next_timing.t6 = true;
        } else if state.timing.t6 {
            // if the timing is not set manually by the instruction by this point the processor
            // will enter an infinite loop, maybe add a warning or loop detection here
        }
    };
}

fn ins_adc(state: &mut State) -> () {
    let value = state.pd;
    if state.registers.status_has(CARRY) {
        let (value, carry) = state.registers.ac.overflowing_add(1);
        state.registers.ac = value;
        if carry {
            state.registers.status_add(CARRY)
        } else {
            state.registers.status_remove(CARRY)
        };
    }
    let (result, carry) = state.registers.ac.overflowing_add(value);
    state.registers.ac = result;
    if carry {
        state.registers.status_add(CARRY)
    };
}
