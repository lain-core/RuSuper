use std::num::Wrapping;

/**************************************** Constant Values ***************************************************************/

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

pub(super) const STATUS_CARRY_BIT: usize = 0;
pub(super) const STATUS_ZERO_BIT: usize = 1;
pub(super) const STATUS_IRQ_BIT: usize = 2;
pub(super) const STATUS_DECIMAL_BIT: usize = 3;
pub(super) const STATUS_IREG_SIZE_BIT: usize = 4;
pub(super) const STATUS_AREG_SIZE_BIT: usize = 5;
pub(super) const STATUS_OVERFLOW_BIT: usize = 6;
pub(super) const STATUS_NEGATIVE_BIT: usize = 7;

pub(super) const ALU_8BIT_CARRY_BIT: usize = 8;
pub(super) const ALU_16BIT_CARRY_BIT: usize = 16;

pub(super) const REGISTER_MODE_16_BIT: usize = 0;
pub(super) const REGISTER_MODE_8_BIT: usize = 1;

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
    pub(super) index_x: Wrapping<u16>,
    pub(super) index_y: Wrapping<u16>,
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
            index_x: Wrapping(0),
            index_y: Wrapping(0),
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
    pub fn step_pc(&mut self, count: u16) { self.pc += count }
}

/// Status Register.
///
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
    pub(super) flags: [bool; 8],
    pub(super) value: Wrapping<u8>,
}

impl StatusRegister {
    pub const fn new() -> Self {
        StatusRegister {
            flags: [false; 8],
            value: Wrapping(0),
        }
    }

    /// Update the flag value based on the flags set.
    pub fn update_flag_value(&mut self) {
        for (bit, flag) in self.flags.iter().enumerate() {
            let bitvalue = if *flag { 1 } else { 0 };
            self.value = Wrapping(self.value.0 & (bitvalue << bit) as u8);
        }
    }
}
