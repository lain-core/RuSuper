/**************************************** Constant Values ***************************************************************/
/**************************************** Struct and Type definitions ***************************************************/
/**************************************** File Scope Functions **********************************************************/
/**************************************** Public Functions **************************************************************/

use std::num::Wrapping;

use super::{
    memory::{self, Memory},
    registers::{StatusFlags, REGISTER_MODE_16_BIT, REGISTER_MODE_8_BIT},
    CpuInstructionFnArguments, CpuState,
};

/// ADC Immediate
/// Syntax: ADC #const
/// Opcode: 0x69
/// Bytes:  2 if 8-bit param, 3 if 16-bit
/// Flags affected: nv----zc
pub(super) fn immediate(arg: &mut CpuInstructionFnArguments) -> Option<u8> {
    // If the carry flag is already set then carry it forward
    if arg.cpu.registers.get_flag(StatusFlags::Carry) {
        arg.param += 1;
    }

    // If the operation is in 8-bit mode, then perform all of the math in a u8 context.
    match arg.cpu.registers.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => {
            let acc_value: u8 = (arg.cpu.registers.acc.0 & 0x00FF) as u8;
            let param_value: u8 = (arg.param & 0x00FF) as u8;

            // Check if an unsigned overflow occurred. If it did, then set the carry bit.
            match acc_value.checked_add(param_value) {
                Some(_value) => {
                    arg.cpu.registers.clear_flag(StatusFlags::Carry);
                }
                None => {
                    arg.cpu.registers.set_flag(StatusFlags::Carry);
                }
            }

            // Check if a signed overflow occurred. If it did, then set the overflow bit.
            // http://www.6502.org/tutorials/vflag.html
            match (acc_value as i8).checked_add(param_value as i8) {
                Some(_value) => {
                    arg.cpu.registers.clear_flag(StatusFlags::Overflow);
                }
                None => {
                    arg.cpu.registers.set_flag(StatusFlags::Overflow);
                }
            }

            match acc_value.wrapping_add(param_value) as i8 >= 0 {
                true => {
                    arg.cpu.registers.clear_flag(StatusFlags::Negative);
                }
                false => {
                    arg.cpu.registers.set_flag(StatusFlags::Negative);
                }
            }

            arg.cpu.registers.acc = Wrapping(acc_value.wrapping_add(param_value) as u16);
        }
        REGISTER_MODE_16_BIT => {
            // Check if an unsigned overflow occurred and set the carry bit if needed
            match arg.cpu.registers.acc.0.checked_add(arg.param) {
                Some(_value) => {
                    arg.cpu.registers.clear_flag(StatusFlags::Carry);
                }
                None => {
                    arg.cpu.registers.set_flag(StatusFlags::Carry);
                }
            }

            // Check if a signed overflow occurred and set the carry bit if needed.
            match (arg.cpu.registers.acc.0 as i16).checked_add(arg.param as i16) {
                Some(_value) => {
                    arg.cpu.registers.clear_flag(StatusFlags::Overflow);
                }
                None => {
                    arg.cpu.registers.set_flag(StatusFlags::Overflow);
                }
            }

            match arg.cpu.registers.acc.0.wrapping_add(arg.param) as i16 >= 0 {
                true => {
                    arg.cpu.registers.clear_flag(StatusFlags::Negative);
                }
                false => {
                    arg.cpu.registers.set_flag(StatusFlags::Negative);
                }
            }

            arg.cpu.registers.acc += Wrapping(arg.param);
        }
    }

    // Update the flags that will be the same.

    match arg.cpu.registers.acc.0 {
        0 => arg.cpu.registers.set_flag(StatusFlags::Zero),
        _ => arg.cpu.registers.clear_flag(StatusFlags::Zero),
    }

    // Return the number of cycles to pend.
    match arg.cpu.registers.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => Some(2),
        REGISTER_MODE_16_BIT => Some(3),
    }
}

/// ADC absolute
/// Opcode: 0x6D for short, 0x6F for long
/// Bytes: 3 for short, 4 for long
/// Flags Affected: nv----zc
pub(super) fn absolute(arg: &mut CpuInstructionFnArguments) -> Option<u8> {
    match arg.cpu.registers.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => {}
        REGISTER_MODE_16_BIT => {}
    }

    None
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
        // even if we are in 8-bit mode. Currently the carry bit gets set and the ACC stores only
        // the lower byte.
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
            test_cpu.registers.set_flag(StatusFlags::AccSize);
            test_cpu.registers.acc = Wrapping(case[0]);

            let mut test_arg: CpuInstructionFnArguments = CpuInstructionFnArguments {
                cpu: &mut test_cpu,
                memory: &mut test_memory,
                bank: None,
                param: case[1],
            };

            println!("Test case: {:?}", case);

            // Perform the operation.
            adc::immediate(&mut test_arg);

            // Check the outcome.
            print!("Testing output value. ");
            assert_eq!(test_cpu.registers.acc, Wrapping(case[2]));
            // Check the flags.
            print!("Testing Flags: n, ");
            assert_eq!(
                case[3],
                test_cpu.registers.get_flag(StatusFlags::Negative) as u16
            );
            print!("v, ");
            assert_eq!(
                case[4],
                test_cpu.registers.get_flag(StatusFlags::Overflow) as u16
            );
            print!("z, ");
            assert_eq!(
                case[5],
                test_cpu.registers.get_flag(StatusFlags::Zero) as u16
            );
            println!("c.");
            assert_eq!(
                case[6],
                test_cpu.registers.get_flag(StatusFlags::Carry) as u16
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
            test_cpu.registers.clear_flag(StatusFlags::AccSize);
            test_cpu.registers.acc = Wrapping(case[0]);

            let mut test_arg: CpuInstructionFnArguments = CpuInstructionFnArguments {
                cpu: &mut test_cpu,
                memory: &mut test_memory,
                bank: None,
                param: case[1],
            };

            println!("Test case: {:?}", case);

            // Perform the operation.
            adc::immediate(&mut test_arg);

            // Check the outcome.
            print!("Testing output value. ");
            assert_eq!(test_cpu.registers.acc, Wrapping(case[2]));
            // Check the flags.
            print!("Testing Flags: n, ");
            assert_eq!(
                case[3],
                test_cpu.registers.get_flag(StatusFlags::Negative) as u16
            );
            print!("v, ");
            assert_eq!(
                case[4],
                test_cpu.registers.get_flag(StatusFlags::Overflow) as u16
            );
            print!("z, ");
            assert_eq!(
                case[5],
                test_cpu.registers.get_flag(StatusFlags::Zero) as u16
            );
            println!("c.");
            assert_eq!(
                case[6],
                test_cpu.registers.get_flag(StatusFlags::Carry) as u16
            );
        }
    }
}

/**************************************** Test Helpers **************************************************************/
/**************************************** Unit Test Implementations *************************************************/
