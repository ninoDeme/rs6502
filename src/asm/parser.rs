use crate::asm::lexer::{Token, TokenType, Symbol};
use crate::asm::{Instruct, AddressType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OpCode {
    code: u8,
    addr: Vec<u8>
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

fn parse_number(token: Token, radix: Radix) -> Value {
    if token.token != TokenType::Number {
        panic!("Expected number, found {token:?}");
    };
    let value = i32::from_str_radix(token.symbol.text.as_str(), match radix {
        Radix::Bin => 2,
        Radix::Oct => 8,
        Radix::Dec => 10,
        Radix::Hex => 16,
    }).expect(format!("{} is not a valid number", token.symbol.text).as_str());
    let is16bit = value > 255 || token.symbol.text.chars().count() > match radix {
        Radix::Bin => 8,
        Radix::Oct => 4,
        Radix::Dec => 3,
        Radix::Hex => 2,
    };
    return Value {
        long: is16bit,
        symbol: token.symbol,
        value
    }
}

pub fn parse(tokens_vec: Vec<Token>) {
    // println!("{:?}", tokens);
    let mut tokens = tokens_vec.into_iter().peekable();
    let mut labels = HashMap::new();
    let mut state = PState::Default;

    let mut instructions: Vec<InterOpCode> = vec![];

    let mut curr_token = tokens.next();
    while let Some(token) = tokens.next() {
        match state {
            PState::Default => {
                match token.token {
                    TokenType::Identifier => {
                        if let Some(ins) = Instruct::from_str(token.symbol.text.as_str()) {
                            let ins_symbol = token.symbol;
                            if tokens.peek().is_some_and(|t| t.token == TokenType::Identifier) {
                                let token = tokens.peek().unwrap();
                                if is_keyword(token.symbol.text.as_str()) {
                                    instructions.push(InterOpCode {
                                        symbol: ins_symbol,
                                        instruct: ins,
                                        addr: InterAddr::Addr(AddressType::Impl, None)
                                    });
                                } else {
                                    let n_token = tokens.next().unwrap();
                                    instructions.push(InterOpCode {
                                        symbol: ins_symbol,
                                        instruct: ins,
                                        addr: InterAddr::Label(n_token.symbol)
                                    });
                                }
                                state = PState::Default;
                            } else {
                                state = PState::PostIntruction(ins_symbol, ins);
                            }
                        } else if token.symbol.text.to_lowercase() == "define" {
                            todo!("define statement");
                        } else if tokens.peek().is_some_and(|t| t.token == TokenType::Colon) {
                            tokens.next();
                            labels.insert(token.symbol.text.to_lowercase(), instructions.len());
                        }
                    }
                    _ => {
                        println!("{instructions:?}");
                        panic!("Invalid Token: {token:?}");
                    }
                }
            }
            PState::PostIntruction(ins_symbol, ins) => {
                match token.token {
                    TokenType::Hash => {
                        let mut token = tokens.next().expect("Expected number, found EOF");
                        let radix = match token.token {
                            TokenType::Bin => {
                                token = tokens.next().expect("Expected number, found EOF");
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

                        let value = parse_number(token, radix);
                        if value.long {
                            panic!("number can't be bigger than 8 bits");
                        }
                        instructions.push(InterOpCode {
                            symbol: ins_symbol,
                            instruct: ins,
                            addr: InterAddr::Addr(AddressType::Immediate, Some(value))
                        });
                        state = PState::Default;
                    },
                    TokenType::Bin | TokenType::Hex | TokenType::Oct => {
                        let radix = match token.token {
                            TokenType::Bin => Radix::Bin,
                            TokenType::Oct => Radix::Oct,
                            TokenType::Hex => Radix::Hex,
                            _ => unreachable!()
                        };

                        let token = tokens.next().expect("Expected number, found EOF");
                        state = PState::PostNumber(
                            ins_symbol,
                            ins,
                            parse_number(token, radix)
                        );
                    },
                    TokenType::Number => {
                        state = PState::PostNumber(
                            ins_symbol,
                            ins,
                            parse_number(token, Radix::Dec)
                        );
                    }
                    _ => {
                        unimplemented!();
                    }
                }
            }
            PState::PostNumber(ins_symbol, ins, value) => {
                let next = tokens.peek();
                if let Some(next_token) = next {
                    match next_token.token {
                        TokenType::CommaX => {
                            instructions.push(InterOpCode {
                                symbol: ins_symbol,
                                instruct: ins,
                                addr: InterAddr::Addr(if value.long {AddressType::AbsoluteX} else {AddressType::ZeroPageX}, Some(value))
                            });
                            tokens.next();
                        }
                        TokenType::CommaY => {
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
                } else {
                    instructions.push(InterOpCode {
                        symbol: ins_symbol,
                        instruct: ins,
                        addr: InterAddr::Addr(if value.long {AddressType::Absolute} else {AddressType::ZeroPage}, Some(value))
                    });
                    state = PState::Default;
                }
            }
        }
    }

    println!("{instructions:?}");
}
