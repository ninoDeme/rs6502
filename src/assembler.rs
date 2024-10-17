

use std::io;

mod asm;

use asm::lexer::{lex};
use asm::parser::{parse};

const chars: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

fn to_hex(num: u8) -> String {
    let low: char = chars[(num & 0x0F) as usize];
    let high: char = chars[((num & 0xF0) >> 4) as usize];
    return format!("{high}{low}");
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lines().map(|l| l.unwrap());
    let tokens = lex(lines);
    
    // println!("{:?}", tokens);
    let res = parse(tokens);
    let mut first = false;
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
            panic!("{error:?}");
        }
    }
    Ok(())
}

