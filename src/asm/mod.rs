pub mod lexer;
pub mod parser;

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
