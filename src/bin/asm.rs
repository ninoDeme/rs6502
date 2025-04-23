use std::io;
use std::env;
use std::fs;

use rs6502::asm::assemble;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let lines: Vec<String> = if args.len() < 2 {
        let stdin = io::stdin();
        stdin.lines().map(|l| l.unwrap()).collect()
    } else {
        let file_name = &args[1];
        fs::read_to_string(file_name).expect("Invalid file").lines().map(|v| v.to_string()).collect()
    };
    let res = assemble(lines);

    for high in 0x000..=0xFFF {
        let mut line: [u8; 0x10] = [0; 0x10];
        let mut has_byte = false;
        for low in 0x0..=0xF {
            if let Some(byte) = res.get(&(low + (high * 0x10))) {
                line[low as usize] = *byte;
                has_byte = true;
            } 
        }
        if has_byte {
            print!("{:04x}: ", high * 0x10);
            for byte in line {
                print!("{:02x} ", byte);
            }
            print!("\n");
        }
    }
    Ok(())
}
