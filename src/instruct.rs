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

    pub fn from_op_code(op_code: u8) -> Option<InstructionInfo> {
        match op_code {
            0x0 => Some(InstructionInfo {instruction: Instruct::BRK, mode: AddressType::Impl, cycles: 7, extra_cycles: 0}),
            0x1 => Some(InstructionInfo {instruction: Instruct::ORA, mode: AddressType::IndirectX, cycles: 6, extra_cycles: 0}),
            0x5 => Some(InstructionInfo {instruction: Instruct::ORA, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0x6 => Some(InstructionInfo {instruction: Instruct::ASL, mode: AddressType::ZeroPage, cycles: 5, extra_cycles: 0}),
            0x8 => Some(InstructionInfo {instruction: Instruct::PHP, mode: AddressType::Impl, cycles: 3, extra_cycles: 0}),
            0x9 => Some(InstructionInfo {instruction: Instruct::ORA, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0xA => Some(InstructionInfo {instruction: Instruct::ASL, mode: AddressType::Accumulator, cycles: 2, extra_cycles: 0}),
            0xD => Some(InstructionInfo {instruction: Instruct::ORA, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0xE => Some(InstructionInfo {instruction: Instruct::ASL, mode: AddressType::Absolute, cycles: 6, extra_cycles: 0}),
            0x10 => Some(InstructionInfo {instruction: Instruct::BPL, mode: AddressType::Relative, cycles: 2, extra_cycles: 2}),
            0x11 => Some(InstructionInfo {instruction: Instruct::ORA, mode: AddressType::IndirectY, cycles: 5, extra_cycles: 1}),
            0x15 => Some(InstructionInfo {instruction: Instruct::ORA, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0x16 => Some(InstructionInfo {instruction: Instruct::ASL, mode: AddressType::ZeroPageX, cycles: 6, extra_cycles: 0}),
            0x18 => Some(InstructionInfo {instruction: Instruct::CLC, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0x19 => Some(InstructionInfo {instruction: Instruct::ORA, mode: AddressType::AbsoluteY, cycles: 4, extra_cycles: 1}),
            0x1D => Some(InstructionInfo {instruction: Instruct::ORA, mode: AddressType::AbsoluteX, cycles: 4, extra_cycles: 1}),
            0x1E => Some(InstructionInfo {instruction: Instruct::ASL, mode: AddressType::AbsoluteX, cycles: 7, extra_cycles: 0}),
            0x20 => Some(InstructionInfo {instruction: Instruct::JSR, mode: AddressType::Absolute, cycles: 6, extra_cycles: 0}),
            0x21 => Some(InstructionInfo {instruction: Instruct::AND, mode: AddressType::IndirectX, cycles: 6, extra_cycles: 0}),
            0x24 => Some(InstructionInfo {instruction: Instruct::BIT, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0x25 => Some(InstructionInfo {instruction: Instruct::AND, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0x26 => Some(InstructionInfo {instruction: Instruct::ROL, mode: AddressType::ZeroPage, cycles: 5, extra_cycles: 0}),
            0x28 => Some(InstructionInfo {instruction: Instruct::PLP, mode: AddressType::Impl, cycles: 4, extra_cycles: 0}),
            0x29 => Some(InstructionInfo {instruction: Instruct::AND, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0x2A => Some(InstructionInfo {instruction: Instruct::ROL, mode: AddressType::Accumulator, cycles: 2, extra_cycles: 0}),
            0x2C => Some(InstructionInfo {instruction: Instruct::BIT, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0x2D => Some(InstructionInfo {instruction: Instruct::AND, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0x2E => Some(InstructionInfo {instruction: Instruct::ROL, mode: AddressType::Absolute, cycles: 6, extra_cycles: 0}),
            0x30 => Some(InstructionInfo {instruction: Instruct::BMI, mode: AddressType::Relative, cycles: 2, extra_cycles: 2}),
            0x31 => Some(InstructionInfo {instruction: Instruct::AND, mode: AddressType::IndirectY, cycles: 5, extra_cycles: 1}),
            0x35 => Some(InstructionInfo {instruction: Instruct::AND, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0x36 => Some(InstructionInfo {instruction: Instruct::ROL, mode: AddressType::ZeroPageX, cycles: 6, extra_cycles: 0}),
            0x38 => Some(InstructionInfo {instruction: Instruct::SEC, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0x39 => Some(InstructionInfo {instruction: Instruct::AND, mode: AddressType::AbsoluteY, cycles: 4, extra_cycles: 1}),
            0x3D => Some(InstructionInfo {instruction: Instruct::AND, mode: AddressType::AbsoluteX, cycles: 4, extra_cycles: 1}),
            0x3E => Some(InstructionInfo {instruction: Instruct::ROL, mode: AddressType::AbsoluteX, cycles: 7, extra_cycles: 0}),
            0x40 => Some(InstructionInfo {instruction: Instruct::RTI, mode: AddressType::Impl, cycles: 6, extra_cycles: 0}),
            0x41 => Some(InstructionInfo {instruction: Instruct::EOR, mode: AddressType::IndirectX, cycles: 6, extra_cycles: 0}),
            0x45 => Some(InstructionInfo {instruction: Instruct::EOR, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0x46 => Some(InstructionInfo {instruction: Instruct::LSR, mode: AddressType::ZeroPage, cycles: 5, extra_cycles: 0}),
            0x48 => Some(InstructionInfo {instruction: Instruct::PHA, mode: AddressType::Impl, cycles: 3, extra_cycles: 0}),
            0x49 => Some(InstructionInfo {instruction: Instruct::EOR, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0x4A => Some(InstructionInfo {instruction: Instruct::LSR, mode: AddressType::Accumulator, cycles: 2, extra_cycles: 0}),
            0x4C => Some(InstructionInfo {instruction: Instruct::JMP, mode: AddressType::Absolute, cycles: 3, extra_cycles: 0}),
            0x4D => Some(InstructionInfo {instruction: Instruct::EOR, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0x4E => Some(InstructionInfo {instruction: Instruct::LSR, mode: AddressType::Absolute, cycles: 6, extra_cycles: 0}),
            0x50 => Some(InstructionInfo {instruction: Instruct::BVC, mode: AddressType::Relative, cycles: 2, extra_cycles: 2}),
            0x51 => Some(InstructionInfo {instruction: Instruct::EOR, mode: AddressType::IndirectY, cycles: 5, extra_cycles: 1}),
            0x55 => Some(InstructionInfo {instruction: Instruct::EOR, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0x56 => Some(InstructionInfo {instruction: Instruct::LSR, mode: AddressType::ZeroPageX, cycles: 6, extra_cycles: 0}),
            0x58 => Some(InstructionInfo {instruction: Instruct::CLI, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0x59 => Some(InstructionInfo {instruction: Instruct::EOR, mode: AddressType::AbsoluteY, cycles: 4, extra_cycles: 1}),
            0x5D => Some(InstructionInfo {instruction: Instruct::EOR, mode: AddressType::AbsoluteX, cycles: 4, extra_cycles: 1}),
            0x5E => Some(InstructionInfo {instruction: Instruct::LSR, mode: AddressType::AbsoluteX, cycles: 7, extra_cycles: 0}),
            0x60 => Some(InstructionInfo {instruction: Instruct::RTS, mode: AddressType::Impl, cycles: 6, extra_cycles: 0}),
            0x61 => Some(InstructionInfo {instruction: Instruct::ADC, mode: AddressType::IndirectX, cycles: 6, extra_cycles: 0}),
            0x65 => Some(InstructionInfo {instruction: Instruct::ADC, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0x66 => Some(InstructionInfo {instruction: Instruct::ROR, mode: AddressType::ZeroPage, cycles: 5, extra_cycles: 0}),
            0x68 => Some(InstructionInfo {instruction: Instruct::PLA, mode: AddressType::Impl, cycles: 4, extra_cycles: 0}),
            0x69 => Some(InstructionInfo {instruction: Instruct::ADC, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0x6A => Some(InstructionInfo {instruction: Instruct::ROR, mode: AddressType::Accumulator, cycles: 2, extra_cycles: 0}),
            0x6C => Some(InstructionInfo {instruction: Instruct::JMP, mode: AddressType::Indirect, cycles: 5, extra_cycles: 0}),
            0x6D => Some(InstructionInfo {instruction: Instruct::ADC, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0x6E => Some(InstructionInfo {instruction: Instruct::ROR, mode: AddressType::Absolute, cycles: 6, extra_cycles: 0}),
            0x70 => Some(InstructionInfo {instruction: Instruct::BVS, mode: AddressType::Relative, cycles: 2, extra_cycles: 2}),
            0x71 => Some(InstructionInfo {instruction: Instruct::ADC, mode: AddressType::IndirectY, cycles: 5, extra_cycles: 1}),
            0x75 => Some(InstructionInfo {instruction: Instruct::ADC, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0x76 => Some(InstructionInfo {instruction: Instruct::ROR, mode: AddressType::ZeroPageX, cycles: 6, extra_cycles: 0}),
            0x78 => Some(InstructionInfo {instruction: Instruct::SEI, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0x79 => Some(InstructionInfo {instruction: Instruct::ADC, mode: AddressType::AbsoluteY, cycles: 4, extra_cycles: 1}),
            0x7D => Some(InstructionInfo {instruction: Instruct::ADC, mode: AddressType::AbsoluteX, cycles: 4, extra_cycles: 1}),
            0x7E => Some(InstructionInfo {instruction: Instruct::ROR, mode: AddressType::AbsoluteX, cycles: 7, extra_cycles: 0}),
            0x81 => Some(InstructionInfo {instruction: Instruct::STA, mode: AddressType::IndirectX, cycles: 6, extra_cycles: 0}),
            0x84 => Some(InstructionInfo {instruction: Instruct::STY, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0x85 => Some(InstructionInfo {instruction: Instruct::STA, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0x86 => Some(InstructionInfo {instruction: Instruct::STX, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0x88 => Some(InstructionInfo {instruction: Instruct::DEY, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0x8A => Some(InstructionInfo {instruction: Instruct::TXA, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0x8C => Some(InstructionInfo {instruction: Instruct::STY, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0x8D => Some(InstructionInfo {instruction: Instruct::STA, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0x8E => Some(InstructionInfo {instruction: Instruct::STX, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0x90 => Some(InstructionInfo {instruction: Instruct::BCC, mode: AddressType::Relative, cycles: 2, extra_cycles: 2}),
            0x91 => Some(InstructionInfo {instruction: Instruct::STA, mode: AddressType::IndirectY, cycles: 6, extra_cycles: 0}),
            0x94 => Some(InstructionInfo {instruction: Instruct::STY, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0x95 => Some(InstructionInfo {instruction: Instruct::STA, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0x96 => Some(InstructionInfo {instruction: Instruct::STX, mode: AddressType::ZeroPageY, cycles: 4, extra_cycles: 0}),
            0x98 => Some(InstructionInfo {instruction: Instruct::TYA, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0x99 => Some(InstructionInfo {instruction: Instruct::STA, mode: AddressType::AbsoluteY, cycles: 5, extra_cycles: 0}),
            0x9A => Some(InstructionInfo {instruction: Instruct::TXS, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0x9D => Some(InstructionInfo {instruction: Instruct::STA, mode: AddressType::AbsoluteX, cycles: 5, extra_cycles: 0}),
            0xA0 => Some(InstructionInfo {instruction: Instruct::LDY, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0xA1 => Some(InstructionInfo {instruction: Instruct::LDA, mode: AddressType::IndirectX, cycles: 6, extra_cycles: 0}),
            0xA2 => Some(InstructionInfo {instruction: Instruct::LDX, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0xA4 => Some(InstructionInfo {instruction: Instruct::LDY, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0xA5 => Some(InstructionInfo {instruction: Instruct::LDA, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0xA6 => Some(InstructionInfo {instruction: Instruct::LDX, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0xA8 => Some(InstructionInfo {instruction: Instruct::TAY, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xA9 => Some(InstructionInfo {instruction: Instruct::LDA, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0xAA => Some(InstructionInfo {instruction: Instruct::TAX, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xAC => Some(InstructionInfo {instruction: Instruct::LDY, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0xAD => Some(InstructionInfo {instruction: Instruct::LDA, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0xAE => Some(InstructionInfo {instruction: Instruct::LDX, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0xB0 => Some(InstructionInfo {instruction: Instruct::BCS, mode: AddressType::Relative, cycles: 2, extra_cycles: 2}),
            0xB1 => Some(InstructionInfo {instruction: Instruct::LDA, mode: AddressType::IndirectY, cycles: 5, extra_cycles: 1}),
            0xB4 => Some(InstructionInfo {instruction: Instruct::LDY, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0xB5 => Some(InstructionInfo {instruction: Instruct::LDA, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0xB6 => Some(InstructionInfo {instruction: Instruct::LDX, mode: AddressType::ZeroPageY, cycles: 4, extra_cycles: 0}),
            0xB8 => Some(InstructionInfo {instruction: Instruct::CLV, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xB9 => Some(InstructionInfo {instruction: Instruct::LDA, mode: AddressType::AbsoluteY, cycles: 4, extra_cycles: 1}),
            0xBA => Some(InstructionInfo {instruction: Instruct::TSX, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xBC => Some(InstructionInfo {instruction: Instruct::LDY, mode: AddressType::AbsoluteX, cycles: 4, extra_cycles: 1}),
            0xBD => Some(InstructionInfo {instruction: Instruct::LDA, mode: AddressType::AbsoluteX, cycles: 4, extra_cycles: 1}),
            0xBE => Some(InstructionInfo {instruction: Instruct::LDX, mode: AddressType::AbsoluteY, cycles: 4, extra_cycles: 1}),
            0xC0 => Some(InstructionInfo {instruction: Instruct::CPY, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0xC1 => Some(InstructionInfo {instruction: Instruct::CMP, mode: AddressType::IndirectX, cycles: 6, extra_cycles: 0}),
            0xC4 => Some(InstructionInfo {instruction: Instruct::CPY, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0xC5 => Some(InstructionInfo {instruction: Instruct::CMP, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0xC6 => Some(InstructionInfo {instruction: Instruct::DEC, mode: AddressType::ZeroPage, cycles: 5, extra_cycles: 0}),
            0xC8 => Some(InstructionInfo {instruction: Instruct::INY, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xC9 => Some(InstructionInfo {instruction: Instruct::CMP, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0xCA => Some(InstructionInfo {instruction: Instruct::DEX, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xCC => Some(InstructionInfo {instruction: Instruct::CPY, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0xCD => Some(InstructionInfo {instruction: Instruct::CMP, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0xCE => Some(InstructionInfo {instruction: Instruct::DEC, mode: AddressType::Absolute, cycles: 6, extra_cycles: 0}),
            0xD0 => Some(InstructionInfo {instruction: Instruct::BNE, mode: AddressType::Relative, cycles: 2, extra_cycles: 2}),
            0xD1 => Some(InstructionInfo {instruction: Instruct::CMP, mode: AddressType::IndirectY, cycles: 5, extra_cycles: 1}),
            0xD5 => Some(InstructionInfo {instruction: Instruct::CMP, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0xD6 => Some(InstructionInfo {instruction: Instruct::DEC, mode: AddressType::ZeroPageX, cycles: 6, extra_cycles: 0}),
            0xD8 => Some(InstructionInfo {instruction: Instruct::CLD, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xD9 => Some(InstructionInfo {instruction: Instruct::CMP, mode: AddressType::AbsoluteY, cycles: 4, extra_cycles: 1}),
            0xDD => Some(InstructionInfo {instruction: Instruct::CMP, mode: AddressType::AbsoluteX, cycles: 4, extra_cycles: 1}),
            0xDE => Some(InstructionInfo {instruction: Instruct::DEC, mode: AddressType::AbsoluteX, cycles: 7, extra_cycles: 0}),
            0xE0 => Some(InstructionInfo {instruction: Instruct::CPX, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0xE1 => Some(InstructionInfo {instruction: Instruct::SBC, mode: AddressType::IndirectX, cycles: 6, extra_cycles: 0}),
            0xE4 => Some(InstructionInfo {instruction: Instruct::CPX, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0xE5 => Some(InstructionInfo {instruction: Instruct::SBC, mode: AddressType::ZeroPage, cycles: 3, extra_cycles: 0}),
            0xE6 => Some(InstructionInfo {instruction: Instruct::INC, mode: AddressType::ZeroPage, cycles: 5, extra_cycles: 0}),
            0xE8 => Some(InstructionInfo {instruction: Instruct::INX, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xE9 => Some(InstructionInfo {instruction: Instruct::SBC, mode: AddressType::Immediate, cycles: 2, extra_cycles: 0}),
            0xEA => Some(InstructionInfo {instruction: Instruct::NOP, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xEC => Some(InstructionInfo {instruction: Instruct::CPX, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0xED => Some(InstructionInfo {instruction: Instruct::SBC, mode: AddressType::Absolute, cycles: 4, extra_cycles: 0}),
            0xEE => Some(InstructionInfo {instruction: Instruct::INC, mode: AddressType::Absolute, cycles: 6, extra_cycles: 0}),
            0xF0 => Some(InstructionInfo {instruction: Instruct::BEQ, mode: AddressType::Relative, cycles: 2, extra_cycles: 2}),
            0xF1 => Some(InstructionInfo {instruction: Instruct::SBC, mode: AddressType::IndirectY, cycles: 5, extra_cycles: 1}),
            0xF5 => Some(InstructionInfo {instruction: Instruct::SBC, mode: AddressType::ZeroPageX, cycles: 4, extra_cycles: 0}),
            0xF6 => Some(InstructionInfo {instruction: Instruct::INC, mode: AddressType::ZeroPageX, cycles: 6, extra_cycles: 0}),
            0xF8 => Some(InstructionInfo {instruction: Instruct::SED, mode: AddressType::Impl, cycles: 2, extra_cycles: 0}),
            0xF9 => Some(InstructionInfo {instruction: Instruct::SBC, mode: AddressType::AbsoluteY, cycles: 4, extra_cycles: 1}),
            0xFD => Some(InstructionInfo {instruction: Instruct::SBC, mode: AddressType::AbsoluteX, cycles: 4, extra_cycles: 1}),
            0xFE => Some(InstructionInfo {instruction: Instruct::INC, mode: AddressType::AbsoluteX, cycles: 7, extra_cycles: 0}),
            _ => None
        }
    }
}

pub struct InstructionInfo {
    pub instruction: Instruct,
    pub mode: AddressType,
    pub cycles: u8,
    pub extra_cycles: u8
}

