use std::num::Wrapping;

use super::{
    memory::{self, Memory},
    registers::{StatusFlags, REGISTER_MODE_16_BIT, REGISTER_MODE_8_BIT},
    CpuInstructionFnArguments, CpuState,
};

/**************************************** Constant Values ***************************************************************/
/**************************************** Struct and Type definitions ***************************************************/
/**************************************** File Scope Functions **********************************************************/

/// Perform an addition with carry and set the flags accordingly.
/// Parameters:
///     - `cpu`: State of the CPU to modify.
///     - `param`: The value to add to the ACC value.
fn perform_add(cpu: &mut CpuState, mut param: u16) {
    // If the carry flag is already set then carry it forward
    if cpu.registers.get_flag(StatusFlags::Carry) {
        param += 1;
    }

    // If the operation is in 8-bit mode, then perform all of the math in a u8 context.
    match cpu.registers.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => {
            let acc_value: u8 = (cpu.registers.acc.0 & 0x00FF) as u8;
            let param_value: u8 = (param & 0x00FF) as u8;

            // Check if an unsigned overflow occurred. If it did, then set the carry bit.
            match acc_value.checked_add(param_value) {
                Some(_value) => {
                    cpu.registers.clear_flag(StatusFlags::Carry);
                }
                None => {
                    cpu.registers.set_flag(StatusFlags::Carry);
                }
            }

            // Check if a signed overflow occurred. If it did, then set the overflow bit.
            // http://www.6502.org/tutorials/vflag.html
            match (acc_value as i8).checked_add(param_value as i8) {
                Some(_value) => {
                    cpu.registers.clear_flag(StatusFlags::Overflow);
                }
                None => {
                    cpu.registers.set_flag(StatusFlags::Overflow);
                }
            }

            match acc_value.wrapping_add(param_value) as i8 >= 0 {
                true => {
                    cpu.registers.clear_flag(StatusFlags::Negative);
                }
                false => {
                    cpu.registers.set_flag(StatusFlags::Negative);
                }
            }

            cpu.registers.acc = Wrapping(acc_value.wrapping_add(param_value) as u16);
        }
        REGISTER_MODE_16_BIT => {
            // Check if an unsigned overflow occurred and set the carry bit if needed
            match cpu.registers.acc.0.checked_add(param) {
                Some(_value) => {
                    cpu.registers.clear_flag(StatusFlags::Carry);
                }
                None => {
                    cpu.registers.set_flag(StatusFlags::Carry);
                }
            }

            // Check if a signed overflow occurred and set the carry bit if needed.
            match (cpu.registers.acc.0 as i16).checked_add(param as i16) {
                Some(_value) => {
                    cpu.registers.clear_flag(StatusFlags::Overflow);
                }
                None => {
                    cpu.registers.set_flag(StatusFlags::Overflow);
                }
            }

            match cpu.registers.acc.0.wrapping_add(param) as i16 >= 0 {
                true => {
                    cpu.registers.clear_flag(StatusFlags::Negative);
                }
                false => {
                    cpu.registers.set_flag(StatusFlags::Negative);
                }
            }

            cpu.registers.acc += Wrapping(param);
        }
    }

    // Update the flags that will be the same.

    match cpu.registers.acc.0 {
        0 => cpu.registers.set_flag(StatusFlags::Zero),
        _ => cpu.registers.clear_flag(StatusFlags::Zero),
    }
}

/**************************************** Public Functions **************************************************************/

/// ADC Immediate
/// Syntax: ADC #const
/// Opcode: 0x69
/// Bytes:  2 if 8-bit param, 3 if 16-bit
/// Flags affected: nv----zc
pub(super) fn immediate(arg: &mut CpuInstructionFnArguments) -> Option<u8> {
    // Add the value.
    perform_add(arg.cpu, arg.param);

    let mut cycles_to_pend: u8 = 0;

    // If the parameter is 0, then this takes one additional cycle.
    if arg.param == 0 {
        cycles_to_pend += 1;
    }
    // Return the number of cycles to pend.
    match arg.cpu.registers.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => cycles_to_pend += 2,
        REGISTER_MODE_16_BIT => cycles_to_pend += 3,
    }

    Some(cycles_to_pend)
}

/// ADC absolute
/// Opcode: 0x6D for short, 0x6F for long
/// Bytes: 3 for short, 4 for long
/// Flags Affected: nv----zc
pub(super) fn absolute(arg: &mut CpuInstructionFnArguments) -> Option<u8> {
    let value: u16;
    match arg.bank {
        Some(bank) => {
            let memaddr = memory::compose_address(bank, arg.param);
            value = arg
                .memory
                .get_word(memaddr)
                .expect("Parameter was out of bounds in memory.");
        }
        None => {
            let memaddr = memory::compose_address(arg.cpu.registers.data_bank.0, arg.param);
            value = arg
                .memory
                .get_word(memaddr)
                .expect("Parameter was out of bounds in memory.");
        }
    }

    perform_add(arg.cpu, value);

    let mut cycles_to_pend = 0;

    if value == 0 {
        cycles_to_pend += 1;
    }

    match arg.bank {
        Some(_) => {
            cycles_to_pend += 5;
        }
        None => {
            cycles_to_pend += 4;
        }
    }

    Some(cycles_to_pend)
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
