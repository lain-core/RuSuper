mod adc;
mod branch;
mod lda;
mod misc;

use registers::REGISTER_MODE_16_BIT;

use super::*;

/**************************************** Constant Values ***************************************************************/
/// Number of instructions
const NUM_INSTRUCTIONS: usize = 256;

/// Parameter for an offset is always instruction + 1.
const INST_PARAM_OFFSET: u16 = 1;

/**************************************** Struct and Type definitions ***************************************************/
/// Enumerated type to match to an opcode. Useful for debugging because it can be represented easily as a string.
#[derive(Debug, Clone, Copy)]
pub enum CpuOpcode {
    Adc,
    Lda,
    Stp,
    Nop,
    // Many More
}

/// Generalized function signature for CPU Instruction functions.
/// Takes modifiable CPU State, Memory, and 16-bit parameter (widened if u8).
pub(super) type CpuInstructionFn = fn(&mut CpuInstructionFnArguments) -> Option<u8>;
pub(super) struct CpuInstructionFnArguments<'a> {
    cpu: &'a mut CpuState,
    memory: &'a mut memory::Memory,
    bank: Option<u8>,
    param: u16,
}

/// The width of the parameter for this operation.
/// The underlying value is the number of bytes to increment the PC by.
#[repr(u16)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum CpuParamWidth {
    Variable = 0, // Paramater is variable width (depends on ALU setting)
    None = 1,
    Byte = 2, // Parameter is 8-bit (1 Byte)
    Word = 3, // Parameter is 16-bit (1 Word)
    Long = 4, // Parameter is 24-bit (long)
}

/// A conglomerate wrapper of the prior enums.
///     - `opcode`      Opcode of next operation to run.
///     - `width`       Width of next operation, to calculate parameters.
///     - `function`    Function pointer to handler for next operation.
#[derive(Debug, Clone, Copy)]
pub(super) struct CpuInstruction {
    opcode: CpuOpcode,
    width: CpuParamWidth,
    function: CpuInstructionFn,
}

/// Map of the cpu opcodes.
/// Would prefer this to be a hashmap, but rust cannot generate a HashMap::From() as const, and a global cannot be declared using `let`.
pub(super) const INSTRUCTION_MAP: [CpuInstruction; NUM_INSTRUCTIONS] = [
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x00 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x01 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x02 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x03 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x04 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x05 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x06 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x07 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x08 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x09 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x0A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x0B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x0C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x0D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x0E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x0F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x10 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x11 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x12 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x13 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x14 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x15 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x16 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x17 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x18 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x19 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x1A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x1B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x1C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x1D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x1E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x1F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x20 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x21 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x22 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x23 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x24 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x25 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x26 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x27 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x28 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x29 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x2A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x2B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x2C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x2D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x2E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x2F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x30 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x31 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x32 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x33 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x34 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x35 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x36 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x37 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x38 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x39 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x3A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x3B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x3C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x3D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x3E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x3F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x40 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x41 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x42 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x43 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x44 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x45 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x46 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x47 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x48 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x49 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x4A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x4B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x4C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x4D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x4E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x4F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x50 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x51 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x52 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x53 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x54 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x55 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x56 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x57 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x58 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x59 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x5A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x5B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x5C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x5D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x5E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x5F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x60 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x61 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x62 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x63 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x64 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x65 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x66 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x67 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x68 */
    CpuInstruction {
        opcode: CpuOpcode::Adc,
        width: CpuParamWidth::Variable,
        function: adc::immediate,
    }, /* 0x69 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x6A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x6B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x6C */
    CpuInstruction {
        opcode: CpuOpcode::Adc,
        width: CpuParamWidth::Word,
        function: adc::absolute,
    }, /* 0x6D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x6E */
    CpuInstruction {
        opcode: CpuOpcode::Adc,
        width: CpuParamWidth::Long,
        function: adc::absolute,
    }, /* 0x6F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x70 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x71 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x72 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x73 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x74 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x75 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x76 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x77 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x78 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x79 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x7A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x7B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x7C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x7D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x7E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x7F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x80 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x81 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x82 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x83 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x84 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x85 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x86 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x87 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x88 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x89 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x8A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x8B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x8C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x8D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x8E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x8F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x90 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x91 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x92 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x93 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x94 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x95 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x96 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x97 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x98 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x99 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x9A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x9B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x9C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x9D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x9E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0x9F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xA0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xA1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xA2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xA3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xA4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xA5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xA6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xA7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xA8 */
    CpuInstruction {
        opcode: CpuOpcode::Lda,
        width: CpuParamWidth::Word,
        function: lda::immediate,
    }, /* 0xA9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xAA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xAB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xAC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xAD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xAE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xAF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xB9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xBA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xBB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xBC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xBD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xBE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xBF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xC9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xCA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xCB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xCC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xCD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xCE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xCF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xD9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xDA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xDB Stp */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xDC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xDD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xDE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xDF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xE9 */
    CpuInstruction {
        opcode: CpuOpcode::Nop,
        width: CpuParamWidth::None,
        function: misc::nop,
    }, /* 0xEA Nop */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xEB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xEC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xED */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xEE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xEF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xF9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xFA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xFB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xFC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xFD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xFE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::None,
        function: misc::stp,
    }, /* 0xFF */
];

/**************************************** Public Functions **************************************************************/

/// Fetch an instruction from memory.
/// An instruction can be 8-bit or a 16-bit. We will just widen if it is 8-bit.
/// # Parameters
///     - `self`
///     - `memory`: Pointer to memory object to read data from.
/// # Returns
///     - `CpuInstruction`:     Decoded CPU instruction from the table.
pub(super) fn fetch_and_decode(cpu: &mut CpuState, memory: &memory::Memory) -> CpuInstruction {
    let address = cpu.get_pc();
    INSTRUCTION_MAP[memory.get_byte(address).unwrap() as usize]
}

/// Execute an instruction.
/// # Parameters
///     - `self`
///     - `inst`:    Instruction struct containing all relevant information about an operation.
///     - `p_mem`:   Mutable pointer to the memory for this instance.
/// # Returns
///     - true:      If continuing running
///     - false:     If a BRK or Stp has been reached.
pub(super) fn execute(
    cpu: &mut CpuState, inst: CpuInstruction, memory: &mut memory::Memory,
) -> bool {
    let mut arg: CpuInstructionFnArguments = CpuInstructionFnArguments {
        cpu,
        memory,
        bank: None,
        param: 0,
    };

    let mut parameter_location: usize = arg.cpu.get_pc() + INST_PARAM_OFFSET as usize;

    // Prepare the parameter.
    arg.param = match inst.width {
        CpuParamWidth::None => 0,
        CpuParamWidth::Variable => {
            match arg.cpu.registers.get_flag(registers::StatusFlags::AccSize) {
                registers::REGISTER_MODE_8_BIT => arg
                    .memory
                    .get_byte(parameter_location)
                    .expect("Parameter for instruction was out of bounds")
                    as u16,
                REGISTER_MODE_16_BIT => arg
                    .memory
                    .get_word(parameter_location)
                    .expect("Parameter for instruction was out of bounds"),
            }
        }
        CpuParamWidth::Byte => {
            arg.memory
                .get_byte(parameter_location)
                .expect("Parameter for instruction was out of bounds") as u16
        }
        CpuParamWidth::Word => arg
            .memory
            .get_word(parameter_location)
            .expect("Parameter for instruction was out of bounds"),
        CpuParamWidth::Long => {
            arg.bank = Some(
                arg.memory
                    .get_byte(parameter_location)
                    .expect("Bank for parameter was out of bounds"),
            );
            parameter_location += 1;
            arg.memory
                .get_word(parameter_location)
                .expect("Parameter for instruction was out of bounds")
        }
    };

    // Call the function to execute.
    println!(
        "Executing {:?} with parameter {:08X}",
        inst.opcode, arg.param
    );

    let running = match (inst.function)(&mut arg) {
        // TODO: Implement cycle pending
        Some(_cycle_count) => true,
        None => false,
    };

    // Calculate the PC Offset if the instruction was of variable len and then increment the pc.
    arg.cpu.registers.pc += if inst.width == CpuParamWidth::Variable {
        match arg.cpu.registers.get_flag(registers::StatusFlags::AccSize) {
            registers::REGISTER_MODE_8_BIT => CpuParamWidth::Byte as u16,
            registers::REGISTER_MODE_16_BIT => CpuParamWidth::Word as u16,
        }
    } else {
        inst.width as u16
    };

    running
}
