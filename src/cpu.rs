use crate::memory;

mod instructions;
mod registers;

/**************************************** Constant Values ***************************************************************/

/**************************************** Struct and Type definitions ***************************************************/

/// Virtualized representation of the CPU internally.
#[derive(Debug)]
pub struct CpuState {
    _acc: u16,              // Accumulator
    pc: u16,                // Program Counter
    _sp: u16,               // Stack Pointer
    _flags: u8,             // Flags
    _direct_page: u16,      // Direct page addressing offset (Lower 4 bytes of address)
    _data_bank: u8,         // Reference to current data bank addr (Upper 2 bytes of address)
    prog_bank: u8,          // Reference to current bank of instr (Upper 2 bytes of address)
    pub cycles_to_pend: u8, // Number of cycles to pend before running next operation.
}

impl CpuState {
    /// Return a new blank CpuState instance.
    pub const fn new() -> Self {
        Self {
            _acc: 0x0000,
            pc: 0x8000,
            _sp: 0x0000,
            _flags: 0x00,
            _direct_page: 0x0000,
            _data_bank: 0x00,
            prog_bank: 0x80,
            cycles_to_pend: 0x00,
        }
    }

    /// Fetch, Decode, Execute the next instruction, and return false if the VM needs to stop running.
    /// # Parameters
    ///     - `self`
    ///     - `memory`: Mutable pointer to current memory state.
    /// # Returns
    ///     - `true`:    If running,
    ///     - `false`:   If run should halt.
    pub fn step(&mut self, mem: &mut memory::Memory) -> bool {
        let next_instruction = instructions::fetch_and_decode(self, mem);
        instructions::execute(self, next_instruction, mem)
    }

    /// Print the current state of the CPU.
    pub fn _print_state(&self) {
        print!(
            "\nPC: {:#08X} ACC: {:#06X} SP: {:#06X}\nData Bank: {:#04X} Prog Bank: {:#04X} Direct Page: {:#06X}\nFlags nvmxdizc: {:#04X}\n    {:#010b}\n"
             ,self.pc, self._acc, self._sp, self._data_bank, self.prog_bank, self._direct_page, self._flags, self._flags
        );
    }

    /// Compose a fully-formed absolute address from the current PC and return it.
    pub fn get_pc(&self) -> usize { memory::compose_address(self.prog_bank, self.pc) }
}
/**************************************** File Scope Functions **********************************************************/

/**************************************** Tests *************************************************************************/
