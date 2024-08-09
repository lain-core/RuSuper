use std::num::Wrapping;

/**************************************** Constant Values ***************************************************************/

pub(super) const _ALU_8BIT_NEGATIVE_BIT: usize = 7;
pub(super) const _ALU_16BIT_NEGATIVE_BIT: usize = 15;
pub(super) const _ALU_8BIT_CARRY_BIT: usize = 8;
pub(super) const _ALU_16BIT_CARRY_BIT: usize = 16;

pub(super) const REGISTER_MODE_16_BIT: bool = false;
pub(super) const REGISTER_MODE_8_BIT: bool = true;

/**************************************** Struct and Type definitions ***************************************************/
/// CPU Register fields.
///     acc:                Accumulator
///     index_x, index_y:   Index registers, used to reference memory, pass data to memory, or counters for loops.
///     stack_ptr:          Stack Pointer
///     data_bank:          Holds the default bank for memory transfers.
///     direct_page:        Used for direct page addressing modes. Holds memory bank address of the data the CPU is accessing.
///     program_bank:       Holds bank address of all instruction fetches
///     processor_status:   Holds flags & test results. broken out in StatusRegister explanation.
///     pc:                 Program Counter
#[derive(Debug, Clone, Copy)]
pub(super) struct CpuRegisters {
    pub(super) acc: Wrapping<u16>,
    pub(super) _index_x: Wrapping<u16>,
    pub(super) _index_y: Wrapping<u16>,
    pub(super) stack_ptr: Wrapping<u16>,
    pub(super) data_bank: Wrapping<u8>,
    pub(super) direct_page: Wrapping<u16>,
    pub(super) program_bank: Wrapping<u8>,
    pub(super) status: StatusRegister,
    pub(super) pc: Wrapping<u16>,
}

impl CpuRegisters {
    pub const fn new() -> Self {
        CpuRegisters {
            acc: Wrapping(0),
            _index_x: Wrapping(0),
            _index_y: Wrapping(0),
            stack_ptr: Wrapping(0),
            data_bank: Wrapping(0),
            direct_page: Wrapping(0),
            program_bank: Wrapping(0x80),
            status: StatusRegister::new(),
            pc: Wrapping(0x8000),
        }
    }

    /// Print the current state of the CPU.
    pub fn print_state(&self) {
        println!(
            "\nPC: {:#08X} ACC: {:#06X} SP: {:#06X}\nData Bank: {:#04X} Prog Bank: {:#04X} Direct Page: {:#06X}"
             ,self.pc, self.acc, self.stack_ptr, self.data_bank, self.program_bank, self.direct_page,
        );
    }

    /// Step the PC by `count` steps.
    pub fn _step_pc(&mut self, count: u16) {
        self.pc += count
    }

    /// Set a target flag.
    /// Parameters:
    ///     - `self`
    ///     - `flag`: Target flag to set.
    pub fn set_flag(&mut self, flag: StatusFlags) {
        self.status.flags[flag as usize] = true;
        self.status.value |= Wrapping(1 << flag as u8);

        // If the user flips from 16-bit mode to 8-bit mode, clear the top byte in the ACC.
        if flag == StatusFlags::AccSize {
            self.acc = Wrapping(self.acc.0 & 0x00FF);
        }
    }

    /// Clear a target flag.
    /// Parameters:
    ///     - `self`
    ///     - `flag`: Target flag to clear.
    pub fn clear_flag(&mut self, flag: StatusFlags) {
        self.status.flags[flag as usize] = false;
        self.status.value &= Wrapping(!(1 << flag as u8));
    }

    /// Get an individual flag register value.
    /// Parameters:
    ///     - `self`
    ///     - `flag`: Flag value to check.
    /// Returns:
    ///     - `false`: if flag is currently un-set
    ///     - `true`: if flag is currently set
    pub fn get_flag(&self, flag: StatusFlags) -> bool {
        self.status.flags[flag as usize]
    }

    /// Get the stored register value of all of the flags.
    pub fn _get_flag_vals(&self) -> u8 {
        self.status.value.0
    }
}

/// Status Flags.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusFlags {
    Carry = 0,
    Zero = 1,
    _IRQDisable = 2,
    _Decimal = 3,
    _IndexSize = 4,
    AccSize = 5,
    Overflow = 6,
    Negative = 7,
}

///
/// Status Register.
/// Contains the flags for caluclated values, and the stored value of those set flags.
///
/// NVMXDIZC
/// 00000000
/// ^^^^^^^^
/// |||||||└> Carry
/// ||||||└─> Zero
/// |||||└──> IRQ Disable
/// ||||└───> Decimal
/// |||└────> Index Register Size (Native Mode Only). 0 = 16-bit. 1 = 8-bit. Break in emulation mode.
/// ||└─────> Accumulator Register Size (Native Mode Only). 0 = 16-bit. 1 = 8-bit.
/// |└──────> Overflow
/// └───────> Negative
#[derive(Debug, Clone, Copy)]
pub(super) struct StatusRegister {
    flags: [bool; 8],
    value: Wrapping<u8>,
}

impl StatusRegister {
    pub const fn new() -> Self {
        StatusRegister {
            flags: [false; 8],
            value: Wrapping(0),
        }
    }
}
