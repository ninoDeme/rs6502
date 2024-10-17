

use std::io;

mod asm;

use asm::lexer::{lex};
use asm::parser::{parse};

const CHARS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

fn to_hex(num: u8) -> String {
    let low: char = CHARS[(num & 0x0F) as usize];
    let high: char = CHARS[((num & 0xF0) >> 4) as usize];
    return format!("{high}{low}");
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().map(|l| l.unwrap()).collect();
    let tokens = lex(lines.iter());
    
    // println!("{:?}", tokens);
    let res = parse(tokens);
    match res {
        Ok(value) => {
            let mut first = false;
            for op in value {
                if !first {
                    first = true;
                } else {
                    print!(" ");
                }
                print!("{}", to_hex(op.code));
                for arg in op.addr {
                    print!(" {}", to_hex(arg));
                }
            }
        },
        Err(error) => {
            println!("ERROR: {}", error.reason);
            if let Some(symbol) = error.symbol {
                println!(" -> STDIN:{}:{}", symbol.start.line, symbol.start.col + 1);
                println!("{} | {}", symbol.start.line, lines[symbol.start.line - 1]);
                let n_width = format!("{}", symbol.start.line).len();
                let start = symbol.start.col;
                let width = symbol.end.col - symbol.start.col;
                println!("{} | {}{}", " ".repeat(n_width), " ".repeat(start - 1), "~".repeat(width));
            }
        }
    }
    Ok(())
}

