use std::io;

#[derive(Debug)]
enum Radix {
    Hex,
    Dec,
    Oct,
    Bin
}

#[derive(Debug)]
enum LState {
    NewLine,
    Identifier(Pos, String),
    Number(Pos, String),
}

#[derive(Debug)]
struct Pos {
    line: usize,
    col: usize
}

#[derive(Debug)]
struct Symbol {
    start: Pos,
    end: Pos,
    text: String
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
enum Token {
    Address(Symbol),
    Identifier(Symbol),
    Colon(Symbol),
    Number(Symbol),
    Hash(Symbol),
    LParen(Symbol),
    RParen(Symbol),
    Comma(Symbol),
    Hex(Symbol),
    Bin(Symbol),
    Oct(Symbol)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lines().map(|l| l.unwrap());
    let tokens = lex(lines);
    
    println!("{:?}", tokens);
    Ok(())
}

fn lex(input: impl Iterator<Item = String>) -> Vec<Token> {
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
                            tokens.push(Token::Colon(Symbol::new(line_i, col_i, String::from(':'))));
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some('#') => {
                            tokens.push(Token::Hash(Symbol::new(line_i, col_i, String::from('#'))));
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some('$') => {
                            tokens.push(Token::Hex(Symbol::new(line_i, col_i, String::from('$'))));
                            char = chars.next();
                            col_i = col_i + 1;
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(""));
                        }
                        Some('@') => {
                            tokens.push(Token::Oct(Symbol::new(line_i, col_i, String::from('@'))));
                            char = chars.next();
                            col_i = col_i + 1;
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(""));
                        }
                        Some('%') => {
                            tokens.push(Token::Bin(Symbol::new(line_i, col_i, String::from('%'))));
                            char = chars.next();
                            col_i = col_i + 1;
                            state = LState::Number(Pos { line: line_i, col: col_i }, String::from(""));
                        }
                        Some('(') => {
                            tokens.push(Token::LParen(Symbol::new(line_i, col_i, String::from('('))));
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some(')') => {
                            tokens.push(Token::RParen(Symbol::new(line_i, col_i, String::from(')'))));
                            char = chars.next();
                            col_i = col_i + 1;
                        }
                        Some(',') => {
                            tokens.push(Token::Comma(Symbol::new(line_i, col_i, String::from(','))));
                            char = chars.next();
                            col_i = col_i + 1;
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
                    tokens.push(Token::Identifier(Symbol {
                        start: start,
                        end: Pos {
                            line: line_i,
                            col: col_i
                        },
                        text: text.to_string()
                    }));
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
                    tokens.push(Token::Number(Symbol {
                        start: start,
                        end: Pos {
                            line: line_i,
                            col: col_i
                        },
                        text: text.to_string()
                    }));
                    state = LState::NewLine;
                }
                _ => {
                    panic!("Not Implemented");
                }
            }
        }
    }
    return tokens;
}

