pub mod lexer;
pub mod parser;

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

    pub fn addr_modes(&self, addr: &AddressType) -> Option<u8> {
        match self {
            Instruct::ADC => match addr {
                AddressType::Impl => Some(0x00),
                AddressTy::Accumulator => Some(0x00),
                AddressTy::Immediate => Some(0x00),
                AddressTy::Relative => Some(0x00),
                AddressTy::ZeroPage => Some(0x00),
                AddressTy::ZeroPageX => Some(0x00),
                AddressTy::ZeroPageY => Some(0x00),
                AddressTy::Absolute => Some(0x00),
                AddressTy::AbsoluteX => Some(0x00),
                AddressTy::AbsoluteY => Some(0x00),
                AddressTy::Indirect => Some(0x00),
                AddressTy::IndirectX => Some(0x00),
                AddressTy::IndirectY => Some(0x00),
                _ => None
            },
            // AND => Some(Instruct::AND),
            // ASL => Some(Instruct::ASL),
            // BCC => Some(Instruct::BCC),
            // BCS => Some(Instruct::BCS),
            // BEQ => Some(Instruct::BEQ),
            // BIT => Some(Instruct::BIT),
            // BMI => Some(Instruct::BMI),
            // BNE => Some(Instruct::BNE),
            // BPL => Some(Instruct::BPL),
            // BRK => Some(Instruct::BRK),
            // BVC => Some(Instruct::BVC),
            // BVS => Some(Instruct::BVS),
            // CLC => Some(Instruct::CLC),
            // CLD => Some(Instruct::CLD),
            // CLI => Some(Instruct::CLI),
            // CLV => Some(Instruct::CLV),
            // CMP => Some(Instruct::CMP),
            // CPX => Some(Instruct::CPX),
            // CPY => Some(Instruct::CPY),
            // DEC => Some(Instruct::DEC),
            // DEX => Some(Instruct::DEX),
            // DEY => Some(Instruct::DEY),
            // EOR => Some(Instruct::EOR),
            // INC => Some(Instruct::INC),
            // INX => Some(Instruct::INX),
            // INY => Some(Instruct::INY),
            // JMP => Some(Instruct::JMP),
            // JSR => Some(Instruct::JSR),
            // LDA => Some(Instruct::LDA),
            // LDX => Some(Instruct::LDX),
            // LDY => Some(Instruct::LDY),
            // LSR => Some(Instruct::LSR),
            // NOP => Some(Instruct::NOP),
            // ORA => Some(Instruct::ORA),
            // PHA => Some(Instruct::PHA),
            // PHP => Some(Instruct::PHP),
            // PLA => Some(Instruct::PLA),
            // PLP => Some(Instruct::PLP),
            // ROL => Some(Instruct::ROL),
            // ROR => Some(Instruct::ROR),
            // RTI => Some(Instruct::RTI),
            // RTS => Some(Instruct::RTS),
            // SBC => Some(Instruct::SBC),
            // SEC => Some(Instruct::SEC),
            // SED => Some(Instruct::SED),
            // SEI => Some(Instruct::SEI),
            // STA => Some(Instruct::STA),
            // STX => Some(Instruct::STX),
            // STY => Some(Instruct::STY),
            // TAX => Some(Instruct::TAX),
            // TAY => Some(Instruct::TAY),
            // TSX => Some(Instruct::TSX),
            // TXA => Some(Instruct::TXA),
            // TXS => Some(Instruct::TXS),
            // TYA => Some(Instruct::TYA),
            _ => unimplemented!()
        }
    }
}

