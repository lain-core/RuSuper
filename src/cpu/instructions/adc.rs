/**************************************** Constant Values ***************************************************************/
/**************************************** Struct and Type definitions ***************************************************/
/**************************************** File Scope Functions **********************************************************/
/**************************************** Public Functions **************************************************************/

use std::num::Wrapping;

use super::{
    memory,
    registers::{
        self, ALU_16BIT_CARRY_BIT, ALU_8BIT_CARRY_BIT, REGISTER_MODE_16_BIT, REGISTER_MODE_8_BIT,
    },
    CpuState,
};

/// ADC Immediate
/// Syntax: ADC #const
/// Opcode: 0x69
/// Bytes:  2 if 8-bit param, 3 if 16-bit
/// Flags affected: nv----zc
pub(super) fn immediate(
    cpu: &mut CpuState, _memory: &mut memory::Memory, mut param: u16,
) -> Option<u8> {
    // If the carry flag is already set then carry it forward
    if cpu.registers.status.flags[registers::STATUS_CARRY_BIT] == true {
        param += 1;
    }

    // If the operation is in 8-bit mode, then perform all of the math in a u8 context.
    match cpu.registers.status.flags[registers::STATUS_AREG_SIZE_BIT] {
        registers::REGISTER_MODE_8_BIT => {
            let acc_value: u8 = (cpu.registers.acc.0 & 0x00FF) as u8;
            let param_value: u8 = (param & 0x00FF) as u8;

            // Check if an unsigned overflow occurred. If it did, then set the carry bit.
            match acc_value.checked_add(param_value) {
                Some(_value) => {
                    cpu.registers.status.flags[registers::STATUS_CARRY_BIT] = false;
                }
                None => {
                    cpu.registers.status.flags[registers::STATUS_CARRY_BIT] = true;
                }
            }

            // Check if a signed overflow occurred. If it did, then set the overflow bit.
            // http://www.6502.org/tutorials/vflag.html
            match (acc_value as i8).checked_add(param_value as i8) {
                Some(_value) => {
                    cpu.registers.status.flags[registers::STATUS_OVERFLOW_BIT] = false;
                }
                None => {
                    cpu.registers.status.flags[registers::STATUS_OVERFLOW_BIT] = true;
                }
            }

            cpu.registers.acc = Wrapping(acc_value.wrapping_add(param_value) as u16);
        }
        registers::REGISTER_MODE_16_BIT => {
            // Check if an unsigned overflow occurred and set the carry bit if needed
            match cpu.registers.acc.0.checked_add(param) {
                Some(_value) => {
                    cpu.registers.status.flags[registers::STATUS_CARRY_BIT] = false;
                }
                None => {
                    cpu.registers.status.flags[registers::STATUS_CARRY_BIT] = true;
                }
            }

            // Check if a signed overflow occurred and set the carry bit if needed.
            match (cpu.registers.acc.0 as i16).checked_add(param as i16) {
                Some(_value) => {
                    cpu.registers.status.flags[registers::STATUS_OVERFLOW_BIT] = false;
                }
                None => {
                    cpu.registers.status.flags[registers::STATUS_OVERFLOW_BIT] = true;
                }
            }

            cpu.registers.acc += Wrapping(param);
        }
    }

    // Update the flags that will be the same.
    cpu.registers.status.flags[registers::STATUS_ZERO_BIT] = cpu.registers.acc.0 == 0;
    cpu.registers.status.flags[registers::STATUS_NEGATIVE_BIT] =
        (cpu.registers.acc.0 >> registers::STATUS_NEGATIVE_BIT as u16) != 0;

    // Return the number of cycles to pend.
    match cpu.registers.status.flags[registers::STATUS_AREG_SIZE_BIT] {
        REGISTER_MODE_8_BIT => Some(2),
        REGISTER_MODE_16_BIT => Some(3),
    }
}

/**************************************** Tests *************************************************************************/

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::adc;
    use memory::Memory;

    use super::*;

    #[test]
    fn test_immediate_8bit() {
        // TODO: Check whether e.g. 0xFF + 0x01 would result in 0x0100 being stored into the ACC
        // even if we are in 8-bit mode.
        let test_cases = vec![
            //ACC +  B =  C,         n, v, z, c
            [0x0001, 0x0001, 0x0002, 0, 0, 0, 0],
            [0x007F, 0x0082, 0x0001, 0, 0, 0, 1],
            [0x0000, 0x0000, 0x0000, 0, 0, 1, 0],
            [0x0001, 0x00FF, 0x0000, 0, 0, 1, 1],
            [0x007F, 0x0001, 0x0080, 1, 1, 0, 0],
            [0x0080, 0x0001, 0x0081, 1, 0, 0, 0],
        ];

        for case in test_cases {
            let mut test_cpu: CpuState = CpuState::new();
            let mut test_memory: Memory = Memory::new();
            test_cpu.registers.status.flags[registers::STATUS_AREG_SIZE_BIT] =
                registers::REGISTER_MODE_8_BIT;
            test_cpu.registers.acc = Wrapping(case[0]);

            println!("Test case: {:?}", case);

            // Perform the operation.
            adc::immediate(&mut test_cpu, &mut test_memory, case[1]);

            // Check the outcome.
            print!("Testing output value. ");
            assert_eq!(test_cpu.registers.acc, Wrapping(case[2]));
            // Check the flags.
            print!("Testing Flags: n, ");
            assert_eq!(
                case[3],
                test_cpu.registers.status.flags[registers::STATUS_NEGATIVE_BIT] as u16
            );
            print!("v, ");
            assert_eq!(
                case[4],
                test_cpu.registers.status.flags[registers::STATUS_OVERFLOW_BIT] as u16
            );
            print!("z, ");
            assert_eq!(
                case[5],
                test_cpu.registers.status.flags[registers::STATUS_ZERO_BIT] as u16
            );
            println!("c.");
            assert_eq!(
                case[6],
                test_cpu.registers.status.flags[registers::STATUS_CARRY_BIT] as u16
            );
        }
    }

    #[test]
    fn test_immediate_16bit() {
        let test_cases = vec![
            //ACC +  B =  C,         n, v, z, c
            [0x0001, 0x0001, 0x0002, 0, 0, 0, 0],
            [0xFFFF, 0x0002, 0x0001, 0, 0, 0, 1],
            [0xFFFF, 0x0001, 0x0000, 0, 0, 1, 1],
            [0x7FFF, 0x0001, 0x8000, 1, 1, 0, 0],
            [0x8000, 0x0001, 0x8001, 1, 0, 0, 0],
        ];

        for case in test_cases {
            let mut test_cpu: CpuState = CpuState::new();
            let mut test_memory: Memory = Memory::new();
            test_cpu.registers.status.flags[registers::STATUS_AREG_SIZE_BIT] =
                registers::REGISTER_MODE_16_BIT;
            test_cpu.registers.acc = Wrapping(case[0]);

            println!("Test case: {:?}", case);

            // Perform the operation.
            adc::immediate(&mut test_cpu, &mut test_memory, case[1]);

            // Check the outcome.
            print!("Testing output value. ");
            assert_eq!(test_cpu.registers.acc, Wrapping(case[2]));
            // Check the flags.
            print!("Testing Flags: n, ");
            assert_eq!(
                case[3],
                test_cpu.registers.status.flags[registers::STATUS_NEGATIVE_BIT] as u16
            );
            print!("v, ");
            assert_eq!(
                case[4],
                test_cpu.registers.status.flags[registers::STATUS_OVERFLOW_BIT] as u16
            );
            print!("z, ");
            assert_eq!(
                case[5],
                test_cpu.registers.status.flags[registers::STATUS_ZERO_BIT] as u16
            );
            println!("c.");
            assert_eq!(
                case[6],
                test_cpu.registers.status.flags[registers::STATUS_CARRY_BIT] as u16
            );
        }
    }
}

/**************************************** Test Helpers **************************************************************/
/**************************************** Unit Test Implementations *************************************************/
