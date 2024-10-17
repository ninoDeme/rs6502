use crate::asm::lexer::{Token, TokenType, Symbol};
use crate::asm::{Instruct, AddressType, AsmError};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OpCode {
    pub code: u8,
    pub addr: Vec<u8>
}

#[derive(Debug)]
enum InterAddr {
    Addr(AddressType, Option<Value>),
    Label(Symbol)
}

#[derive(Debug)]
struct InterOpCode {
    symbol: Symbol,
    instruct: Instruct,
    addr: InterAddr
}

// pub fn get_op(instruction: &str, addressing: AddressType) -> OpCode { match instruction.to_uppercase().as_str() {
//         "ADC" => {
//
//         },
//         _ => {
//             panic!("Invalid instruction: {instruction}")
//         }
//     }
// }

#[derive(Debug)]
pub enum Radix {
    Hex,
    Dec,
    Oct,
    Bin
}

#[derive(Debug)]
struct Value {
    long: bool,
    symbol: Symbol,
    value: i32
}

enum PState {
    Default,
    PostIntruction(Symbol, Instruct),
    PostNumber(Symbol, Instruct, Value),
}

fn is_keyword(text: &str) -> bool {
    return text == "define" || Instruct::from_str(text).is_some();
}

fn parse_number(token: Token, radix: Radix) -> Result<Value, AsmError> {
    if token.token != TokenType::Number {
        return Err(AsmError::new(&format!("Expected number, found {}", token.symbol.text), Some(token.symbol)))
    };
    let value = match i32::from_str_radix(token.symbol.text.as_str(), match radix {
        Radix::Bin => 2,
        Radix::Oct => 8,
        Radix::Dec => 10,
        Radix::Hex => 16,
    }) {
        Ok(res) => res,
        Err(_) => return Err(AsmError::new(format!("{} is not a valid number", token.symbol.text).as_str(), Some(token.symbol)))
    };
    let is16bit = value > 255 || token.symbol.text.chars().count() > match radix {
        Radix::Bin => 8,
        Radix::Oct => 4,
        Radix::Dec => 3,
        Radix::Hex => 2,
    };
    return Ok(Value {
        long: is16bit,
        symbol: token.symbol,
        value
    })
}

pub fn parse(tokens_vec: Vec<Token>) -> Result<Vec<OpCode>, AsmError> {
    let mut tokens = tokens_vec.into_iter().peekable();
    let mut labels = HashMap::new();
    let mut state = PState::Default;

    let mut instructions: Vec<InterOpCode> = vec![];

    loop {
        match state {
            PState::Default => {
                let curr_token = tokens.next();
                if let Some(token) = curr_token {
                    match token.token {
                        TokenType::Identifier => {
                            if let Some(ins) = Instruct::from_str(token.symbol.text.as_str()) {
                                let ins_symbol = token.symbol;
                                state = PState::PostIntruction(ins_symbol, ins);
                            } else if token.symbol.text.to_lowercase() == "define" {
                                todo!("define statement");
                            } else if tokens.next_if(|t| t.token == TokenType::Colon).is_some() {
                                labels.insert(token.symbol.text.to_lowercase(), instructions.len());
                            } else {
                                return Err(AsmError::new("Unknown instruction or invalid token", Some(token.symbol)))
                            }
                        }
                        _ => {
                            return Err(AsmError::new("Invalid token", Some(token.symbol)))
                        }
                    }
                } else {
                    break;
                }
            }
            PState::PostIntruction(ins_symbol, ins) => {
                let curr_token = tokens.peek();
                match curr_token {
                    Some(Token {token: TokenType::Hash, ..}) => {
                        let mut token = tokens.next().unwrap();
                        token = tokens.next().ok_or(AsmError::new("Expected number, found EOF", Some(token.symbol)))?;
                        let radix = match token.token {
                            TokenType::Bin => {
                                token = tokens.next().unwrap();
                                Radix::Bin
                            },
                            TokenType::Oct => {
                                token = tokens.next().expect("Expected number, found EOF");
                                Radix::Oct
                            },
                            TokenType::Hex => {
                                token = tokens.next().expect("Expected number, found EOF");
                                Radix::Hex
                            },
                            TokenType::Number => {
                                Radix::Dec
                            },
                            _ => unreachable!()
                        };

                        let value = parse_number(token, radix)?;
                        if value.long {
                            return Err(AsmError::new("number can't be bigger than 8 bits", Some(value.symbol)))
                        }
                        instructions.push(InterOpCode {
                            symbol: ins_symbol,
                            instruct: ins,
                            addr: InterAddr::Addr(AddressType::Immediate, Some(value))
                        });
                        state = PState::Default;
                    },
                    Some(Token {token: TokenType::LParen, ..}) => {
                        let mut token = tokens.next().unwrap();
                        token = tokens.next().ok_or(AsmError::new("Expected number, found EOF", Some(token.symbol)))?;
                        let radix = match token.token {
                            TokenType::Bin => {
                                token = tokens.next().unwrap();
                                Radix::Bin
                            },
                            TokenType::Oct => {
                                token = tokens.next().expect("Expected number, found EOF");
                                Radix::Oct
                            },
                            TokenType::Hex => {
                                token = tokens.next().expect("Expected number, found EOF");
                                Radix::Hex
                            },
                            TokenType::Number => {
                                Radix::Dec
                            },
                            _ => unreachable!()
                        };

                        let value = parse_number(token, radix)?;
                        token = tokens.next().ok_or(AsmError::new("Early EOF", None))?;
                        match token.token {
                            TokenType::RParen => {
                                if let Some(Token {token: TokenType::CommaY, ..}) = tokens.peek() {
                                    tokens.next();
                                    instructions.push(InterOpCode {
                                        symbol: ins_symbol,
                                        instruct: ins,
                                        addr: InterAddr::Addr(AddressType::IndirectY, Some(value))
                                    });
                                } else {
                                    instructions.push(InterOpCode {
                                        symbol: ins_symbol,
                                        instruct: ins,
                                        addr: InterAddr::Addr(AddressType::Indirect, Some(value))
                                    });
                                }
                            }
                            TokenType::CommaX => {
                                token = tokens.next().ok_or(AsmError::new("Early EOF", None))?;
                                if let TokenType::RParen = token.token {
                                    instructions.push(InterOpCode {
                                        symbol: ins_symbol,
                                        instruct: ins,
                                        addr: InterAddr::Addr(AddressType::IndirectX, Some(value))
                                    });
                                } else {
                                    return Err(AsmError::new("Unexpected Token", Some(token.symbol)))
                                }
                            },
                            _ => return Err(AsmError::new("Unexpected Token", Some(token.symbol)))
                        }
                        state = PState::Default;
                    },
                    Some(Token {token: TokenType::Bin | TokenType::Hex | TokenType::Oct, ..}) => {
                        let token = tokens.next().unwrap();
                        let radix = match token.token {
                            TokenType::Bin => Radix::Bin,
                            TokenType::Oct => Radix::Oct,
                            TokenType::Hex => Radix::Hex,
                            _ => unreachable!()
                        };

                        let token = tokens.next().ok_or(AsmError::new("Expected number, found EOF", Some(token.symbol)))?;
                        state = PState::PostNumber(
                            ins_symbol,
                            ins,
                            parse_number(token, radix)?
                        );
                    },
                    Some(Token {token: TokenType::Number, ..}) => {
                        let token = tokens.next().unwrap();
                        state = PState::PostNumber(
                            ins_symbol,
                            ins,
                            parse_number(token, Radix::Dec)?
                        );
                    },
                    Some(token @ Token {token: TokenType::Identifier, ..}) => {
                        if is_keyword(token.symbol.text.as_str()) {
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                addr: InterAddr::Addr(AddressType::Impl, None)
                            });
                        } else {
                            let token = tokens.next().unwrap();
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                addr: InterAddr::Label(token.symbol)
                            });
                        }
                        state = PState::Default;
                    }
                    None => {
                        instructions.push(InterOpCode {
                            symbol: ins_symbol,
                            instruct: ins,
                            addr: InterAddr::Addr(AddressType::Impl, None)
                        });
                        state = PState::Default;
                    }
                    _ => {
                        unimplemented!();
                    }
                }
            }
            PState::PostNumber(ins_symbol, ins, value) => {
                let curr_token = tokens.peek();
                match curr_token {
                    Some(Token {token: TokenType::CommaX, ..}) => {
                        instructions.push(InterOpCode {
                            symbol: ins_symbol,
                            instruct: ins,
                            addr: InterAddr::Addr(if value.long {AddressType::AbsoluteX} else {AddressType::ZeroPageX}, Some(value))
                        });
                        tokens.next();
                    }
                    Some(Token {token: TokenType::CommaY, ..}) => {
                        instructions.push(InterOpCode {
                            symbol: ins_symbol,
                            instruct: ins,
                            addr: InterAddr::Addr(if value.long {AddressType::AbsoluteY} else {AddressType::ZeroPageY}, Some(value))
                        });
                        tokens.next();
                    },
                    _ => {
                        instructions.push(InterOpCode {
                            symbol: ins_symbol,
                            instruct: ins,
                            addr: InterAddr::Addr(if value.long {AddressType::Absolute} else {AddressType::ZeroPage}, Some(value))
                        });
                    }
                } 
                state = PState::Default;
            }
        }
    }

    // print_instructions(&instructions);

    let mut result: Vec<OpCode> = vec![];
    for item in instructions.into_iter().enumerate() {
        let (location, op) = item;
        match op.addr {
            InterAddr::Label(label) => {
                let op_addr = labels.get(&label.text).expect(&format!("Undefined label: {label:?}")).clone();
                if let Some(op_code) = op.instruct.get_op_code(&AddressType::Absolute) {
                    if op_addr > u16::MAX as usize {
                        panic!("Absolute address doesnt fit in u16: {op_addr} ({label:?})");
                    }
                    let full_addr = op_addr as u16;
                    let low: u8 = ((full_addr & 0xFF00) >> 8) as u8;
                    let high: u8 = (full_addr & 0x00FF) as u8;
                    result.push(OpCode {
                        code: op_code,
                        addr: vec![high, low]
                    })
                } else if let Some(op_code) = op.instruct.get_op_code(&AddressType::Relative) {
                    let diff = location - op_addr;
                    result.push(OpCode {
                        code: op_code,
                        addr: vec![i8::try_from(diff).expect(&format!("Relative address doesnt fit in i8: {diff} ({label:?})")) as u8]
                    })
                } else {
                    return Err(AsmError::new("Instruction doesn't allow this type of addressing", Some(op.symbol)));
                }
            },
            InterAddr::Addr(addr, value) => {
                match addr {
                    AddressType::Immediate |
                    AddressType::IndirectX | AddressType::IndirectY |
                    AddressType::ZeroPage | AddressType::ZeroPageX | AddressType::ZeroPageY => {
                        if let Some(value) = value {
                            let op_code = match op.instruct.get_op_code(&addr) {
                                Some(val) => val,
                                None => return Err(AsmError::new(&format!("Invalid addres type for instruction {}", op.symbol.text), Some(value.symbol)))
                            };
                            result.push(OpCode {
                                code: op_code,
                                addr: vec![value.value as u8]
                            });
                        } else {
                            return Err(AsmError::new("Missing value", Some(op.symbol)))
                        }
                    },
                    AddressType::Impl | AddressType::Accumulator => {
                        if let Some(value) = value {
                            return Err(AsmError::new("Unexpected value", Some(value.symbol)))
                        } else {
                            if let Some(op_code) = op.instruct.get_op_code(&AddressType::Impl) {
                                result.push(OpCode {
                                    code: op_code,
                                    addr: vec![]
                                });
                            } else if let Some(op_code) = op.instruct.get_op_code(&AddressType::Accumulator) {
                                result.push(OpCode {
                                    code: op_code,
                                    addr: vec![]
                                });
                            } else {
                                return Err(AsmError::new(&format!("Instruction \"{:?}\" needs an address", op.instruct), Some(op.symbol)))
                            }
                        }
                    },
                    AddressType::Indirect |
                    AddressType::Absolute | AddressType::AbsoluteX | AddressType::AbsoluteY => {
                        if let Some(value) = value {
                            let op_code = match op.instruct.get_op_code(&addr) {
                                Some(val) => val,
                                None => return Err(AsmError::new(&format!("Invalid addres type for instruction {}", op.symbol.text), Some(value.symbol)))
                            };
                            let op_addr = value.value;
                            if op_addr > u16::MAX as i32 {
                                panic!("Absolute address doesnt fit in u16: {op_addr}");
                            }
                            let full_addr = op_addr as u16;
                            let low: u8 = ((full_addr & 0xFF00) >> 8) as u8;
                            let high: u8 = (full_addr & 0x00FF) as u8;
                            result.push(OpCode {
                                code: op_code,
                                addr: vec![high, low]
                            })
                        } else {
                            return Err(AsmError::new("Missing value", Some(op.symbol)))
                        }
                    },
                    _ => unimplemented!("{:?}", addr),
                };
            },
        };
    };
    return Ok(result)
}

fn print_instructions(instructions: &Vec<InterOpCode>) {
    for op in instructions {
        println!("{:?} => {}", op.instruct, match &op.addr {
            InterAddr::Label(s) => format!("Label({:?})", &s.text),
            s @ InterAddr::Addr(_, None) => format!("{:?}", &s),
            InterAddr::Addr(addr, Some(val)) => format!("Addr({:?}, {:?})", &addr, (&val.symbol.text, &val.value)),
        });
    }
}
