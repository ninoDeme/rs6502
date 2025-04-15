use std::io;

mod asm;
mod instruct;

use asm::assemble;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().map(|l| l.unwrap()).collect();
    let res = assemble(lines, 0x0600);

    let mut i = 0;
    for op in res {
        if i % 16 == 0 {
            print!("\n {:04x}: ", i + 0x0600);
        }
        i += 1;
        print!("{:02x} ", op);
    }
    Ok(())
}
