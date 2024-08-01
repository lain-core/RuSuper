use super::{memory::Memory, registers, CpuState};
use std::num::Wrapping;

/// LDA absolute
/// Syntax: LDA #const
/// Opcode: 0xA9
/// Bytes: 2 for 8-bit, 3 for 16-bit
/// Flags affected: n-----z-
pub(super) fn absolute(cpu: &mut CpuState, mem: &mut Memory, param: u16) -> Option<u8> {
    match cpu
        .registers
        .status
        .get_flag(registers::StatusFlags::AccSize)
    {
        registers::REGISTER_MODE_8_BIT => {
            let masked_param = param & 0x00FF;
            cpu.registers.acc = Wrapping(masked_param as u16);
        }
        registers::REGISTER_MODE_16_BIT => {
            cpu.registers.acc = Wrapping(param);
        }
    }

    // Update the flags.
    if cpu.registers.acc.0 == 0 {
        cpu.registers.status.set_flag(registers::StatusFlags::Zero);
    }

    None
}
