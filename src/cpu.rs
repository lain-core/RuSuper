use std::{path::PathBuf, time};
use crate::{memory, romdata};

// Instructions mod declarations

// Generalized ASM Help: https://ersanio.gitbook.io/assembly-for-the-snes
// SFC Dev Wiki: https://wiki.superfamicom.org/learning-65816-assembly
// ASAR Docs: https://rpghacker.github.io/asar/asar_2_beta/arch-65816.html
mod branch;
mod misc;

/***** Instruction/Ops related constants *****/
const NUM_INSTRUCTIONS:                         usize   = 256;              /// Number of instructions
const INST_PARAM_OFFSET:                        u16     = 1;                /// Parameter for an offset is always instruction + 1.

/*  Number of bytes to increment the PC by for an instruction. */
const PC_INCREMENT_NO_ARG:                      u16     = 1;                /// Instruction is only one byte long.
const PC_INCREMENT_SHORT_ARG:                   u16     = 2;                /// Instruction takes an 8-bit parameter.
const PC_INCREMENT_LONG_ARG:                    u16     = 3;                /// Instruction takes a  16-bit parameter. 

/// Map of the cpu opcodes. 
/// Would prefer this to be a hashmap, but rust cannot generate a HashMap::From() as const, and a global cannot be declared using `let`.
const INSTRUCTION_MAP: [CpuInstruction; NUM_INSTRUCTIONS] = [
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x00 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x01 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x02 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x03 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x04 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x05 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x06 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x07 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x08 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x09 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x0A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x0B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x0C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x0D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x0E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x0F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x10 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x11 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x12 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x13 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x14 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x15 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x16 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x17 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x18 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x19 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x1A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x1B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x1C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x1D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x1E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x1F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x20 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x21 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x22 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x23 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x24 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x25 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x26 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x27 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x28 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x29 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x2A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x2B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x2C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x2D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x2E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x2F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x30 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x31 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x32 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x33 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x34 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x35 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x36 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x37 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x38 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x39 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x3A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x3B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x3C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x3D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x3E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x3F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x40 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x41 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x42 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x43 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x44 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x45 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x46 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x47 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x48 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x49 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x4A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x4B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x4C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x4D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x4E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x4F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x50 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x51 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x52 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x53 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x54 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x55 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x56 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x57 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x58 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x59 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x5A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x5B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x5C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x5D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x5E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x5F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x60 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x61 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x62 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x63 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x64 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x65 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x66 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x67 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x68 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x69 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x6A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x6B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x6C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x6D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x6E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x6F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x70 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x71 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x72 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x73 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x74 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x75 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x76 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x77 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x78 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x79 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x7A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x7B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x7C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x7D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x7E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x7F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x80 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x81 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x82 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x83 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x84 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x85 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x86 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x87 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x88 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x89 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x8A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x8B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x8C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x8D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x8E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x8F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x90 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x91 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x92 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x93 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x94 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x95 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x96 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x97 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x98 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x99 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x9A */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x9B */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x9C */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x9D */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x9E */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0x9F */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA0 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA1 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA2 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA3 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA4 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA5 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA6 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA7 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA8 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xA9 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xAA */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xAB */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xAC */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xAD */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xAE */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xAF */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB0 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB1 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB2 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB3 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB4 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB5 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB6 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB7 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB8 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xB9 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xBA */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xBB */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xBC */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xBD */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xBE */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xBF */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC0 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC1 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC2 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC3 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC4 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC5 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC6 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC7 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC8 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xC9 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xCA */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xCB */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xCC */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xCD */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xCE */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xCF */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD0 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD1 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD2 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD3 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD4 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD5 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD6 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD7 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD8 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xD9 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xDA */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xDB STP */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xDC */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xDD */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xDE */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xDF */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE0 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE1 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE2 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE3 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE4 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE5 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE6 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE7 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE8 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xE9 */
    CpuInstruction{opcode: CpuOpcode::NOP, width: CpuParamWidth::NO, function: misc::nop}, /* 0xEA NOP */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xEB */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xEC */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xED */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xEE */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xEF */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF0 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF1 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF2 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF3 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF4 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF5 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF6 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF7 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF8 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xF9 */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xFA */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xFB */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xFC */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xFD */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xFE */
    CpuInstruction{opcode: CpuOpcode::STP, width: CpuParamWidth::NO, function: misc::stp}, /* 0xFF */
];

/***** Timing related constants *****/
// The SNES master clock runs at about 21.477MHz NTSC (theoretically 1.89e9/88 Hz).
// https://wiki.superfamicom.org/timing
const CLOCK_CYCLE_TICK_NS:                      f64     = 46.5614378172;    /// Approximation of 1x 21.477MHz pulse in nanoseconds 
const CYCLES_PER_SCANLINE:                      usize   = 1364;             /// Number of cycles between draw of scanline.
const NON_INTERLACE_MODE_ALTERNATE_CYCLES_PER:  usize   = 1360;             /// Every other frame in non-interlaced, 4 less cycles per frame. This is "extra credit".
const SCANLINES_PER_FRAME:                      usize   = 262;              /// Number of scanlines per 1 frame (e.g. 60Hz)
const CYCLES_PER_FRAME:                         usize   = SCANLINES_PER_FRAME * CYCLES_PER_SCANLINE;    /// Number of cycles per 1 frame (e.g. 60Hz)

/***** Implementation of enums and structures for CPU *****/

/// VM Struct which contains the individual pieces of the system.
struct VirtualMachine {
    cpu: CpuState,
    memory: memory::Memory,
    romdata: romdata::RomData
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            cpu: CpuState::new(),
            memory: memory::Memory::new(),
            romdata: romdata::RomData::new()
        }
    }
}

/// Enumerated type to match to an opcode. Useful for debugging because it can be represented easily as a string.
/// https://wiki.superfamicom.org/65816-reference
#[derive(Debug, Clone, Copy)]
pub enum CpuOpcode {
    STP,
    NOP,
    // Many More
}

/// Defines width of operation. 
/// - NO    = Bare opcode (e.g. NOP). 
/// - SHORT = 8bit param. 
/// - LONG  = 16bit param.
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
enum CpuParamWidth {
    NO      = PC_INCREMENT_NO_ARG,
    SHORT   = PC_INCREMENT_SHORT_ARG,
    LONG    = PC_INCREMENT_LONG_ARG
}

/// A conglomerate wrapper of the prior enums.
///     - `opcode`      Opcode of next operation
///     - `width`       Width of next operation, to calculate parameters.
///     - `function`    Function pointer to handler for next operation.
#[derive(Debug, Clone, Copy)]
struct CpuInstruction {
    opcode: CpuOpcode,
    width: CpuParamWidth,
    function: fn(&mut CpuState, &mut memory::Memory, u16) -> bool
}

impl CpuInstruction {
    /// Generates a NOP.
    pub fn new() -> Self {
        Self { opcode: CpuOpcode::NOP, width: CpuParamWidth::NO, function: misc::nop }
    }
}

/// Virtualized representation of the CPU internally.
#[derive(Debug)]
pub struct CpuState {
    acc: u16,               // Accumulator TODO: Union this
    pc: u16,                // Program Counter
    sp: u16,                // Stack Pointer
    flags: u8,              // Flags TODO: this should be a union of bits
    direct_page: u16,       // Direct page addressing offset  
    data_bank: u8,          // Reference to current data bank addr
    prog_bank: u8,          // Reference to current bank of instr
    pub cycles_to_pend: u8  // Number of cycles to pend before running next operation. 
}

/* Associated Functions */
impl CpuState {
    /// Return a new blank CpuState instance.
    pub const fn new() -> Self {
        Self {
            acc: 0x0000,
            pc: 0x0000,
            sp: 0x0000,
            flags: 0x00,
            direct_page: 0x0000,
            data_bank: 0x00,
            prog_bank: 0x00,
            cycles_to_pend: 0x00
        }
    }

    /// Fetch, Decode, Execute the next instruction, and return false if we encountered a HALT.
    /// # Parameters
    ///     - `memory`: Mutable pointer to current memory state.
    /// # Returns
    ///     - `true` if running,
    ///     - `false` if run should halt.
    pub fn step(&mut self, mut mem: &mut memory::Memory) -> bool {
        let next_instruction = self.fetch_and_decode(&mem);
        self.execute(next_instruction, &mut mem)
    }

    /// Fetch an instruction from memory.
    /// An instruction can be an 8-bit or a 16-bit one. We will just widen if it is 8-bit.
    /// # Parameters
    ///     - `self`
    ///     - `memory`: Pointer to memory object to read data from.
    fn fetch_and_decode(&self, p_mem: &memory::Memory) -> CpuInstruction {
        let address = memory::compose_address(self.prog_bank, self.pc);
        INSTRUCTION_MAP[ p_mem.get_byte(address).unwrap() as usize ]
    }

    /// Execute an instruction.
    /// # Parameters
    ///     - `self`
    ///     - `inst`    Instruction struct containing all relevant information about an operation.
    ///     - `p_mem`   Mutable pointer to the memory for this instance.
    /// # Returns
    ///     - true      If continuing running
    ///     - false     If a BRK or STP has been reached.
    fn execute(&mut self, inst: CpuInstruction, mut p_mem: &mut memory::Memory) -> bool {
        let continue_run: bool;
        let parameter_location: usize = memory::compose_address(self.prog_bank, self.pc + INST_PARAM_OFFSET);
        let parameter_value: u16;    // Calculated parameter value, if applicable.
        let pc_addr_increment: u16 = inst.width as u16;  // Number of bytes to increment the PC by after this operation.
        
        match inst.width {
            CpuParamWidth::NO       => { parameter_value = 0 },
            CpuParamWidth::SHORT    => { parameter_value = p_mem.get_byte(parameter_location).unwrap() as u16 },
            CpuParamWidth::LONG     => { parameter_value = p_mem.get_word(parameter_location).unwrap() }
        }

        // Call the function to execute.
        println!("Executing {:#?}", inst);
        continue_run = (inst.function)(self, &mut p_mem, parameter_value);
        self.pc += pc_addr_increment;

        continue_run
    }
}

/***** File scope functions *****/

/// Run the system.
/// Also manages timings and delegates to other legs of the system. Might be worth breaking up in the future.
/// # Parameters
///     - `vm`  Object holding CPU state and Memory for this instance.
pub fn run(mut path: std::path::PathBuf) {
    // TODO: Spin off thread for debugger
    // TODO: Spin off thread for SPC700
    // TODO: Spin off thread for PPU(?)

    // Initialize the VM and then load the ROM into memory.
    let mut vm = VirtualMachine::new();
    romdata::load_rom(path, &mut vm.memory);

    // Debugger loop which parses user inputs. 
    let mut vm_running = true;

    // Track number of cycles to do calculations on. Doesn't matter if this rolls over.
    let mut cycles_elapsed: std::num::Wrapping<usize> = std::num::Wrapping(0);
    loop {
        // Check if the vm is running and step if so.
        // This is not self-contained in a loop because the outside will contain debugger functions in the future.
        // The SNES master clock runs at about 21.477MHz NTSC (theoretically 1.89e9/88 Hz).
        // The SNES CPU runs at either 2.68MHz or 3.58MHz based on what a rom requests.
        // https://wiki.superfamicom.org/timing

        if vm_running {
            
            // Draw a scanline.
            if cycles_elapsed % std::num::Wrapping(CYCLES_PER_SCANLINE as usize) == std::num::Wrapping(0) {
                // TODO: PPU something
            }

            // If there is no need to pend on another cycle, then go ahead and run an operation.
            if vm.cpu.cycles_to_pend == 0 {
                vm_running = vm.cpu.step(&mut vm.memory);
                println!("Next instruction stalled by {} cycles", vm.cpu.cycles_to_pend);
            }
            // Otherwise, punt on operating for however long we need to.
            else if vm.cpu.cycles_to_pend > 0 {
                // We have to round because rust does not implement fractional nanoseconds (how unbelievable!!)
                std::thread::sleep( time::Duration::from_nanos(CLOCK_CYCLE_TICK_NS as u64) );
                cycles_elapsed += 1;
                vm.cpu.cycles_to_pend -= 1;
            }
        }





    }
}

/***** Tests *****/
#[cfg(test)]
mod tests {
    use self::memory::Memory;
    use super::*;
}