use crate::memory;

mod branch;
mod misc;

/**************************************** Constant Values ***************************************************************/
/// Number of instructions
const NUM_INSTRUCTIONS: usize = 256;

/// Parameter for an offset is always instruction + 1.
const INST_PARAM_OFFSET: u16 = 1;

/// Number of bytes to increment the PC by for an instruction.
/// Instruction is only one byte long.
const PC_INCREMENT_NO_ARG: u16 = 1;

/// Instruction takes an 8-bit parameter.
const PC_INCREMENT_SHORT_ARG: u16 = 2;

/// Instruction takes a  16-bit parameter.
const PC_INCREMENT_LONG_ARG: u16 = 3;

/// Map of the cpu opcodes.
/// Would prefer this to be a hashmap, but rust cannot generate a HashMap::From() as const, and a global cannot be declared using `let`.
const INSTRUCTION_MAP: [CpuInstruction; NUM_INSTRUCTIONS] = [
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x00 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x01 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x02 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x03 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x04 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x05 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x06 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x07 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x08 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x09 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x0A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x0B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x0C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x0D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x0E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x0F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x10 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x11 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x12 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x13 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x14 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x15 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x16 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x17 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x18 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x19 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x1A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x1B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x1C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x1D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x1E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x1F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x20 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x21 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x22 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x23 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x24 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x25 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x26 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x27 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x28 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x29 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x2A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x2B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x2C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x2D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x2E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x2F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x30 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x31 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x32 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x33 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x34 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x35 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x36 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x37 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x38 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x39 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x3A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x3B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x3C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x3D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x3E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x3F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x40 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x41 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x42 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x43 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x44 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x45 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x46 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x47 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x48 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x49 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x4A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x4B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x4C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x4D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x4E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x4F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x50 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x51 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x52 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x53 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x54 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x55 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x56 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x57 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x58 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x59 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x5A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x5B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x5C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x5D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x5E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x5F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x60 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x61 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x62 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x63 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x64 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x65 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x66 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x67 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x68 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x69 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x6A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x6B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x6C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x6D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x6E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x6F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x70 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x71 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x72 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x73 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x74 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x75 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x76 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x77 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x78 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x79 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x7A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x7B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x7C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x7D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x7E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x7F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x80 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x81 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x82 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x83 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x84 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x85 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x86 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x87 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x88 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x89 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x8A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x8B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x8C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x8D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x8E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x8F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x90 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x91 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x92 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x93 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x94 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x95 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x96 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x97 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x98 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x99 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x9A */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x9B */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x9C */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x9D */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x9E */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0x9F */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xA9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xAA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xAB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xAC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xAD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xAE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xAF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xB9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xBA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xBB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xBC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xBD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xBE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xBF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xC9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xCA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xCB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xCC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xCD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xCE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xCF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xD9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xDA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xDB Stp */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xDC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xDD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xDE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xDF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xE9 */
    CpuInstruction {
        opcode: CpuOpcode::Nop,
        width: CpuParamWidth::NO,
        function: misc::nop,
    }, /* 0xEA NOP */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xEB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xEC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xED */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xEE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xEF */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF0 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF1 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF2 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF3 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF4 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF5 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF6 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF7 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF8 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xF9 */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xFA */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xFB */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xFC */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xFD */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xFE */
    CpuInstruction {
        opcode: CpuOpcode::Stp,
        width: CpuParamWidth::NO,
        function: misc::stp,
    }, /* 0xFF */
];

/**************************************** Struct and Type definitions ***************************************************/

/// Enumerated type to match to an opcode. Useful for debugging because it can be represented easily as a string.
#[derive(Debug, Clone, Copy)]
pub enum CpuOpcode {
    Stp,
    Nop,
    // Many More
}

/// Defines width of operation.
/// - NO    = Bare opcode (e.g. NOP).
/// - SHORT = 8bit param.
/// - LONG  = 16bit param.
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
enum CpuParamWidth {
    NO     = PC_INCREMENT_NO_ARG,
    _SHORT = PC_INCREMENT_SHORT_ARG,
    _LONG  = PC_INCREMENT_LONG_ARG,
}

/// Generalized function signature for CPU Instruction functions.
/// Takes modifiable CPU State, Memory, and 16-bit parameter (widened if u8).
type CpuInstructionFn = fn(&mut CpuState, &mut memory::Memory, u16) -> bool;

/// A conglomerate wrapper of the prior enums.
///     - `opcode`      Opcode of next operation to run.
///     - `width`       Width of next operation, to calculate parameters.
///     - `function`    Function pointer to handler for next operation.
#[derive(Debug, Clone, Copy)]
struct CpuInstruction {
    opcode: CpuOpcode,
    width: CpuParamWidth,
    function: CpuInstructionFn,
}

/// Virtualized representation of the CPU internally.
#[derive(Debug)]
pub struct CpuState {
    _acc: u16,              // Accumulator
    pc: u16,                // Program Counter
    _sp: u16,               // Stack Pointer
    _flags: u8,             // Flags
    _direct_page: u16,      // Direct page addressing offset (Lower 4 bytes of address)
    _data_bank: u8,         // Reference to current data bank addr (Upper 2 bytes of address)
    prog_bank: u8,          // Reference to current bank of instr (Upper 2 bytes of address)
    pub cycles_to_pend: u8, // Number of cycles to pend before running next operation.
}

impl CpuState {
    /// Return a new blank CpuState instance.
    pub const fn new() -> Self {
        Self {
            _acc: 0x0000,
            pc: 0x8000,
            _sp: 0x0000,
            _flags: 0x00,
            _direct_page: 0x0000,
            _data_bank: 0x00,
            prog_bank: 0x80,
            cycles_to_pend: 0x00,
        }
    }

    /// Fetch, Decode, Execute the next instruction, and return false if the VM needs to stop running.
    /// # Parameters
    ///     - `self`
    ///     - `memory`: Mutable pointer to current memory state.
    /// # Returns
    ///     - `true`:    If running,
    ///     - `false`:   If run should halt.
    pub fn step(&mut self, mem: &mut memory::Memory) -> bool {
        let next_instruction = self.fetch_and_decode(mem);
        self.execute(next_instruction, mem)
    }

    /// Fetch an instruction from memory.
    /// An instruction can be an 8-bit or a 16-bit one. We will just widen if it is 8-bit.
    /// # Parameters
    ///     - `self`
    ///     - `memory`: Pointer to memory object to read data from.
    /// # Returns
    ///     - `CpuInstruction`:     Decoded CPU instruction from the hash table.
    fn fetch_and_decode(&self, p_mem: &memory::Memory) -> CpuInstruction {
        let address = memory::compose_address(self.prog_bank, self.pc);
        INSTRUCTION_MAP[p_mem.get_byte(address).unwrap() as usize]
    }

    /// Execute an instruction.
    /// # Parameters
    ///     - `self`
    ///     - `inst`    Instruction struct containing all relevant information about an operation.
    ///     - `p_mem`   Mutable pointer to the memory for this instance.
    /// # Returns
    ///     - true:      If continuing running
    ///     - false:     If a BRK or Stp has been reached.
    fn execute(&mut self, inst: CpuInstruction, p_mem: &mut memory::Memory) -> bool {
        let parameter_location: usize =
            memory::compose_address(self.prog_bank, self.pc + INST_PARAM_OFFSET);
        let pc_addr_increment: u16 = inst.width as u16;

        let parameter_value = match inst.width {
            CpuParamWidth::NO => 0,
            CpuParamWidth::_SHORT => p_mem.get_byte(parameter_location).unwrap() as u16,
            CpuParamWidth::_LONG => p_mem.get_word(parameter_location).unwrap(),
        };

        // Call the function to execute.
        println!("Executing {:?} with width {:?}", inst.opcode, inst.width);
        let continue_run = (inst.function)(self, p_mem, parameter_value);
        self.pc += pc_addr_increment;

        continue_run
    }

    /// Print the current state of the CPU.
    pub fn _print_state(&self) {
        print!(
            "\nPC: {:#08X} ACC: {:#06X} SP: {:#06X}\nData Bank: {:#04X} Prog Bank: {:#04X} Direct Page: {:#06X}\nFlags nvmxdizc: {:#04X}\n    {:#010b}\n"
             ,self.pc, self._acc, self._sp, self._data_bank, self.prog_bank, self._direct_page, self._flags, self._flags
        );
    }

    /// Compose a fully-formed absolute address from the current PC and return it.
    pub fn get_pc(&self) -> usize { memory::compose_address(self.prog_bank, self.pc) }
}
/**************************************** File Scope Functions **********************************************************/

/**************************************** Tests *************************************************************************/
