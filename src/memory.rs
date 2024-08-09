use crate::romdata;
use core::fmt;

/**************************************** Constant Values ***************************************************************/
pub const MEMORY_SIZE: usize = (0xFFFFFF) + 1;
pub const MEMORY_START: usize = 0x000000;
pub const MEMORY_END: usize = 0xFFFFFF;
pub const _MEMORY_BANK_COUNT: usize = 0xFF; // Number of addressable memory banks.
pub const _MEMORY_BANK_START: usize = 0x0000;
pub const _MEMORY_BANK_SIZE: usize = 0xFFFF; // Size of one memory bank.
pub const MEMORY_BANK_INDEX: u8 = 16; // Bit index to shift a u8 by to obtain a bank address.

/**************************************** Struct and Type definitions ***************************************************/
/// Wrapper type for a u8 array which represents memory.
type MemoryData = Box<[u8; MEMORY_SIZE]>;

/// Struct for an Invalid Address or an index out of bounds.
#[derive(Debug, Clone)]
pub struct InvalidAddressError {
    addr: usize,
}

impl InvalidAddressError {
    pub fn new(addr: usize) -> Self {
        Self { addr }
    }
}

impl fmt::Display for InvalidAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Address index {:#08x} out of bounds", self.addr)
    }
}

/// Structure to represent memory.
/// Really just a wrapper for an array.
pub struct Memory {
    memory: MemoryData,
}

impl Memory {
    /// Creates a new instance of a Memory object, with all addresses initialized to 0.
    pub fn new() -> Self {
        Memory {
            // https://github.com/rust-lang/rust/issues/53827
            memory: vec![0; MEMORY_SIZE].into_boxed_slice().try_into().unwrap(),
        }
    }

    /// Print out a few rows of memory.
    pub fn print_bytes(&self, address: Option<usize>) {
        let start_addr = match address {
            Some(start) => start,
            None => 0x808000,
        };

        print!("\n0x|");
        for i in 0..16 {
            print!("{:02X} ", i);
        }
        println!("\n==================================================");

        for i in 0..8 {
            print!("{:02x}|", i);
            for j in 0..16 {
                print!("{:02x} ", self.memory[start_addr + (16 * i) + j]);
            }
            println!();
        }
    }

    /// Return one byte from memory.
    /// # Parameters:
    ///     - `self`: Pointer to memory object which contains memory to read from.
    ///     - `address`: Address of byte to fetch, as a fully assembled absolute address.
    /// # Returns:
    ///     - `Ok(value)`                   If OK, the byte as a wrapped u8.
    ///     - `InvalidAddressError(e)`      If an invalid address was passed.
    pub fn get_byte(&self, address: usize) -> Result<u8, InvalidAddressError> {
        match address_is_valid(address) {
            Ok(_t) => Ok(self.memory[address]),
            Err(e) => Err(e),
        }
    }

    /// Put a byte into memory.
    /// # Parameters:
    ///     - `self`:       Pointer to mutable memory object to write byte into.
    ///     - `address`:    Location in memory to write to, as a fully assembled absolute address.
    ///     - `byte`:       Byte to write.
    /// # Returns:
    ///     - `Ok(())`:                     If OK.
    ///     - `InvalidAddressError(e)`:     If an invalid address was passed.
    pub fn put_byte(&mut self, address: usize, byte: u8) -> Result<(), InvalidAddressError> {
        match address_is_valid(address) {
            Ok(_t) => {
                self.memory[address] = byte;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Write an entire bank into memory.
    /// # Parameters:
    ///     - `self`
    ///     - `banktype`:       The size of a rom, used to determine the bank length.
    ///     - `address`:        Start address to write this bank from, as an absolute address.
    ///     - `bankdata`:       Data to write to target bank.
    /// # Returns
    ///     - `Ok(())`:                     If written OK.
    ///     - `InvalidAddressError(e)`:     If an invalid address was passed.
    pub fn put_bank(
        &mut self, banktype: romdata::BankSize, address: usize, bankdata: &[u8],
    ) -> Result<(), InvalidAddressError> {
        match address_is_valid(address + banktype as usize - 1) {
            Ok(_t) => {
                for offset in 0..banktype as usize - 1 {
                    self.memory[address + offset] = bankdata[offset];
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Put a big endian word into memory.
    /// # Parameters:
    ///     - `self`:       Pointer to mutable memory object to write word into.
    ///     - `address`:    Absolute address location in memory to write to (for byte 0).
    ///     - `word`:       Word to write.
    /// # Returns:
    ///     - `Ok(())`:                     If OK.
    ///     - `InvalidAddressError`:        If an invalid address was passed.
    pub fn _put_word(&mut self, address: usize, word: u16) -> Result<(), InvalidAddressError> {
        match address_is_valid(address + 1) {
            Ok(_t) => {
                self.memory[address] = word.to_le_bytes()[0];
                self.memory[address + 1] = word.to_le_bytes()[1];
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Fetch a word from memory in Big Endian format, for readability.
    /// # Parameters:
    ///     - `self`:       Pointer to memory object which contains memory to read from.
    ///     - `address`:    Absolute address of byte to fetch.
    /// # Returns:
    ///     - `Ok(value)`   If OK, the word as a wrapped u16.
    ///     - `Err(e)`      If an invalid address was passed.
    pub fn get_word(&self, address: usize) -> Result<u16, InvalidAddressError> {
        match address_is_valid(address + 1) {
            Ok(_t) => {
                let word_val: u16 =
                    u16::from_le_bytes([self.memory[address], self.memory[address + 1]]);
                Ok(word_val)
            }
            Err(e) => Err(e),
        }
    }
}

/**************************************** File Scope Functions **********************************************************/

/// Given an 8-bit bank reference and a 16-bit address within that bank, return the composed address that points to.
/// # Parameters:
///     - `bank`:       An 8-bit bank address.
///     - `byte_addr`:  A 16-bit address within a bank.
/// # Returns:
///     - Composed full address of a byte in memory.
pub fn compose_address(bank: u8, byte_addr: u16) -> usize {
    ((bank as usize) << MEMORY_BANK_INDEX) | byte_addr as usize
}

/// Checks if an address is within memory range (0x000000 - 0xFFFFFF).
/// # Parameters:
///     - `address`     Address to test.
/// # Returns:
///     - `Ok(true)`                        For a valid address.
///     - `InvalidAddressError(address)`    For an invalid address (>= 0xFFFFFF).
pub fn address_is_valid(address: usize) -> Result<(), InvalidAddressError> {
    if address < MEMORY_SIZE {
        Ok(())
    } else {
        println!("Error");
        Err(InvalidAddressError::new(address))
    }
}

/**************************************** Tests *************************************************************************/
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, RngCore};

    const MEMORY_END: usize = 0xFFFFFF;

    /**************************************** Test Helpers **************************************************************/
    /// Given a memory ptr fill it with random test data, and return the random test data it was filled from.
    /// # Parameters:
    ///     - `memory_under_test`:      Memory to fill.
    /// # Returns:
    ///     - `random_data`:            Randomly generated list of u8s.
    fn fill_random(memory_under_test: &mut Memory) -> MemoryData {
        let mut random_data: MemoryData =
            vec![0; MEMORY_SIZE].into_boxed_slice().try_into().unwrap();
        rand::thread_rng().fill_bytes(&mut *random_data);

        for addr in 0..MEMORY_SIZE {
            memory_under_test.memory[addr] = random_data[addr];
        }

        random_data
    }

    /**************************************** Unit Test Implementations *************************************************/
    /***** Byte Tests *****/

    #[test]
    fn test_put_byte() {
        let mut memory_under_test = Memory::new();
        let mut random_data: Box<[u8; MEMORY_SIZE]> =
            vec![0; MEMORY_SIZE].into_boxed_slice().try_into().unwrap();
        rand::thread_rng().fill_bytes(&mut *random_data);

        for addr in 0..MEMORY_END {
            memory_under_test.put_byte(addr, random_data[addr]).unwrap();
        }

        for addr in 0..MEMORY_END {
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

        for addr in 0..MEMORY_END {
            assert_eq!(random_data[addr], memory_under_test.get_byte(addr).unwrap());
        }
    }

    #[test]
    #[should_panic]
    fn test_get_invalid_byte() {
        let memory_under_test = Memory::new();
        let _ = &memory_under_test.get_byte(MEMORY_SIZE).unwrap();
    }

    /***** Word Tests *****/
    #[test]
    fn test_put_word() {
        let mut memory_under_test: Memory = Memory::new();
        let mut random_data: Box<[u16; MEMORY_SIZE / 2]> = vec![0; MEMORY_SIZE / 2]
            .into_boxed_slice()
            .try_into()
            .unwrap();

        memory_under_test._put_word(0x000000, 0xAABB).unwrap();
        assert_eq!(memory_under_test.memory[0], 0xBB);
        assert_eq!(memory_under_test.memory[1], 0xAA);

        let mut rand_word: u16;
        for addr in 0..MEMORY_SIZE {
            if addr % 2 == 0 {
                rand_word = rand::thread_rng().gen();
                random_data[addr / 2] = rand_word;
                memory_under_test._put_word(addr, rand_word).unwrap();
            }
        }

        for addr in 0..MEMORY_SIZE {
            if addr % 2 == 0 {
                let test_word: u16 = u16::from_le_bytes([
                    memory_under_test.memory[addr],
                    memory_under_test.memory[addr + 1],
                ]);
                assert_eq!(random_data[addr / 2], test_word);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_put_invalid_word() {
        let mut memory_under_test: Memory = Memory::new();
        memory_under_test._put_word(MEMORY_SIZE + 1, 0).unwrap();
    }

    #[test]
    fn test_get_word() {
        let mut memory_under_test: Memory = Memory::new();
        let mut random_data: Box<[u16; MEMORY_SIZE / 2]> = vec![0; MEMORY_SIZE / 2]
            .into_boxed_slice()
            .try_into()
            .unwrap();

        // Quick sanity check to make sure our get_word() gives back a BE value.
        memory_under_test.memory[0] = 0xBB;
        memory_under_test.memory[1] = 0xAA;
        assert_eq!(memory_under_test.get_word(0x000000).unwrap(), 0xAABB);

        let mut rand_word: u16;
        for addr in 0..MEMORY_SIZE {
            if addr % 2 == 0 {
                rand_word = rand::thread_rng().gen();
                random_data[addr / 2] = rand_word;
                memory_under_test._put_word(addr, rand_word).unwrap();
            }
        }

        for addr in 0..MEMORY_SIZE {
            if addr % 2 == 0 {
                let test_word: u16 = u16::from_le_bytes([
                    memory_under_test.memory[addr],
                    memory_under_test.memory[addr + 1],
                ]);
                assert_eq!(random_data[addr / 2], test_word);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_get_invalid_word() {
        let memory_under_test: Memory = Memory::new();
        memory_under_test.get_word(MEMORY_SIZE).unwrap();
    }
}
