use core::fmt;
use std::{fs, io::Read};
/* https://en.wikibooks.org/wiki/Super_NES_Programming/SNES_memory_map */

const MEMORY_SIZE: usize                = (0xFFFFFF) + 1;   // Total memory is addressable from 0x000000 - 0xFFFFFF
pub const MEMORY_END:  usize            = 0x7FFFFF;         // Memory commonly ends at bank $7F https://ersanio.gitbook.io/assembly-for-the-snes/the-fundamentals/memory
pub const MEMORY_BANK_COUNT: usize      =     0xFF;         // Number of addressable memory banks.
pub const MEMORY_BANK_SIZE: usize       =   0xFFFF;         // Size of one memory bank.
pub const MEMORY_BANK_INDEX: u8         = 16;               // Bit index to shift a u8 by to obtain a bank address.

/// Individual
/// TODO: Do I need to make a better structure for this?
const LOW_RAM_MIRROR: usize         =   0x0000;
const HW_REGISTERS: usize           =   0x2000;

type MemoryData = Box<[u8; MEMORY_SIZE]>;

/// Structure to represent memory.
/// Really just a wrapper for an array; we are doing this to avoid implementing file-scope global state.
pub struct Memory {
    memory: MemoryData
}

impl Memory {
    /// Creates a new instance of a Memory object, with all addresses initialized to 0.
    pub fn new() -> Self {
        Memory {
            // https://github.com/rust-lang/rust/issues/53827
            memory: vec![0; MEMORY_SIZE].into_boxed_slice().try_into().unwrap()
        }
    }

    /// Visually dump a bank to stdout.
    /// # Parameters
    ///     - `self`:           Pointer to object containing memory to dump.
    ///     - `target_bank`:    Target bank to print to screen.
    pub fn dump_bank(&self, target_bank: u8) {
        println!(
            "        0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x08 0x09 0x0A 0x0B 0x0C 0x0D 0x0E 0x0F"
        );
        let bank_index: usize = (target_bank as usize) << MEMORY_BANK_INDEX;

        for row_index in 0..(MEMORY_BANK_SIZE + 1) {
            let byte_index = bank_index | row_index;
            let byte_value = &self.memory[byte_index];

            if byte_index % 0x10 == 0
            {
                // Stupid: The width specifier for hex formatting applies to the leading "0x" also; all widths must be +2.
                print!("\n{:#06X}: [", byte_index);
                print!(" {:#04X}, ", byte_value);
            }
            else if byte_index % 0x10 == 0x0F
            {
                print!("{:#04X} ]", byte_value);
            }
            else {
                print!("{:#04X}, ", byte_value);
            }
        }
    }

    /// Return one byte from memory.
    /// # Parameters:
    ///     - `self`: Pointer to memory object which contains memory to read from.
    ///     - `address`: Address of byte to fetch.
    /// # Returns:
    ///     - `Ok(value)`   If the argument was valid
    ///     - `Err(e)`      If the arument was invalid
    pub fn get_byte(&self, address: usize) -> Result<u8, AddressOutOfBoundsError> {
        match address_is_valid(address) {
            Ok(_t) => {
                Ok(self.memory[address])
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Return a constructed word from memory.
    /// # Parameters:
    ///     - `self`: Pointer to memory object which contains memory to read from.
    ///     - `address`: Address of byte to fetch.
    /// # Returns:
    ///     - `Ok(value)`   If the argument was valid
    ///     - `Err(e)`      If the arument was invalid
    pub fn get_word(&self, address: usize) -> Result<u16, AddressOutOfBoundsError> {
        match address_is_valid(address) {
            Ok(_t) => {
                Ok(self.memory[address] as u16 | (self.memory[address + 1] as u16) << 8)
            }
            Err(e) => {
                Err(e)
            }
        }
        
    }

    /// Put a byte into memory.
    /// # Parameters:
    ///     - `self`:       Pointer to mutable memory object to write word into.
    ///     - `address`:    Location in memory to write to.
    ///     - `word`:       Word to write.
    pub fn put_byte(&mut self, address: usize, byte: u8) -> Result<(), AddressOutOfBoundsError> {
        match address_is_valid(address) {
            Ok(_t) => {
                self.memory[address] = byte;
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Put a word into memory.
    /// # Parameters:
    ///     - `self`:       Pointer to mutable memory object to write word into.
    ///     - `address`:    Location in memory to write to.
    ///     - `word`:       Word to write.
    pub fn put_word(&mut self, address: usize, word: u16) -> Result<(), AddressOutOfBoundsError> {
        match address_is_valid(address) {
            Ok(_t) => {
                self.memory[address]        = ((word & 0xFF00) >> 8) as u8;
                self.memory[address + 1]    = (word & 0x00FF) as u8;
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

/// Struct for an OutOfBoundsError on being passed a bad index.
#[derive(Debug, Clone)]
pub struct AddressOutOfBoundsError {
    addr: usize
}

impl AddressOutOfBoundsError {
    pub fn new(addr: usize) -> Self {
        Self {
            addr: addr
        }
    }
}

impl fmt::Display for AddressOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Address index {:#08x} out of bounds", self.addr)
    }
}

pub trait IntoWord {
    fn to_word(self) -> u16;
}

impl IntoWord for &[u8; 2] {
    fn to_word(self) -> u16 {
        ((self[0] as u16) >> 8 | (self[1] as u16) << 8) as u16
    }
}

/***** File-scope functions *****/

/// Given an 8-bit bank reference and a 16-bit address within that bank, return the composed address that points to.
/// # Parameters:
///     - `bank`:       An 8-bit bank address.
///     - `byte_addr`:  A 16-bit address within a bank.
/// # Returns:
///     - Composed full address of a byte in memory.
pub fn compose_address(bank: u8, byte_addr: u16) -> usize {
    ((bank as usize) << MEMORY_BANK_INDEX) | byte_addr as usize
}

/// Checks if an address is within range.
/// # Parameters:
///     - `address`     An address to test.
/// # Returns:
///     - `Ok(true)` for a valid address
///     - `AddressOutOfBoundsError(address)` for an invalid address
pub fn address_is_valid(address: usize) -> Result<(), AddressOutOfBoundsError> {
    if address <= MEMORY_SIZE {
        Ok(())
    }
    else {
        Err(AddressOutOfBoundsError::new(address))
    }
}

/***** Tests *****/
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    /// Given a memory ptr fill it with random test data, and return the random test data it was filled from.
    /// # Parameters:
    ///     - `memory_under_test`:      Memory to fill.
    /// # Returns:
    ///     - `random_data`:            Randomly generated list of u8s.
    fn fill_random(memory_under_test: &mut Memory) -> MemoryData {
        let mut random_data: MemoryData = vec![0; MEMORY_SIZE].into_boxed_slice().try_into().unwrap();

        for addr in 0 .. MEMORY_END {
            let rand_byte: u8 = rand::thread_rng().gen();
            random_data[addr] = rand_byte;
            memory_under_test.memory[addr] = rand_byte;
        }

        random_data
    }

    #[test]
    fn test_put_byte() {
        let mut memory_under_test = Memory::new();
        let mut random_data: Box<[u8; MEMORY_SIZE]> = vec![0; MEMORY_SIZE].into_boxed_slice().try_into().unwrap();

        for addr in 0 .. MEMORY_END {
            let rand_byte: u8 = rand::thread_rng().gen();
            random_data[addr] = rand_byte;
            memory_under_test.put_byte(addr, rand_byte).unwrap();
        }

        for addr in 0 .. MEMORY_END {
            assert_eq!(random_data[addr], memory_under_test.memory[addr]);
        }
    }

    #[test]
    #[should_panic]
    fn test_put_invalid_byte() {
        let mut memory_under_test = Memory::new();
        memory_under_test.put_byte(MEMORY_SIZE, 0).unwrap();
    }

    #[test]
    fn test_get_byte() {
        let mut memory_under_test = Memory::new();
        let random_data: MemoryData = fill_random(&mut memory_under_test);
 
        for addr in 0 .. MEMORY_END {
            assert_eq!(random_data[addr], memory_under_test.get_byte(addr).unwrap());
        }
    }

    #[test]
    #[should_panic]
    fn test_get_invalid_byte() {
        let memory_under_test = Memory::new();
        let _ = &memory_under_test.get_byte(MEMORY_SIZE).unwrap();
    }
}