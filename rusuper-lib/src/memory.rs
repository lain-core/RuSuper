use std::{fs, io::{BufReader, Read}};
/* https://en.wikibooks.org/wiki/Super_NES_Programming/SNES_memory_map */

const MEMORY_SIZE: usize            = 0xFFFFFF;
const MEMORY_END:  usize            = 0x7FFFFF; /// Memory commonly ends at bank $7F https://ersanio.gitbook.io/assembly-for-the-snes/the-fundamentals/memory
const MEMORY_BANK_COUNT: usize      =     0xFF; /// Number of addressable memory banks.
const MEMORY_BANK_SIZE: usize       =   0xFFFF; /// Size of one memory bank.
const MEMORY_BANK_INDEX: u8         = 16;       /// Bit index to shift a u8 by to obtain a bank address.

/// Individual
/// TODO: Do I need to make a better structure for this?
const LOW_RAM_MIRROR: usize         =   0x0000;
const HW_REGISTERS: usize           =   0x2000;

pub struct Memory {
    memory: Vec<u8>
}

impl Memory {
    
    /// Creates a new instance of a Memory object, with all addresses initialized to 0.
    pub fn new() -> Self {
        let mut new_memory = Memory {
            memory: Vec::with_capacity(MEMORY_SIZE)
        };

        new_memory
    }

    /// Visually dump a bank to stdout.
    /// # Parameters
    ///     - `target_bank`:    Target bank to print to screen.
    pub fn dump_bank(&self, target_bank: u8) {
        println!(
            "        0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x08 0x09 0x0A 0x0B 0x0C 0x0D 0x0E 0x0F"
        );
        let mut baseaddr: usize = (target_bank as usize) << MEMORY_BANK_INDEX;

        for byte_bank_index in 0..(MEMORY_BANK_SIZE + 1) {
            if byte_bank_index % 0x10 == 0
            {
                print!("\n{:#06X}: [", (baseaddr | byte_bank_index));
                print!(" {:#04X}, ", &self.memory[baseaddr | byte_bank_index]);
            }
            else if byte_bank_index % 0x10 == 0x0F
            {
                print!("{:#04X} ]", &self.memory[baseaddr | byte_bank_index]);
            }
            else {
                print!("{:#04X}, ", &self.memory[baseaddr | byte_bank_index]);
            }
        }
    }

    /// Load a ROM image into memory.
    /// # Parameters:
    ///     - `self`:    Pointer to memory object which contains memory to write file to.
    ///     - `file`:    file to read the data from
    pub fn load_rom(&mut self, mut file: fs::File) {
        let read_result = file.read_to_end(&mut self.memory).unwrap();
        println!("Read {} bytes", read_result);
    }
}
