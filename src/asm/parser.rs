
use crate::asm::lexer::{Token};

pub fn parse(tokens: Vec<Token>) {
    println!("{:?}", tokens);
    let mut tokens_iter = tokens.into_iter();
    let mut token = tokens_iter.next();

    loop {
        if token.is_none() {
            break;
        } else {
            println!("{:?}", token.unwrap().symbol.text);
        }
        token = tokens_iter.next();
    }
}
