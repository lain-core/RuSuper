use super::{
    memory::Memory,
    registers::{StatusFlags, REGISTER_MODE_16_BIT, REGISTER_MODE_8_BIT},
    CpuState,
};
use std::num::Wrapping;

/// LDA absolute
/// Syntax: LDA #const
/// Opcode: 0xA9
/// Bytes: 2 for 8-bit, 3 for 16-bit
/// Flags affected: n-----z-
pub(super) fn immediate(cpu: &mut CpuState, _mem: &mut Memory, param: u16) -> Option<u8> {
    match cpu.registers.status.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => {
            let masked_param = param & 0x00FF;
            cpu.registers.acc = Wrapping(masked_param as u16);

            match cpu.registers.acc.0 as i8 >= 0 {
                true => {
                    cpu.registers.status.clear_flag(StatusFlags::Negative);
                }
                false => {
                    cpu.registers.status.set_flag(StatusFlags::Negative);
                }
            }
        }
        REGISTER_MODE_16_BIT => {
            cpu.registers.acc = Wrapping(param);

            match cpu.registers.acc.0 as i16 >= 0 {
                true => {
                    cpu.registers.status.clear_flag(StatusFlags::Negative);
                }
                false => {
                    cpu.registers.status.set_flag(StatusFlags::Negative);
                }
            }
        }
    }

    // Update the flags.
    match cpu.registers.acc.0 == 0 {
        true => {
            cpu.registers.status.set_flag(StatusFlags::Zero);
        }
        false => {
            cpu.registers.status.clear_flag(StatusFlags::Zero);
        }
    }

    match cpu.registers.status.get_flag(StatusFlags::AccSize) {
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
        test_cpu.registers.status.set_flag(StatusFlags::AccSize);

        for case in test_cases {
            lda::immediate(&mut test_cpu, &mut test_mem, case[0]);

            println!("Test Case: {:?}", case);
            print!("Testing Result");
            assert_eq!(case[0], test_cpu.registers.acc.0);

            print!(" Testing Flags: ");
            print!("n, ");
            assert_eq!(
                case[1],
                test_cpu.registers.status.get_flag(StatusFlags::Negative) as u16
            );
            println!("z");
            assert_eq!(
                case[2],
                test_cpu.registers.status.get_flag(StatusFlags::Zero) as u16
            );
        }
    }
}
