use std::{fs, io::{BufReader, Read}};
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

    pub fn dump(&self) {
        println!(
            "0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x08 0x09 0x0A 0x0B 0x0C 0x0D 0x0E 0x0F"
        );
        let mut baseaddr: u16 = 0x0000;
        // for value in &self.memory{
        //     if value % 16 == 0
        //     {
        //         print!("{:#02X}: [", baseaddr);
        //         baseaddr += 0x10;
        //     }
        //     print!("{:02X} ", value);
        // }
    }
}

pub fn load(memory: &mut Memory, mut file: fs::File) {
    let mut buf: Vec<u8> = vec![];
    let result = file.read_to_end(&mut buf).unwrap();

    for i in 0..16{
        print!("{:02X} ", buf[i]);
    }

    println!("Read {} bytes", result);
}
