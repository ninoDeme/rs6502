use std::io;

mod asm;

use asm::lexer::{lex};
use asm::parser::{parse};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().map(|l| l.unwrap()).collect();
    let tokens = lex(lines.iter());
    
    // println!("{:?}", tokens);
    let res = parse(tokens);
    match res {
        Ok(value) => {
            let mut i = 0;
            for op in value {
                if i % 16 == 0 {
                    print!("\n 0x{:04x}: ", i + 0x0600);
                }
                i += 1;
                print!("{:02x} ", op.code);
                for arg in op.addr {
                    if i % 16 == 0 {
                        print!("\n 0x{:04x}: ", i + 0x0600);
                    }
                    i += 1;
                    print!("{:02x} ", arg);
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

