use std::fs::File;

use std::io;
use std::io::BufRead;
use std::path::Path;
use std::collections::BTreeMap;

use crate::asm::lexer::lex;
use crate::asm::parser::parse;
pub mod lexer;
pub mod parser;

#[derive(Debug, Clone)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub start: Pos,
    pub end: Pos,
    pub text: String,
}

impl Symbol {
    pub fn new(line: usize, col: usize, text: String) -> Symbol {
        Symbol {
            start: Pos { line, col },
            end: Pos {
                line,
                col: col + text.chars().count(),
            },
            text,
        }
    }
}

#[derive(Debug)]
pub struct AsmError {
    pub symbol: Option<Symbol>,
    pub reason: String,
}

impl AsmError {
    pub fn new(reason: &str, symbol: Option<Symbol>) -> AsmError {
        return AsmError {
            symbol,
            reason: String::from(reason),
        };
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn assemble(input: Vec<String>) -> BTreeMap<u16, u8> {
    let tokens = lex(input.iter());

    // println!("{:?}", tokens);
    let res = parse(tokens);
    match res {
        Ok(value) => {
            return value;
        }
        Err(error) => {
            println!("ERROR: {}", error.reason);
            if let Some(symbol) = error.symbol {
                println!(" -> STDIN:{}:{}", symbol.start.line, symbol.start.col + 1);
                println!("{} | {}", symbol.start.line, input[symbol.start.line - 1]);
                let n_width = format!("{}", symbol.start.line).len();
                let start = symbol.start.col;
                let width = symbol.end.col - symbol.start.col;
                println!(
                    "{} | {}{}",
                    " ".repeat(n_width),
                    " ".repeat(start - 1),
                    "~".repeat(width)
                );
            }
            panic!("");
        }
    }
}
