use crate::asm::{Symbol, Pos};

#[derive(Debug)]
pub enum LState {
    Default,
    Identifier(Pos, String),
    Number(Pos, String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub symbol: Symbol,
    pub token: TokenType
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Identifier,
    Colon,
    Number,
    Hash,
    LParen,
    RParen,
    CommaX,
    CommaY,
    Hex,
    Bin,
    Oct,
    NewLine
}

pub fn lex<'a>(input: impl Iterator<Item = &'a String>) -> Vec<Token> {
    let lines = input.enumerate().map(|(i, l)| (i + 1, l));
    let mut tokens: Vec<Token> = vec![Token {
        token: TokenType::NewLine,
        symbol: Symbol {
            start: Pos {
                line: 0,
                col: 0
            },
            end: Pos {
                line: 0,
                col: 0
            },
            text: String::from("\n")
        }
    }];

    for (line_i, line) in lines {
        let mut state = LState::Default;
        let mut chars = line.chars();
        let mut col_i = 1;
        let mut char = chars.next();
        loop {
            if char == Some(';') {
                char = None;
            }
            match state {
                LState::Default => {
                    match char {
                        Some(':') => {
                            tokens.push(Token {
                                token: TokenType::Colon,
                                symbol: Symbol::new(line_i, col_i, String::from(':'))
                            });
                            char = chars.next();
                            col_i += 1;
                        }
                        Some('#') => {
                            tokens.push(Token {
                                token: TokenType::Hash,
                                symbol: Symbol::new(line_i, col_i, String::from('#'))
                            });
                            char = chars.next();
                            col_i += 1;
                        }
                        Some('$') => {
                            tokens.push(Token {
                                token: TokenType::Hex,
                                symbol: Symbol::new(line_i, col_i, String::from('$'))
                            });
                            char = chars.next();
                            col_i += 1;
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(""));
                        }
                        Some('@') => {
                            tokens.push(Token {
                                token: TokenType::Oct,
                                symbol: Symbol::new(line_i, col_i, String::from('@'))
                            });
                            char = chars.next();
                            col_i += 1;
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(""));
                        }
                        Some('%') => {
                            tokens.push(Token {
                                token: TokenType::Bin,
                                symbol: Symbol::new(line_i, col_i, String::from('%'))
                            });
                            char = chars.next();
                            col_i += 1;
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(""));
                        }
                        Some('(') => {
                            tokens.push(Token {
                                token: TokenType::LParen,
                                symbol: Symbol::new(line_i, col_i, String::from('('))
                            });
                            char = chars.next();
                            col_i += 1;
                        }
                        Some(')') => {
                            tokens.push(Token {
                                token: TokenType::RParen,
                                symbol: Symbol::new(line_i, col_i, String::from(')'))
                            });
                            char = chars.next();
                            col_i += 1;
                        }
                        Some(',') => {
                            match chars.next() {
                                Some(txt @ 'x') | Some(txt @ 'X') => {
                                    tokens.push(Token {
                                        token: TokenType::CommaX,
                                        symbol: Symbol::new(line_i, col_i, format!(",{txt}"))
                                    });
                                    char = chars.next();
                                    col_i += 2;
                                }
                                Some(txt @ 'y') | Some(txt @ 'Y') => {
                                    tokens.push(Token {
                                        token: TokenType::CommaY,
                                        symbol: Symbol::new(line_i, col_i, format!(",{txt}"))
                                    });
                                    char = chars.next();
                                    col_i += 2;
                                }
                                t => {
                                    panic!("Invalid char at pos {line_i}:{col_i} '{}'", t.map_or(String::from("EOF"), String::from));
                                }
                            };
                        }
                        Some(curr_char) if curr_char.is_whitespace() => {
                            char = chars.next();
                            col_i += 1;
                        }
                        Some(curr_char @ '0'..='9') => {
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(curr_char));
                            char = chars.next();
                            col_i += 1;
                        }
                        Some(curr_char @ ('a'..='z' | 'A'..='Z' | '_')) => {
                            state = LState::Identifier(Pos { line: line_i, col: col_i }, String::from(curr_char));
                            char = chars.next();
                            col_i += 1;
                        }
                        None => {
                            break;
                        }
                        Some(curr_char) => {
                            panic!("Invalid char at pos {line_i}:{col_i} '{}'", curr_char);
                        }
                    }
                }
                LState::Identifier(_, ref mut text) if matches!(char, Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9')) => {
                    text.push(char.unwrap());
                    char = chars.next();
                    col_i += 1;
                },
                LState::Identifier(start, text) => {
                    tokens.push(Token {
                        token: TokenType::Identifier,
                        symbol: Symbol {
                            start,
                            end: Pos {
                                line: line_i,
                                col: col_i
                            },
                            text: text.to_string()
                        }
                    });
                    state = LState::Default;
                }
                LState::Number(_, ref mut text) if char.is_some_and(|c| c.is_ascii_alphanumeric()) => {
                    text.push(char.unwrap());
                    char = chars.next();
                    col_i += 1;
                } 
                LState::Number(start, text) => {
                    if text.is_empty() {
                        panic!("Invalid char at pos {line_i}:{col_i}, expected number, found {}", char.map_or(String::from("NULL"), String::from));
                    }
                    tokens.push(Token {
                        token: TokenType::Number,
                        symbol: Symbol {
                            start,
                            end: Pos {
                                line: line_i,
                                col: col_i
                            },
                            text: text.to_string()
                        }
                    });
                    state = LState::Default;
                }
                // _ => {
                //     panic!("Not Implemented");
                // }
            }
        };

        tokens.push(Token {
            token: TokenType::NewLine,
            symbol: Symbol {
                start: Pos {
                    line: line_i,
                    col: col_i
                },
                end: Pos {
                    line: line_i + 1,
                    col: col_i
                },
                text: String::from("\n")
            }
        });
    }
    return tokens;
}

