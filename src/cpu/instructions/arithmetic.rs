/**************************************** Constant Values ***************************************************************/
/**************************************** Struct and Type definitions ***************************************************/
/**************************************** File Scope Functions **********************************************************/
/**************************************** Public Functions **************************************************************/

use std::num::Wrapping;

use super::{
    memory,
    registers::{self, ALU_8BIT_CARRY_BIT},
    CpuState,
};

/// Name:   ADC Immediate
/// Syntax: ADC #const
/// Opcode: 0x6D
/// Bytes:  2 if 8-bit param, 3 if 16-bit
/// Extra: +1 if
pub(super) fn adc_immediate(cpu: &mut CpuState, _memory: &mut memory::Memory, param: u16) -> bool {
    // If the operation is in 8-bit mode, then perform all of the math in a u8 context.
    if cpu.registers.status.flags[registers::STATUS_AREG_SIZE_BIT] as usize
        == registers::REGISTER_MODE_8_BIT
    {
        let acc_value: u8 = (cpu.registers.acc.0 & 0x00FF) as u8;
        let param_value: u8 = (param & 0x00FF) as u8;
        match acc_value.checked_add(param_value) {
            Some(value) => {}
            None => {
                cpu.registers.status.flags[registers::STATUS_CARRY_BIT] =
                    (((acc_value + param_value) as u16 & (1 << ALU_8BIT_CARRY_BIT))
                        >> ALU_8BIT_CARRY_BIT)
                        != 0;
                cpu.registers.acc = Wrapping(acc_value.wrapping_add(param_value) as u16);
            }
        }
    }
    // If the operation is in 16-bit mode, perform the math that way.

    // Check if the addition will overflow. If not, proceed as usual.
    match cpu.registers.acc.0.checked_add(param) {
        Some(_value) => {
            cpu.registers.acc += param;
        }
        // Otherwise
        None => {
            cpu.registers.acc += param;
            cpu.registers.status.flags[registers::STATUS_CARRY_BIT] = true;
        }
    }

    // Update the flags that will be the same.
    cpu.registers.status.flags[registers::STATUS_ZERO_BIT] = cpu.registers.acc.0 == 0;
    cpu.registers.status.flags[registers::STATUS_NEGATIVE_BIT] =
        (cpu.registers.acc.0 >> registers::STATUS_NEGATIVE_BIT as u16) != 0;
    true
}

/**************************************** Tests *************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adc_immediate() {
        panic!("Unimplemented");
    }
}

/**************************************** Test Helpers **************************************************************/
/**************************************** Unit Test Implementations *************************************************/
