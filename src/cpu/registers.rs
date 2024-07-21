/**************************************** Constant Values ***************************************************************/

use std::num::Wrapping;

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
    acc: Wrapping<u16>,
    index_x: Wrapping<u16>,
    index_y: Wrapping<u16>,
    stack_ptr: Wrapping<u16>,
    data_bank: Wrapping<u16>,
    direct_page: Wrapping<u16>,
    program_bank: Wrapping<u16>,
    processor_status: StatusRegister,
    pc: Wrapping<u16>,
}

/// Status Register.
/// CZIDXMVN
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
    value: u8,
}
