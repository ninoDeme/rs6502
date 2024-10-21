pub mod lexer;
pub mod parser;


#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Instruct {
    ADC, // add with carry
    AND, // and (with accumulator)
    ASL, // arithmetic shift left
    BCC, // branch on carry clear
    BCS, // branch on carry set
    BEQ, // branch on equal (zero set)
    BIT, // bit test
    BMI, // branch on minus (negative set)
    BNE, // branch on not equal (zero clear)
    BPL, // branch on plus (negative clear)
    BRK, // break / interrupt
    BVC, // branch on overflow clear
    BVS, // branch on overflow set
    CLC, // clear carry
    CLD, // clear decimal
    CLI, // clear interrupt disable
    CLV, // clear overflow
    CMP, // compare (with accumulator)
    CPX, // compare with X
    CPY, // compare with Y
    DEC, // decrement
    DEX, // decrement X
    DEY, // decrement Y
    EOR, // exclusive or (with accumulator)
    INC, // increment
    INX, // increment X
    INY, // increment Y
    JMP, // jump
    JSR, // jump subroutine
    LDA, // load accumulator
    LDX, // load X
    LDY, // load Y
    LSR, // logical shift right
    NOP, // no operation
    ORA, // or with accumulator
    PHA, // push accumulator
    PHP, // push processor status (SR)
    PLA, // pull accumulator
    PLP, // pull processor status (SR)
    ROL, // rotate left
    ROR, // rotate right
    RTI, // return from interrupt
    RTS, // return from subroutine
    SBC, // subtract with carry
    SEC, // set carry
    SED, // set decimal
    SEI, // set interrupt disable
    STA, // store accumulator
    STX, // store X
    STY, // store Y
    TAX, // transfer accumulator to X
    TAY, // transfer accumulator to Y
    TSX, // transfer stack pointer to X
    TXA, // transfer X to accumulator
    TXS, // transfer X to stack pointer
    TYA, // transfer Y to accumulator 
}

#[derive(Debug)]
pub enum AddressType {
    // OPC 
    Impl,
    // OPC A
    Accumulator,
    // OPC #<8>
    Immediate,
    // OPC <8>
    Relative,
    // OPC <8>
    ZeroPage,
    // OPC <8>,X
    ZeroPageX,
    // OPC <8>,Y
    ZeroPageY,
    // <op> <16>
    Absolute,
    // <op> <16>,X
    AbsoluteX,
    // <op> <16>,Y
    AbsoluteY,
    // <op> (<16>)
    Indirect,
    // <op> (<8>,X)
    IndirectX,
    // <op> (<8>),Y
    IndirectY,
}

impl Instruct {
    pub fn from_str(text: &str) -> Option<Instruct> {
        match text.to_uppercase().as_str() {
            "ADC" => Some(Instruct::ADC),
            "AND" => Some(Instruct::AND),
            "ASL" => Some(Instruct::ASL),
            "BCC" => Some(Instruct::BCC),
            "BCS" => Some(Instruct::BCS),
            "BEQ" => Some(Instruct::BEQ),
            "BIT" => Some(Instruct::BIT),
            "BMI" => Some(Instruct::BMI),
            "BNE" => Some(Instruct::BNE),
            "BPL" => Some(Instruct::BPL),
            "BRK" => Some(Instruct::BRK),
            "BVC" => Some(Instruct::BVC),
            "BVS" => Some(Instruct::BVS),
            "CLC" => Some(Instruct::CLC),
            "CLD" => Some(Instruct::CLD),
            "CLI" => Some(Instruct::CLI),
            "CLV" => Some(Instruct::CLV),
            "CMP" => Some(Instruct::CMP),
            "CPX" => Some(Instruct::CPX),
            "CPY" => Some(Instruct::CPY),
            "DEC" => Some(Instruct::DEC),
            "DEX" => Some(Instruct::DEX),
            "DEY" => Some(Instruct::DEY),
            "EOR" => Some(Instruct::EOR),
            "INC" => Some(Instruct::INC),
            "INX" => Some(Instruct::INX),
            "INY" => Some(Instruct::INY),
            "JMP" => Some(Instruct::JMP),
            "JSR" => Some(Instruct::JSR),
            "LDA" => Some(Instruct::LDA),
            "LDX" => Some(Instruct::LDX),
            "LDY" => Some(Instruct::LDY),
            "LSR" => Some(Instruct::LSR),
            "NOP" => Some(Instruct::NOP),
            "ORA" => Some(Instruct::ORA),
            "PHA" => Some(Instruct::PHA),
            "PHP" => Some(Instruct::PHP),
            "PLA" => Some(Instruct::PLA),
            "PLP" => Some(Instruct::PLP),
            "ROL" => Some(Instruct::ROL),
            "ROR" => Some(Instruct::ROR),
            "RTI" => Some(Instruct::RTI),
            "RTS" => Some(Instruct::RTS),
            "SBC" => Some(Instruct::SBC),
            "SEC" => Some(Instruct::SEC),
            "SED" => Some(Instruct::SED),
            "SEI" => Some(Instruct::SEI),
            "STA" => Some(Instruct::STA),
            "STX" => Some(Instruct::STX),
            "STY" => Some(Instruct::STY),
            "TAX" => Some(Instruct::TAX),
            "TAY" => Some(Instruct::TAY),
            "TSX" => Some(Instruct::TSX),
            "TXA" => Some(Instruct::TXA),
            "TXS" => Some(Instruct::TXS),
            "TYA" => Some(Instruct::TYA),
            _ => None
        }
    }

    pub fn get_op_code(&self, addr: &AddressType) -> Option<u8> {
        match self {
            Instruct::ADC => match addr {
                AddressType::Immediate => Some(0x69),
                AddressType::ZeroPage => Some(0x65),
                AddressType::ZeroPageX => Some(0x75),
                AddressType::Absolute => Some(0x6D),
                AddressType::AbsoluteX => Some(0x7D),
                AddressType::AbsoluteY => Some(0x79),
                AddressType::IndirectX => Some(0x61),
                AddressType::IndirectY => Some(0x71),
                _ => None,
            },
            Instruct::AND => match addr {
                AddressType::Immediate => Some(0x29),
                AddressType::ZeroPage => Some(0x25),
                AddressType::ZeroPageX => Some(0x35),
                AddressType::Absolute => Some(0x2D),
                AddressType::AbsoluteX => Some(0x3D),
                AddressType::AbsoluteY => Some(0x39),
                AddressType::IndirectX => Some(0x21),
                AddressType::IndirectY => Some(0x31),
                _ => None,
            },
            Instruct::ASL => match addr {
                AddressType::Accumulator => Some(0xA),
                AddressType::ZeroPage => Some(0x6),
                AddressType::ZeroPageX => Some(0x16),
                AddressType::Absolute => Some(0xE),
                AddressType::AbsoluteX => Some(0x1E),
                _ => None,
            },
            Instruct::BCC => match addr {
                AddressType::Relative => Some(0x90),
                _ => None,
            },
            Instruct::BCS => match addr {
                AddressType::Relative => Some(0xB0),
                _ => None,
            },
            Instruct::BEQ => match addr {
                AddressType::Relative => Some(0xF0),
                _ => None,
            },
            Instruct::BIT => match addr {
                AddressType::ZeroPage => Some(0x24),
                AddressType::Absolute => Some(0x2C),
                _ => None,
            },
            Instruct::BMI => match addr {
                AddressType::Relative => Some(0x30),
                _ => None,
            },
            Instruct::BNE => match addr {
                AddressType::Relative => Some(0xD0),
                _ => None,
            },
            Instruct::BPL => match addr {
                AddressType::Relative => Some(0x10),
                _ => None,
            },
            Instruct::BRK => match addr {
                AddressType::Impl => Some(0x0),
                _ => None,
            },
            Instruct::BVC => match addr {
                AddressType::Relative => Some(0x50),
                _ => None,
            },
            Instruct::BVS => match addr {
                AddressType::Relative => Some(0x70),
                _ => None,
            },
            Instruct::CLC => match addr {
                AddressType::Impl => Some(0x18),
                _ => None,
            },
            Instruct::CLD => match addr {
                AddressType::Impl => Some(0xD8),
                _ => None,
            },
            Instruct::CLI => match addr {
                AddressType::Impl => Some(0x58),
                _ => None,
            },
            Instruct::CLV => match addr {
                AddressType::Impl => Some(0xB8),
                _ => None,
            },
            Instruct::CMP => match addr {
                AddressType::Immediate => Some(0xC9),
                AddressType::ZeroPage => Some(0xC5),
                AddressType::ZeroPageX => Some(0xD5),
                AddressType::Absolute => Some(0xCD),
                AddressType::AbsoluteX => Some(0xDD),
                AddressType::AbsoluteY => Some(0xD9),
                AddressType::IndirectX => Some(0xC1),
                AddressType::IndirectY => Some(0xD1),
                _ => None,
            },
            Instruct::CPX => match addr {
                AddressType::Immediate => Some(0xE0),
                AddressType::ZeroPage => Some(0xE4),
                AddressType::Absolute => Some(0xEC),
                _ => None,
            },
            Instruct::CPY => match addr {
                AddressType::Immediate => Some(0xC0),
                AddressType::ZeroPage => Some(0xC4),
                AddressType::Absolute => Some(0xCC),
                _ => None,
            },
            Instruct::DEC => match addr {
                AddressType::ZeroPage => Some(0xC6),
                AddressType::ZeroPageX => Some(0xD6),
                AddressType::Absolute => Some(0xCE),
                AddressType::AbsoluteX => Some(0xDE),
                _ => None,
            },
            Instruct::DEX => match addr {
                AddressType::Impl => Some(0xCA),
                _ => None,
            },
            Instruct::DEY => match addr {
                AddressType::Impl => Some(0x88),
                _ => None,
            },
            Instruct::EOR => match addr {
                AddressType::Immediate => Some(0x49),
                AddressType::ZeroPage => Some(0x45),
                AddressType::ZeroPageX => Some(0x55),
                AddressType::Absolute => Some(0x4D),
                AddressType::AbsoluteX => Some(0x5D),
                AddressType::AbsoluteY => Some(0x59),
                AddressType::IndirectX => Some(0x41),
                AddressType::IndirectY => Some(0x51),
                _ => None,
            },
            Instruct::INC => match addr {
                AddressType::ZeroPage => Some(0xE6),
                AddressType::ZeroPageX => Some(0xF6),
                AddressType::Absolute => Some(0xEE),
                AddressType::AbsoluteX => Some(0xFE),
                _ => None,
            },
            Instruct::INX => match addr {
                AddressType::Impl => Some(0xE8),
                _ => None,
            },
            Instruct::INY => match addr {
                AddressType::Impl => Some(0xC8),
                _ => None,
            },
            Instruct::JMP => match addr {
                AddressType::Absolute => Some(0x4C),
                AddressType::Indirect => Some(0x6C),
                _ => None,
            },
            Instruct::JSR => match addr {
                AddressType::Absolute => Some(0x20),
                _ => None,
            },
            Instruct::LDA => match addr {
                AddressType::Immediate => Some(0xA9),
                AddressType::ZeroPage => Some(0xA5),
                AddressType::ZeroPageX => Some(0xB5),
                AddressType::Absolute => Some(0xAD),
                AddressType::AbsoluteX => Some(0xBD),
                AddressType::AbsoluteY => Some(0xB9),
                AddressType::IndirectX => Some(0xA1),
                AddressType::IndirectY => Some(0xB1),
                _ => None,
            },
            Instruct::LDX => match addr {
                AddressType::Immediate => Some(0xA2),
                AddressType::ZeroPage => Some(0xA6),
                AddressType::ZeroPageY => Some(0xB6),
                AddressType::Absolute => Some(0xAE),
                AddressType::AbsoluteY => Some(0xBE),
                _ => None,
            },
            Instruct::LDY => match addr {
                AddressType::Immediate => Some(0xA0),
                AddressType::ZeroPage => Some(0xA4),
                AddressType::ZeroPageX => Some(0xB4),
                AddressType::Absolute => Some(0xAC),
                AddressType::AbsoluteX => Some(0xBC),
                _ => None,
            },
            Instruct::LSR => match addr {
                AddressType::Accumulator => Some(0x4A),
                AddressType::ZeroPage => Some(0x46),
                AddressType::ZeroPageX => Some(0x56),
                AddressType::Absolute => Some(0x4E),
                AddressType::AbsoluteX => Some(0x5E),
                _ => None,
            },
            Instruct::NOP => match addr {
                AddressType::Impl => Some(0xEA),
                _ => None,
            },
            Instruct::ORA => match addr {
                AddressType::Immediate => Some(0x9),
                AddressType::ZeroPage => Some(0x5),
                AddressType::ZeroPageX => Some(0x15),
                AddressType::Absolute => Some(0xD),
                AddressType::AbsoluteX => Some(0x1D),
                AddressType::AbsoluteY => Some(0x19),
                AddressType::IndirectX => Some(0x1),
                AddressType::IndirectY => Some(0x11),
                _ => None,
            },
            Instruct::PHA => match addr {
                AddressType::Impl => Some(0x48),
                _ => None,
            },
            Instruct::PHP => match addr {
                AddressType::Impl => Some(0x8),
                _ => None,
            },
            Instruct::PLA => match addr {
                AddressType::Impl => Some(0x68),
                _ => None,
            },
            Instruct::PLP => match addr {
                AddressType::Impl => Some(0x28),
                _ => None,
            },
            Instruct::ROL => match addr {
                AddressType::Accumulator => Some(0x2A),
                AddressType::ZeroPage => Some(0x26),
                AddressType::ZeroPageX => Some(0x36),
                AddressType::Absolute => Some(0x2E),
                AddressType::AbsoluteX => Some(0x3E),
                _ => None,
            },
            Instruct::ROR => match addr {
                AddressType::Accumulator => Some(0x6A),
                AddressType::ZeroPage => Some(0x66),
                AddressType::ZeroPageX => Some(0x76),
                AddressType::Absolute => Some(0x6E),
                AddressType::AbsoluteX => Some(0x7E),
                _ => None,
            },
            Instruct::RTI => match addr {
                AddressType::Impl => Some(0x40),
                _ => None,
            },
            Instruct::RTS => match addr {
                AddressType::Impl => Some(0x60),
                _ => None,
            },
            Instruct::SBC => match addr {
                AddressType::Immediate => Some(0xE9),
                AddressType::ZeroPage => Some(0xE5),
                AddressType::ZeroPageX => Some(0xF5),
                AddressType::Absolute => Some(0xED),
                AddressType::AbsoluteX => Some(0xFD),
                AddressType::AbsoluteY => Some(0xF9),
                AddressType::IndirectX => Some(0xE1),
                AddressType::IndirectY => Some(0xF1),
                _ => None,
            },
            Instruct::SEC => match addr {
                AddressType::Impl => Some(0x38),
                _ => None,
            },
            Instruct::SED => match addr {
                AddressType::Impl => Some(0xF8),
                _ => None,
            },
            Instruct::SEI => match addr {
                AddressType::Impl => Some(0x78),
                _ => None,
            },
            Instruct::STA => match addr {
                AddressType::ZeroPage => Some(0x85),
                AddressType::ZeroPageX => Some(0x95),
                AddressType::Absolute => Some(0x8D),
                AddressType::AbsoluteX => Some(0x9D),
                AddressType::AbsoluteY => Some(0x99),
                AddressType::IndirectX => Some(0x81),
                AddressType::IndirectY => Some(0x91),
                _ => None,
            },
            Instruct::STX => match addr {
                AddressType::ZeroPage => Some(0x86),
                AddressType::ZeroPageY => Some(0x96),
                AddressType::Absolute => Some(0x8E),
                _ => None,
            },
            Instruct::STY => match addr {
                AddressType::ZeroPage => Some(0x84),
                AddressType::ZeroPageX => Some(0x94),
                AddressType::Absolute => Some(0x8C),
                _ => None,
            },
            Instruct::TAX => match addr {
                AddressType::Impl => Some(0xAA),
                _ => None,
            },
            Instruct::TAY => match addr {
                AddressType::Impl => Some(0xA8),
                _ => None,
            },
            Instruct::TSX => match addr {
                AddressType::Impl => Some(0xBA),
                _ => None,
            },
            Instruct::TXA => match addr {
                AddressType::Impl => Some(0x8A),
                _ => None,
            },
            Instruct::TXS => match addr {
                AddressType::Impl => Some(0x9A),
                _ => None,
            },
            Instruct::TYA => match addr {
                AddressType::Impl => Some(0x98),
                _ => None,
            },
        }
    }

    pub fn from_op_code(op_code: u8) -> Option<(Instruct, AddressType)> {
        match op_code {
            0x00 => Some((Instruct::BRK, AddressType::Impl)),
            0x01 => Some((Instruct::ORA, AddressType::IndirectX)),
            0x05 => Some((Instruct::ORA, AddressType::ZeroPage)),
            0x06 => Some((Instruct::ASL, AddressType::ZeroPage)),
            0x08 => Some((Instruct::PHP, AddressType::Impl)),
            0x09 => Some((Instruct::ORA, AddressType::Immediate)),
            0x0A => Some((Instruct::ASL, AddressType::Accumulator)),
            0x0D => Some((Instruct::ORA, AddressType::Absolute)),
            0x0E => Some((Instruct::ASL, AddressType::Absolute)),
            0x10 => Some((Instruct::BPL, AddressType::Relative)),
            0x11 => Some((Instruct::ORA, AddressType::IndirectY)),
            0x15 => Some((Instruct::ORA, AddressType::ZeroPageX)),
            0x16 => Some((Instruct::ASL, AddressType::ZeroPageX)),
            0x18 => Some((Instruct::CLC, AddressType::Impl)),
            0x19 => Some((Instruct::ORA, AddressType::AbsoluteY)),
            0x1D => Some((Instruct::ORA, AddressType::AbsoluteX)),
            0x1E => Some((Instruct::ASL, AddressType::AbsoluteX)),
            0x20 => Some((Instruct::JSR, AddressType::Absolute)),
            0x21 => Some((Instruct::AND, AddressType::IndirectX)),
            0x24 => Some((Instruct::BIT, AddressType::ZeroPage)),
            0x25 => Some((Instruct::AND, AddressType::ZeroPage)),
            0x26 => Some((Instruct::ROL, AddressType::ZeroPage)),
            0x28 => Some((Instruct::PLP, AddressType::Impl)),
            0x29 => Some((Instruct::AND, AddressType::Immediate)),
            0x2A => Some((Instruct::ROL, AddressType::Accumulator)),
            0x2C => Some((Instruct::BIT, AddressType::Absolute)),
            0x2D => Some((Instruct::AND, AddressType::Absolute)),
            0x2E => Some((Instruct::ROL, AddressType::Absolute)),
            0x30 => Some((Instruct::BMI, AddressType::Relative)),
            0x31 => Some((Instruct::AND, AddressType::IndirectY)),
            0x35 => Some((Instruct::AND, AddressType::ZeroPageX)),
            0x36 => Some((Instruct::ROL, AddressType::ZeroPageX)),
            0x38 => Some((Instruct::SEC, AddressType::Impl)),
            0x39 => Some((Instruct::AND, AddressType::AbsoluteY)),
            0x3D => Some((Instruct::AND, AddressType::AbsoluteX)),
            0x3E => Some((Instruct::ROL, AddressType::AbsoluteX)),
            0x40 => Some((Instruct::RTI, AddressType::Impl)),
            0x41 => Some((Instruct::EOR, AddressType::IndirectX)),
            0x45 => Some((Instruct::EOR, AddressType::ZeroPage)),
            0x46 => Some((Instruct::LSR, AddressType::ZeroPage)),
            0x48 => Some((Instruct::PHA, AddressType::Impl)),
            0x49 => Some((Instruct::EOR, AddressType::Immediate)),
            0x4A => Some((Instruct::LSR, AddressType::Accumulator)),
            0x4C => Some((Instruct::JMP, AddressType::Absolute)),
            0x4D => Some((Instruct::EOR, AddressType::Absolute)),
            0x4E => Some((Instruct::LSR, AddressType::Absolute)),
            0x50 => Some((Instruct::BVC, AddressType::Relative)),
            0x51 => Some((Instruct::EOR, AddressType::IndirectY)),
            0x55 => Some((Instruct::EOR, AddressType::ZeroPageX)),
            0x56 => Some((Instruct::LSR, AddressType::ZeroPageX)),
            0x58 => Some((Instruct::CLI, AddressType::Impl)),
            0x59 => Some((Instruct::EOR, AddressType::AbsoluteY)),
            0x5D => Some((Instruct::EOR, AddressType::AbsoluteX)),
            0x5E => Some((Instruct::LSR, AddressType::AbsoluteX)),
            0x60 => Some((Instruct::RTS, AddressType::Impl)),
            0x61 => Some((Instruct::ADC, AddressType::IndirectX)),
            0x65 => Some((Instruct::ADC, AddressType::ZeroPage)),
            0x66 => Some((Instruct::ROR, AddressType::ZeroPage)),
            0x68 => Some((Instruct::PLA, AddressType::Impl)),
            0x69 => Some((Instruct::ADC, AddressType::Immediate)),
            0x6A => Some((Instruct::ROR, AddressType::Accumulator)),
            0x6C => Some((Instruct::JMP, AddressType::Indirect)),
            0x6D => Some((Instruct::ADC, AddressType::Absolute)),
            0x6E => Some((Instruct::ROR, AddressType::Absolute)),
            0x70 => Some((Instruct::BVS, AddressType::Relative)),
            0x71 => Some((Instruct::ADC, AddressType::IndirectY)),
            0x75 => Some((Instruct::ADC, AddressType::ZeroPageX)),
            0x76 => Some((Instruct::ROR, AddressType::ZeroPageX)),
            0x78 => Some((Instruct::SEI, AddressType::Impl)),
            0x79 => Some((Instruct::ADC, AddressType::AbsoluteY)),
            0x7D => Some((Instruct::ADC, AddressType::AbsoluteX)),
            0x7E => Some((Instruct::ROR, AddressType::AbsoluteX)),
            0x81 => Some((Instruct::STA, AddressType::IndirectX)),
            0x84 => Some((Instruct::STY, AddressType::ZeroPage)),
            0x85 => Some((Instruct::STA, AddressType::ZeroPage)),
            0x86 => Some((Instruct::STX, AddressType::ZeroPage)),
            0x88 => Some((Instruct::DEY, AddressType::Impl)),
            0x8A => Some((Instruct::TXA, AddressType::Impl)),
            0x8C => Some((Instruct::STY, AddressType::Absolute)),
            0x8D => Some((Instruct::STA, AddressType::Absolute)),
            0x8E => Some((Instruct::STX, AddressType::Absolute)),
            0x90 => Some((Instruct::BCC, AddressType::Relative)),
            0x91 => Some((Instruct::STA, AddressType::IndirectY)),
            0x94 => Some((Instruct::STY, AddressType::ZeroPageX)),
            0x95 => Some((Instruct::STA, AddressType::ZeroPageX)),
            0x96 => Some((Instruct::STX, AddressType::ZeroPageY)),
            0x98 => Some((Instruct::TYA, AddressType::Impl)),
            0x99 => Some((Instruct::STA, AddressType::AbsoluteY)),
            0x9A => Some((Instruct::TXS, AddressType::Impl)),
            0x9D => Some((Instruct::STA, AddressType::AbsoluteX)),
            0xA0 => Some((Instruct::LDY, AddressType::Immediate)),
            0xA1 => Some((Instruct::LDA, AddressType::IndirectX)),
            0xA2 => Some((Instruct::LDX, AddressType::Immediate)),
            0xA4 => Some((Instruct::LDY, AddressType::ZeroPage)),
            0xA5 => Some((Instruct::LDA, AddressType::ZeroPage)),
            0xA6 => Some((Instruct::LDX, AddressType::ZeroPage)),
            0xA8 => Some((Instruct::TAY, AddressType::Impl)),
            0xA9 => Some((Instruct::LDA, AddressType::Immediate)),
            0xAA => Some((Instruct::TAX, AddressType::Impl)),
            0xAC => Some((Instruct::LDY, AddressType::Absolute)),
            0xAD => Some((Instruct::LDA, AddressType::Absolute)),
            0xAE => Some((Instruct::LDX, AddressType::Absolute)),
            0xB0 => Some((Instruct::BCS, AddressType::Relative)),
            0xB1 => Some((Instruct::LDA, AddressType::IndirectY)),
            0xB4 => Some((Instruct::LDY, AddressType::ZeroPageX)),
            0xB5 => Some((Instruct::LDA, AddressType::ZeroPageX)),
            0xB6 => Some((Instruct::LDX, AddressType::ZeroPageY)),
            0xB8 => Some((Instruct::CLV, AddressType::Impl)),
            0xB9 => Some((Instruct::LDA, AddressType::AbsoluteY)),
            0xBA => Some((Instruct::TSX, AddressType::Impl)),
            0xBC => Some((Instruct::LDY, AddressType::AbsoluteX)),
            0xBD => Some((Instruct::LDA, AddressType::AbsoluteX)),
            0xBE => Some((Instruct::LDX, AddressType::AbsoluteY)),
            0xC0 => Some((Instruct::CPY, AddressType::Immediate)),
            0xC1 => Some((Instruct::CMP, AddressType::IndirectX)),
            0xC4 => Some((Instruct::CPY, AddressType::ZeroPage)),
            0xC5 => Some((Instruct::CMP, AddressType::ZeroPage)),
            0xC6 => Some((Instruct::DEC, AddressType::ZeroPage)),
            0xC8 => Some((Instruct::INY, AddressType::Impl)),
            0xC9 => Some((Instruct::CMP, AddressType::Immediate)),
            0xCA => Some((Instruct::DEX, AddressType::Impl)),
            0xCC => Some((Instruct::CPY, AddressType::Absolute)),
            0xCD => Some((Instruct::CMP, AddressType::Absolute)),
            0xCE => Some((Instruct::DEC, AddressType::Absolute)),
            0xD0 => Some((Instruct::BNE, AddressType::Relative)),
            0xD1 => Some((Instruct::CMP, AddressType::IndirectY)),
            0xD5 => Some((Instruct::CMP, AddressType::ZeroPageX)),
            0xD6 => Some((Instruct::DEC, AddressType::ZeroPageX)),
            0xD8 => Some((Instruct::CLD, AddressType::Impl)),
            0xD9 => Some((Instruct::CMP, AddressType::AbsoluteY)),
            0xDD => Some((Instruct::CMP, AddressType::AbsoluteX)),
            0xDE => Some((Instruct::DEC, AddressType::AbsoluteX)),
            0xE0 => Some((Instruct::CPX, AddressType::Immediate)),
            0xE1 => Some((Instruct::SBC, AddressType::IndirectX)),
            0xE4 => Some((Instruct::CPX, AddressType::ZeroPage)),
            0xE5 => Some((Instruct::SBC, AddressType::ZeroPage)),
            0xE6 => Some((Instruct::INC, AddressType::ZeroPage)),
            0xE8 => Some((Instruct::INX, AddressType::Impl)),
            0xE9 => Some((Instruct::SBC, AddressType::Immediate)),
            0xEA => Some((Instruct::NOP, AddressType::Impl)),
            0xEC => Some((Instruct::CPX, AddressType::Absolute)),
            0xED => Some((Instruct::SBC, AddressType::Absolute)),
            0xEE => Some((Instruct::INC, AddressType::Absolute)),
            0xF0 => Some((Instruct::BEQ, AddressType::Relative)),
            0xF1 => Some((Instruct::SBC, AddressType::IndirectY)),
            0xF5 => Some((Instruct::SBC, AddressType::ZeroPageX)),
            0xF6 => Some((Instruct::INC, AddressType::ZeroPageX)),
            0xF8 => Some((Instruct::SED, AddressType::Impl)),
            0xF9 => Some((Instruct::SBC, AddressType::AbsoluteY)),
            0xFD => Some((Instruct::SBC, AddressType::AbsoluteX)),
            0xFE => Some((Instruct::INC, AddressType::AbsoluteX)),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub line: usize,
    pub col: usize
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub start: Pos,
    pub end: Pos,
    pub text: String
}

impl Symbol {
    pub fn new(line: usize, col: usize, text: String) -> Symbol {
        Symbol {
            start: Pos {
                line,
                col
            },
            end: Pos {
                line,
                col: col + text.chars().count()
            },
            text
        }
    }
}

#[derive(Debug)]
pub struct AsmError {
    pub symbol: Option<Symbol>,
    pub reason: String
}

impl AsmError {
    pub fn new(reason: &str, symbol: Option<Symbol>) -> AsmError {
        return AsmError {
            symbol,
            reason: String::from(reason)
        };
    }
}
