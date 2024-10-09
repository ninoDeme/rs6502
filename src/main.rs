#[derive(Debug)]
enum StatusFlags {
    Negative  = 0b10000000,
    Overflow  = 0b01000000,
    Ignored   = 0b00100000,
    Break     = 0b00010000,
    Decimal   = 0b00001000,
    Interrupt = 0b00000100,
    Zero      = 0b00000010,
    Carry     = 0b00000001,
}

#[derive(Debug)]
struct Registers {
    // Program Counter
    pc: u16,
    // Accumulator
    ac: u8,
    // X Register
    x: u8,
    // Y Register
    y: u8,
    // Status register [NV-BDIZC]
    sr: u8,
    // Stack pointer
    sp: u8
}

#[derive(Debug)]
struct State {
    registers: Registers,
    memory: [u8; 65536],
    program: Vec<u8>
}

fn main() {
    println!("Hello, world!");
}
