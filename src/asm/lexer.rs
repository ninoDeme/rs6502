#[derive(Debug)]
pub enum LState {
    NewLine,
    Identifier(Pos, String),
    Number(Pos, String),
}

#[derive(Debug)]
pub struct Pos {
    line: usize,
    col: usize
}

#[derive(Debug)]
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
pub struct Token {
    pub symbol: Symbol,
    pub token: TokenType
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Address,
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
    Oct
}

pub fn lex(input: impl Iterator<Item = String>) -> Vec<Token> {
    let mut lines = input.enumerate().map(|(i, l)| (i + 1, l));
    let mut tokens: Vec<Token> = vec![];

    while let Some((line_i, line)) = lines.next() {
        let mut state = LState::NewLine;
        let mut chars = line.chars().peekable();
        let mut col_i = 1;
        let mut char = chars.next();
        loop {
            if char == Some(';') {
                char = None;
            }
            match state {
                LState::NewLine => {
                    match char {
                        Some(':') => {
                            tokens.push(Token {
                                    token: TokenType::Colon,
                                    symbol: Symbol::new(line_i, col_i, String::from(':'))
                                });
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some('#') => {
                            tokens.push(Token {
                                token: TokenType::Hash,
                                symbol: Symbol::new(line_i, col_i, String::from('#'))
                            });
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some('$') => {
                            tokens.push(Token {
                                token: TokenType::Hex,
                                symbol: Symbol::new(line_i, col_i, String::from('$'))
                            });
                            char = chars.next();
                            col_i = col_i + 1;
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(""));
                        }
                        Some('@') => {
                            tokens.push(Token {
                                token: TokenType::Oct,
                                symbol: Symbol::new(line_i, col_i, String::from('@'))
                            });
                            char = chars.next();
                            col_i = col_i + 1;
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(""));
                        }
                        Some('%') => {
                            tokens.push(Token {
                                token: TokenType::Bin,
                                symbol: Symbol::new(line_i, col_i, String::from('%'))
                            });
                            char = chars.next();
                            col_i = col_i + 1;
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(""));
                        }
                        Some('(') => {
                            tokens.push(Token {
                                token: TokenType::LParen,
                                symbol: Symbol::new(line_i, col_i, String::from('('))
                            });
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some(')') => {
                            tokens.push(Token {
                                token: TokenType::RParen,
                                symbol: Symbol::new(line_i, col_i, String::from(')'))
                            });
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some(',') => {
                            match chars.next() {
                                Some(txt @ 'x') | Some(txt @ 'X') => {
                                    tokens.push(Token {
                                        token: TokenType::CommaX,
                                        symbol: Symbol::new(line_i, col_i, format!(",{txt}"))
                                    });
                                    char = chars.next();
                                    col_i = col_i + 2;
                                }
                                Some(txt @ 'y') | Some(txt @ 'Y') => {
                                    tokens.push(Token {
                                        token: TokenType::CommaY,
                                        symbol: Symbol::new(line_i, col_i, format!(",{txt}"))
                                    });
                                    char = chars.next();
                                    col_i = col_i + 2;
                                }
                                t => {
                                    panic!("Invalid char at pos {line_i}:{col_i} '{}'", t.map_or(String::from("EOF"), |c| String::from(c)));
                                }
                            };
                        }
                        Some(curr_char) if curr_char.is_whitespace() => {
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some(curr_char @ '0'..='9') => {
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(curr_char));
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some(curr_char) if curr_char.is_ascii_alphabetic() => {
                            state = LState::Identifier(Pos { line: line_i, col: col_i }, String::from(curr_char));
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        None => {
                            break;
                        }
                        Some(curr_char) => {
                            panic!("Invalid char at pos {line_i}:{col_i} '{}'", curr_char);
                        }
                    }
                }
                LState::Identifier(_, ref mut text) if char.is_some_and(|c| c.is_ascii_alphanumeric()) => {
                    text.push(char.unwrap());
                    char = chars.next();
                    col_i = col_i + 1;
                },
                LState::Identifier(start, text) => {
                    tokens.push(Token {
                        token: TokenType::Identifier,
                        symbol: Symbol {
                            start: start,
                            end: Pos {
                                line: line_i,
                                col: col_i
                            },
                            text: text.to_string()
                        }
                    });
                    state = LState::NewLine;
                }
                LState::Number(_, ref mut text) if char.is_some_and(|c| c.is_ascii_alphanumeric()) => {
                    text.push(char.unwrap());
                    char = chars.next();
                    col_i = col_i + 1;
                } 
                LState::Number(start, text) => {
                    if text == "" {
                        panic!("Invalid char at pos {line_i}:{col_i}, expected number, found {}", char.map_or(String::from("NULL"), |c| String::from(c)));
                    }
                    tokens.push(Token {
                        token: TokenType::Number,
                        symbol: Symbol {
                            start: start,
                            end: Pos {
                                line: line_i,
                                col: col_i
                            },
                            text: text.to_string()
                        }
                    });
                    state = LState::NewLine;
                }
                // _ => {
                //     panic!("Not Implemented");
                // }
            }
        }
    }
    return tokens;
}

