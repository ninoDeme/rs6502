use crate::asm::lexer::{Token, TokenType};
use crate::asm::{Instruct, AddressType, AsmError, Symbol};
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
    ins_addr: u16,
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

fn throw_newline(token: Option<Token>) -> Result<Token, AsmError> {
    return match token {
        t @ (None | Some(Token {token: TokenType::NewLine, ..})) => {
            Err(AsmError::new("Early EOF", t.map(|token| token.symbol)))
        },
        Some(val) => Ok(val),
    }
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

pub fn extend_tokens(tokens: Vec<Token>) -> Result<Vec<Token>, AsmError> {
    let mut tokens = tokens.into_iter().peekable();
    let mut defines: HashMap<String, Vec<Token>> = HashMap::new();

    let mut new_tokens: Vec<Token> = vec![];
    let mut parsing_define: Option<(String, Vec<Token>)> = None;
    while let Some(token) = tokens.next() {
        match token.token {
            TokenType::NewLine => {
                if let Some((define_name, define_vec)) = parsing_define {
                    defines.insert(define_name, define_vec);
                    parsing_define = None;
                } 
                if tokens.peek().is_some_and(|n_token| &n_token.symbol.text.to_lowercase() == "define" ) {
                    tokens.next();
                    match tokens.next() {
                        Some(define_name @ Token {token: TokenType::Identifier, ..}) => {
                            parsing_define = Some((define_name.symbol.text, vec![]));
                        },
                        token => {
                            return Err(AsmError::new("Invalid define identifier", token.map(|t| t.symbol)))
                        }
                    }
                } else {
                    new_tokens.push(token);
                }
            },
            TokenType::Identifier => {
                // println!("{token:?}");
                // println!("{defines:?}");
                if let Some((_, ref mut def_vec)) = parsing_define {
                    if let Some(define_tokens) = defines.get(&token.symbol.text) {
                        def_vec.append(&mut define_tokens.clone());
                    } else {
                        def_vec.push(token.clone());
                    }
                } else {
                    if let Some(define_tokens) = defines.get(&token.symbol.text) {
                        new_tokens.append(&mut define_tokens.clone());
                    } else {
                        new_tokens.push(token.clone());
                    }
                }
            }
            _ => {
                if let Some((_, ref mut def_vec)) = parsing_define {
                    def_vec.push(token.clone());
                } else {
                    new_tokens.push(token)
                }
            }
        }
    }
    return Ok(new_tokens);
}

const ENTRY_POINT: u16 = 0x0600;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<OpCode>, AsmError> {
    let mut tokens = extend_tokens(tokens)?.into_iter().peekable();
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut state = PState::Default;

    let mut instructions: Vec<InterOpCode> = vec![];

    let mut ins_addr = ENTRY_POINT;
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
                                return Err(AsmError::new("Invalid token", Some(token.symbol)))
                            } else if tokens.next_if(|t| t.token == TokenType::Colon).is_some() {
                                labels.insert(token.symbol.text, ins_addr);
                            } else {
                                return Err(AsmError::new("Unknown instruction or invalid token", Some(token.symbol)))
                            }
                        },
                        TokenType::NewLine => {
                            state = PState::Default;
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
                        tokens.next().unwrap(); 
                        let mut token = throw_newline(tokens.next())?;
                        let radix = match token.token {
                            TokenType::Bin => {
                                token = throw_newline(tokens.next())?;
                                Radix::Bin
                            },
                            TokenType::Oct => {
                                token = throw_newline(tokens.next())?;
                                Radix::Oct
                            },
                            TokenType::Hex => {
                                token = throw_newline(tokens.next())?;
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
                            ins_addr,
                            addr: InterAddr::Addr(AddressType::Immediate, Some(value))
                        });
                        ins_addr += 2;
                        state = PState::Default;
                    },
                    Some(Token {token: TokenType::LParen, ..}) => {
                        tokens.next().unwrap(); 
                        let mut token = throw_newline(tokens.next())?;
                        let radix = match token.token {
                            TokenType::Bin => {
                                token = throw_newline(tokens.next())?;
                                Radix::Bin
                            },
                            TokenType::Oct => {
                                token = throw_newline(tokens.next())?;
                                Radix::Oct
                            },
                            TokenType::Hex => {
                                token = throw_newline(tokens.next())?;
                                Radix::Hex
                            },
                            TokenType::Number => {
                                Radix::Dec
                            },
                            _ => unreachable!()
                        };

                        let value = parse_number(token, radix)?;
                        token = throw_newline(tokens.next())?;
                        match token.token {
                            TokenType::RParen => {
                                if let Some(Token {token: TokenType::CommaY, ..}) = tokens.peek() {
                                    tokens.next();
                                    instructions.push(InterOpCode {
                                        symbol: ins_symbol,
                                        instruct: ins,
                                        ins_addr,
                                        addr: InterAddr::Addr(AddressType::IndirectY, Some(value))
                                    });
                                    ins_addr += 2;
                                } else {
                                    instructions.push(InterOpCode {
                                        symbol: ins_symbol,
                                        instruct: ins,
                                        ins_addr,
                                        addr: InterAddr::Addr(AddressType::Indirect, Some(value))
                                    });
                                    ins_addr += 3;
                                }
                            }
                            TokenType::CommaX => {
                                token = throw_newline(tokens.next())?;
                                if let TokenType::RParen = token.token {
                                    instructions.push(InterOpCode {
                                        symbol: ins_symbol,
                                        instruct: ins,
                                        ins_addr,
                                        addr: InterAddr::Addr(AddressType::IndirectX, Some(value))
                                    });
                                    ins_addr += 2;
                                } else {
                                    return Err(AsmError::new("Unexpected Token", Some(token.symbol)))
                                }
                            },
                            _ => return Err(AsmError::new("Unexpected Token", Some(token.symbol)))
                        }
                        state = PState::Default;
                    },
                    Some(Token {token: TokenType::Bin | TokenType::Hex | TokenType::Oct, ..}) => {
                        let mut token = throw_newline(tokens.next())?; 
                        let radix = match token.token {
                            TokenType::Bin => Radix::Bin,
                            TokenType::Oct => Radix::Oct,
                            TokenType::Hex => Radix::Hex,
                            _ => unreachable!()
                        };

                        token = throw_newline(tokens.next())?;
                        state = PState::PostNumber(
                            ins_symbol,
                            ins,
                            parse_number(token, radix)?
                        );
                    },
                    Some(Token {token: TokenType::Number, ..}) => {
                        let token = throw_newline(tokens.next())?; 
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
                                ins_addr,
                                addr: InterAddr::Addr(AddressType::Impl, None)
                            });
                            ins_addr += 1;
                        } else {
                            let token = throw_newline(tokens.next())?; 
                            let is_rel = ins.get_op_code(&AddressType::Relative).is_some();
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                ins_addr,
                                addr: InterAddr::Label(token.symbol)
                            });
                            if !is_rel {
                                ins_addr += 1;
                            }
                            ins_addr += 2;
                        }
                        state = PState::Default;
                    }
                    None | Some(Token {token: TokenType::NewLine, ..}) => {
                        tokens.next();
                        instructions.push(InterOpCode {
                            symbol: ins_symbol,
                            instruct: ins,
                            ins_addr,
                            addr: InterAddr::Addr(AddressType::Impl, None)
                        });
                        ins_addr += 1;
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
                        if value.long {
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                ins_addr,
                                addr: InterAddr::Addr(AddressType::AbsoluteX, Some(value))
                            });
                            ins_addr += 3;
                        } else {
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                ins_addr,
                                addr: InterAddr::Addr(AddressType::ZeroPageX, Some(value))
                            });
                            ins_addr += 2;
                        }
                        tokens.next();
                    }
                    Some(Token {token: TokenType::CommaY, ..}) => {
                        if value.long {
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                ins_addr,
                                addr: InterAddr::Addr(AddressType::AbsoluteY, Some(value))
                            });
                            ins_addr += 3;
                        } else {
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                ins_addr,
                                addr: InterAddr::Addr(AddressType::ZeroPageY, Some(value))
                            });
                            ins_addr += 2;
                        }
                        tokens.next();
                    },
                    _ => {
                        if value.long {
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                ins_addr,
                                addr: InterAddr::Addr(AddressType::Absolute, Some(value))
                            });
                            ins_addr += 3;
                        } else {
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                ins_addr,
                                addr: InterAddr::Addr(AddressType::ZeroPage, Some(value))
                            });
                            ins_addr += 2;
                        }
                    }
                } 
                state = PState::Default;
            },
        }
    }

    // print_instructions(&instructions);

    let mut result: Vec<OpCode> = vec![];
    for op in instructions.into_iter() {
        let parsed_op = match op.addr {
            InterAddr::Label(label) => {
                let label_addr = match labels.get(&label.text) {
                    Some(x) => Ok(*x),
                    None => Err(AsmError::new(&format!("Undefined label: {label:?}"), None)) 
                }?;
                if let Some(op_code) = op.instruct.get_op_code(&AddressType::Absolute) {
                    if label_addr > u16::MAX {
                        return Err(AsmError::new(&format!("Absolute address doesnt fit in u16: {label_addr}"), Some(label)))
                    }
                    let full_addr = label_addr;
                    let low: u8 = ((full_addr & 0xFF00) >> 8) as u8;
                    let high: u8 = (full_addr & 0x00FF) as u8;
                    OpCode {
                        code: op_code,
                        addr: vec![high, low]
                    }
                } else if let Some(op_code) = op.instruct.get_op_code(&AddressType::Relative) {
                    let diff = (label_addr as i32) - (op.ins_addr as i32) - 2;
                    OpCode {
                        code: op_code,
                        addr: vec![match i8::try_from(diff) {
                            Ok(val) => val as u8,
                            Err(_) => return Err(AsmError::new(&format!("Relative address doesnt fit in i8: {diff}"), Some(label)))
                        }]
                    }
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
                            OpCode {
                                code: op_code,
                                addr: vec![value.value as u8]
                            }
                        } else {
                            return Err(AsmError::new("Missing value", Some(op.symbol)))
                        }
                    },
                    AddressType::Impl | AddressType::Accumulator => {
                        if let Some(value) = value {
                            return Err(AsmError::new("Unexpected value", Some(value.symbol)))
                        } else if let Some(op_code) = op.instruct.get_op_code(&AddressType::Impl) {
                            OpCode {
                                code: op_code,
                                addr: vec![]
                            }
                        } else if let Some(op_code) = op.instruct.get_op_code(&AddressType::Accumulator) {
                            OpCode {
                                code: op_code,
                                addr: vec![]
                            }
                        } else {
                            return Err(AsmError::new(&format!("Instruction \"{:?}\" needs an address", op.instruct), Some(op.symbol)))
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
                            OpCode {
                                code: op_code,
                                addr: vec![high, low]
                            }
                        } else {
                            return Err(AsmError::new("Missing value", Some(op.symbol)))
                        }
                    },
                    _ => unimplemented!("{:?}", addr),
                }
            },
        };
        result.push(parsed_op);
    };
    return Ok(result)
}

fn print_instructions(instructions: &Vec<InterOpCode>) {
    for op in instructions {
        println!("{:?} => {}", op.instruct, match &op.addr {
            InterAddr::Label(s) => format!("Label({:?})", &s.text),
            InterAddr::Addr(addr, Some(val)) => format!("0x{:04x}: Addr({:?}, {:?})", op.ins_addr, &addr, (&val.symbol.text, &val.value)),
            InterAddr::Addr(addr, None) => format!("0x{:04x}: Addr({:?}, None)", op.ins_addr, &addr),
        });
    }
}

