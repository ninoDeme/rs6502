use std::fmt;

pub trait Memory: fmt::Debug {
    fn get(&mut self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, value: u8) -> ();
}

#[derive(Debug)]
pub struct DefaultMemory {
    memory: [u8; 65536],
}

impl DefaultMemory {
    pub fn new() -> Self {
        DefaultMemory {
            memory: [0; 65536]
        }
    }
}

impl Memory for DefaultMemory {
    fn get(&mut self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }
    fn set(&mut self, addr: u16, value: u8) -> () {
        self.memory[addr as usize] = value;
    }
}

