use registers::CpuRegisters;

use crate::memory;

pub(crate) mod instructions;
mod registers;

/**************************************** Constant Values ***************************************************************/

/**************************************** Struct and Type definitions ***************************************************/

/// Virtualized representation of the CPU internally.
#[derive(Debug)]
pub struct CpuState {
    pub(self) registers: CpuRegisters,
    pub cycles_to_pend: u8, // Number of cycles to pend before running next operation.
}

impl CpuState {
    /// Return a new blank CpuState instance.
    pub const fn new() -> Self {
        Self {
            registers: CpuRegisters::new(),
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

    /// Compose a fully-formed absolute address from the current PC and return it.
    pub fn get_pc(&self) -> usize {
        memory::compose_address(self.registers.program_bank.0, self.registers.pc.0)
    }

    // Print the state of the CPU.
    pub fn print_state(&self) {
        self.registers.print_state();
    }
}
/**************************************** File Scope Functions **********************************************************/

/**************************************** Tests *************************************************************************/
