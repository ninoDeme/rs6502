

use std::io;

mod asm;

use asm::lexer::{lex};
use asm::parser::{parse};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lines().map(|l| l.unwrap());
    let tokens = lex(lines);
    
    // println!("{:?}", tokens);
    parse(tokens);
    println!("Pronto!");
    Ok(())
}

