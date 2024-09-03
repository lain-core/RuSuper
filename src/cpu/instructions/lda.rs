use super::{
    memory,
    registers::{StatusFlags, REGISTER_MODE_16_BIT, REGISTER_MODE_8_BIT},
    CpuInstructionFnArguments,
};
use crate::cpu::CpuRegisters;
use std::num::Wrapping;

/**************************************** File Scope Functions **********************************************************/

/// Update the flags based on the value loaded into the accumulator.
///
/// All of the LDA instructions affect the same flags:
/// n-----z-
///
/// Parameters:
///     - `registers`: A mutable pointer to the register state to set the flags in and read ACC
///     from.
fn update_flags(registers: &mut CpuRegisters) {
    let test_value: i16 = match registers.get_flag(StatusFlags::AccSize) {
        // Cast it to i8 to get value, and then cast as i16 to maintain signed-ness, but return the
        // same type.
        REGISTER_MODE_8_BIT => (registers.acc.0 as i8) as i16,
        REGISTER_MODE_16_BIT => registers.acc.0 as i16,
    };

    if test_value < 0 {
        registers.clear_flag(StatusFlags::Zero);
        registers.set_flag(StatusFlags::Negative);
    }
    else if test_value == 0 {
        registers.set_flag(StatusFlags::Zero);
        registers.clear_flag(StatusFlags::Negative);
    }
    else {
        registers.clear_flag(StatusFlags::Zero);
        registers.clear_flag(StatusFlags::Negative);
    }
}

/**************************************** Public Functions **************************************************************/

/// LDA immediate
/// Syntax: LDA #const
/// Opcode: 0xA9
/// Bytes: 2 for 8-bit, 3 for 16-bit
/// Flags affected: n-----z-
///
/// Parameters:
///     - `arg`: Mutable pointer to state of VM and the arguments for this instruction.
///
/// Returns:
///     - `Some(2)`: Number of cycles to pend for an 8-bit LDA.
///     - `Some(3)`: Number of cycles to pend for a 16-bit LDA.
pub(super) fn immediate(arg: &mut CpuInstructionFnArguments) -> Option<u8> {
    match arg.cpu.registers.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => {
            let masked_param = arg.param & 0x00FF;
            arg.cpu.registers.acc = Wrapping(masked_param);
        }
        REGISTER_MODE_16_BIT => {
            arg.cpu.registers.acc = Wrapping(arg.param);
        }
    }

    // Update the flags.
    update_flags(&mut arg.cpu.registers);

    match arg.cpu.registers.get_flag(StatusFlags::AccSize) {
        REGISTER_MODE_8_BIT => Some(2),
        REGISTER_MODE_16_BIT => Some(3),
    }
}

/// LDA absolute
/// Syntax: LDA addr
/// Opcode: 0xAD for bank
/// Bytes: 3
/// Flags affected: n-----z-
///
/// Load a value from an absolute location in memory, using the prog bank as reference and the
/// argument as the address within the bank.
///
/// Parameters:
///     - `arg`: Mutable pointer to state of VM and the arguments for this instruction.
///
/// Returns:
///     - `Some(4)`: Number of cycles to pend.
pub(super) fn absolute(arg: &mut CpuInstructionFnArguments) -> Option<u8> {
    let value_at_address = arg
        .memory
        .get_word(memory::compose_address(
            arg.cpu.registers.program_bank.0,
            arg.param,
        ))
        .expect("Could not read value from memory");

    arg.cpu.registers.acc = Wrapping(value_at_address);

    update_flags(&mut arg.cpu.registers);

    None
}

/// LDA absolute long
/// Syntax: LDA addr
/// Opcode: 0xAF for long
/// Bytes: 4
/// Flags affected: n-----z-
///
/// Load a value from an absolute location in memory.
///
/// Parameters:
///     - `arg`: Mutable pointer to state of VM and the arguments for this instruction.
///
/// Returns:
///     - `Some(5)`: Number of cycles to pend.
pub(super) fn absolute_long(arg: &mut CpuInstructionFnArguments) -> Option<u8> {
    let value_at_address = arg
        .memory
        .get_word(memory::compose_address(
            arg.bank
                .expect("Bank was not included with argument passed to instruction"),
            arg.param,
        ))
        .expect("Unable to read value from memory");

    arg.cpu.registers.acc = Wrapping(value_at_address);

    update_flags(&mut arg.cpu.registers);

    Some(5)
}

/// LDA direct page
/// Syntax: LDA dp
/// Opcode: 0xA5
/// Bytes: 2
/// Flags affected: n-----z-
///
/// Load a value ???
///
/// Parameters:
///     - `arg`: Mutable pointer to state of VM and the arguments for this instruction.
///
/// Returns:
///     - `Some(3)`: Number of cycles to pend.
pub(super) fn direct_page(arg: &mut CpuInstructionFnArguments) -> Option<u8> { Some(3) }

/**************************************** Tests *************************************************************************/

#[cfg(test)]
mod tests {
    use super::super::memory::Memory;
    use super::super::CpuState;
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
