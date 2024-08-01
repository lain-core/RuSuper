use super::{
    memory::Memory,
    registers::{StatusFlags, REGISTER_MODE_16_BIT, REGISTER_MODE_8_BIT},
    CpuInstructionFnArguments, CpuState,
};
use std::num::Wrapping;

/**************************************** File Scope Functions **********************************************************/

/**************************************** Public Functions **************************************************************/

/// LDA absolute
/// Syntax: LDA #const
/// Opcode: 0xA9
/// Bytes: 2 for 8-bit, 3 for 16-bit
/// Flags affected: n-----z-
pub(super) fn immediate(arg: &mut CpuInstructionFnArguments) -> Option<u8> {
    match arg.cpu.registers.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => {
            let masked_param = arg.param & 0x00FF;
            arg.cpu.registers.acc = Wrapping(masked_param as u16);

            match arg.cpu.registers.acc.0 as i8 >= 0 {
                true => {
                    arg.cpu.registers.clear_flag(StatusFlags::Negative);
                }
                false => {
                    arg.cpu.registers.set_flag(StatusFlags::Negative);
                }
            }
        }
        REGISTER_MODE_16_BIT => {
            arg.cpu.registers.acc = Wrapping(arg.param);

            match arg.cpu.registers.acc.0 as i16 >= 0 {
                true => {
                    arg.cpu.registers.clear_flag(StatusFlags::Negative);
                }
                false => {
                    arg.cpu.registers.set_flag(StatusFlags::Negative);
                }
            }
        }
    }

    // Update the flags.
    match arg.cpu.registers.acc.0 == 0 {
        true => {
            arg.cpu.registers.set_flag(StatusFlags::Zero);
        }
        false => {
            arg.cpu.registers.clear_flag(StatusFlags::Zero);
        }
    }

    match arg.cpu.registers.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => Some(2),
        REGISTER_MODE_16_BIT => Some(3),
    }
}

/**************************************** Tests *************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instructions::lda;

    #[test]
    fn test_immediate() {
        let test_cases = vec![
            //acc, n, z
            [0x00, 0, 1],
            [0x01, 0, 0],
            [0x7F, 0, 0],
            [0x80, 1, 0],
            [0xFF, 1, 0],
        ];

        let mut test_cpu = CpuState::new();
        let mut test_mem = Memory::new();
        test_cpu.registers.set_flag(StatusFlags::AccSize);

        let mut test_args: CpuInstructionFnArguments = CpuInstructionFnArguments {
            cpu: &mut test_cpu,
            memory: &mut test_mem,
            bank: None,
            param: 0,
        };

        for case in test_cases {
            test_args.param = case[0];
            lda::immediate(&mut test_args);

            println!("Test Case: {:?}", case);
            print!("Testing Result");
            assert_eq!(case[0], test_args.cpu.registers.acc.0);

            print!(" Testing Flags: ");
            print!("n, ");
            assert_eq!(
                case[1],
                test_args.cpu.registers.get_flag(StatusFlags::Negative) as u16
            );
            println!("z");
            assert_eq!(
                case[2],
                test_args.cpu.registers.get_flag(StatusFlags::Zero) as u16
            );
        }
    }
}
