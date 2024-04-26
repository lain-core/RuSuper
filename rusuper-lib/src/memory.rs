use std::{fs, io::{self, Read, Seek}};
/* https://en.wikibooks.org/wiki/Super_NES_Programming/SNES_memory_map */

const MEMORY_SIZE: usize = 0xFFFF;

pub struct Memory {
    memory: [u8; MEMORY_SIZE]
}

impl Memory {
    pub const fn new() -> Self {
        Memory {
            memory: [0; MEMORY_SIZE],
        }
    }
}

pub fn load(memory: &mut Memory, mut file: fs::File) {
    while let Ok(bytes_read) = file.read(&mut memory.memory){
        if bytes_read == 0 { break; }
    }
}
