use core::fmt;
use std::{fs, fmt::Display, io::Read, num::Wrapping, path::PathBuf};
use crate::memory;

/********************************* ROM Info Constants **************************************************/

/// Layout in memory is low->high [Optional Header, Header, Exception Vectors]. Always lives at the end of the first bank of a ROM.
/// Optional Header Breakdown (16 bytes)
const _OPT_MAKER_CODE_LEN:               usize = 2;
const _OPT_GAME_CODE_LEN:                usize = 4;
const _OPT_FIXED_VAL_LEN:                usize = 7;
const _OPT_EXPANSION_RAM_LEN:            usize = 1;
const _OPT_SPECIAL_VERSION_LEN:          usize = 1;
const _OPT_SUB_CART_TYPE_LEN:            usize = 1;
const OPT_HEADER_LEN_BYTES:              usize = 16;

const _OPT_MAKER_CODE_INDEX:             usize = 0;
const _OPT_GAME_CODE_INDEX:              usize = _OPT_MAKER_CODE_LEN;
const _OPT_FIXED_VAL_INDEX:              usize = _OPT_GAME_CODE_INDEX + _OPT_GAME_CODE_LEN;
const _OPT_EXPANSION_RAM_INDEX:          usize = _OPT_FIXED_VAL_INDEX + _OPT_FIXED_VAL_LEN;
const _OPT_SPECIAL_VERSION_INDEX:        usize = _OPT_EXPANSION_RAM_INDEX + _OPT_EXPANSION_RAM_LEN;
const _OPT_SUB_CART_TYPE_INDEX:          usize = _OPT_SPECIAL_VERSION_INDEX + _OPT_SUB_CART_TYPE_LEN;

/// Header Breakdown (32 bytes)
const HDR_TITLE_LEN:                    usize = 21;
const HDR_MAP_MODE_LEN:                 usize = 1;
const HDR_CART_TYPE_LEN:                usize = 1;
const HDR_ROM_SIZE_LEN:                 usize = 1;
const HDR_RAM_SIZE_LEN:                 usize = 1;
const HDR_DEST_CODE_LEN:                usize = 1;
const HDR_FIXED_VAL_LEN:                usize = 1;
const HDR_MASK_ROM_VER_LEN:             usize = 1;
const HDR_COMPLEMENT_CHECK_LEN:         usize = 2;
const HDR_CHECKSUM_LEN:                 usize = 2;
const HDR_LEN_BYTES:                    usize = 32;  

const HDR_TITLE_INDEX:                  usize = 0;
const HDR_MAP_MODE_INDEX:               usize = HDR_TITLE_LEN;
const HDR_CART_TYPE_INDEX:              usize = HDR_MAP_MODE_INDEX + HDR_MAP_MODE_LEN;
const HDR_ROM_SIZE_INDEX:               usize = HDR_CART_TYPE_INDEX + HDR_CART_TYPE_LEN;
const HDR_RAM_SIZE_INDEX:               usize = HDR_ROM_SIZE_INDEX + HDR_ROM_SIZE_LEN;
const HDR_DEST_CODE_INDEX:              usize = HDR_RAM_SIZE_INDEX + HDR_RAM_SIZE_LEN;
const HDR_FIXED_VAL_INDEX:              usize = HDR_DEST_CODE_INDEX + HDR_DEST_CODE_LEN;
const HDR_MASK_ROM_VER_INDEX:           usize = HDR_FIXED_VAL_INDEX + HDR_FIXED_VAL_LEN;
const HDR_COMPLEMENT_CHECK_INDEX:       usize = HDR_MASK_ROM_VER_INDEX + HDR_MASK_ROM_VER_LEN;
const HDR_CHECKSUM_INDEX:               usize = HDR_COMPLEMENT_CHECK_INDEX + HDR_COMPLEMENT_CHECK_LEN;

/// Exception Vector Breakdown (16 bytes)
const _EV_NATIVE_UNUSED_1_LEN:          usize = 4;
const EV_NATIVE_COP_LEN:                usize = 2;
const EV_NATIVE_BRK_LEN:                usize = 2;
const EV_NATIVE_ABORT_LEN:              usize = 2;
const EV_NATIVE_NMI_LEN:                usize = 2;
const _EV_NATIVE_UNUSED_2_LEN:          usize = 2;
const EV_NATIVE_IRQ_LEN:                usize = 2;
const _EV_EMU_UNUSED_1_LEN:             usize = 4;
const EV_EMU_COP_LEN:                   usize = 2;
const _EV_EMU_UNUSED_2_LEN:             usize = 2;
const EV_EMU_ABORT_LEN:                 usize = 2;
const EV_EMU_NMI_LEN:                   usize = 2;
const EV_EMU_RESET_LEN:                 usize = 2;
const EV_EMU_IRQ_BRK_LEN:               usize = 2;
const EV_LEN_BYTES:                     usize = 32;

// NATIVE UNUSED 1
const EV_NATIVE_COP_INDEX:              usize = _EV_NATIVE_UNUSED_1_LEN;
const EV_NATIVE_BRK_INDEX:              usize = EV_NATIVE_COP_INDEX + EV_NATIVE_COP_LEN;
const EV_NATIVE_ABORT_INDEX:            usize = EV_NATIVE_BRK_INDEX + EV_NATIVE_BRK_LEN;
const EV_NATIVE_NMI_INDEX:              usize = EV_NATIVE_ABORT_INDEX + EV_NATIVE_ABORT_LEN;
// NATIVE UNUSED 2
const EV_NATIVE_IRQ_INDEX:              usize = EV_NATIVE_NMI_INDEX + _EV_NATIVE_UNUSED_2_LEN;
// EMU UNUSED 1
const EV_EMU_COP_INDEX:                 usize = EV_NATIVE_IRQ_INDEX + _EV_EMU_UNUSED_2_LEN;
// EMU UNUSED 2
const EV_EMU_ABORT_INDEX:               usize = EV_EMU_COP_INDEX + _EV_EMU_UNUSED_2_LEN;
const EV_EMU_NMI_INDEX:                 usize = EV_EMU_ABORT_INDEX + EV_EMU_ABORT_LEN;
const EV_EMU_RESET_INDEX:               usize = EV_EMU_NMI_INDEX + EV_EMU_NMI_LEN;
const EV_EMU_IRQ_BRK_INDEX:             usize = EV_EMU_RESET_INDEX + EV_EMU_RESET_LEN;

/// LoRom specific values
const LO_ROM_BANK_ADDR:                 usize = 0x80;      // LoRom starts at bank $808000 and is mirrored to $008000.
const LO_ROM_BANK_SIZE_BYTES:           usize = 32 * 1024; // LoRom, ExLoRom Bank size is 32 KiB
const LO_ROM_EXC_VECTOR_ADDR:           usize = LO_ROM_BANK_SIZE_BYTES - EV_LEN_BYTES;
const LO_ROM_HEADER_ADDR:               usize = LO_ROM_EXC_VECTOR_ADDR - HDR_LEN_BYTES;
const LO_ROM_EXT_HEADER_ADDR:           usize = LO_ROM_HEADER_ADDR - OPT_HEADER_LEN_BYTES;

/// HiRom specific values
const HI_ROM_BANK_ADDR:                 usize = 0xC0;      // HiRom starts at bank $C00000 through to $FFFFFF.
const HI_ROM_BANK_SIZE_BYTES:           usize = 64 * 1024; // HiRom, ExHiRom Bank size is 64 KiB
const HI_ROM_EXC_VECTOR_ADDR:           usize = LO_ROM_BANK_SIZE_BYTES - EV_LEN_BYTES;
const HI_ROM_HEADER_ADDR:               usize = HI_ROM_EXC_VECTOR_ADDR - HDR_LEN_BYTES;
const HI_ROM_EXT_HEADER_ADDR:           usize = HI_ROM_HEADER_ADDR - OPT_HEADER_LEN_BYTES;

/// Public constants
pub const ROM_BASE_ADDR:                usize = 0x8000;    // All LoRom banks, and mirrored banks of both Lo and HiRom fall under $XX8000. E.G.: Bank 0: $808000, Bank 1: $908000

/********************************* ROM Info Enums & Struct Definitions *********************************/
/// Info pertaining to the memory map and size of the ROM.
pub enum RomSize {
    ExLoRom,
    LoRom,
    HiRom,
    ExHiRom
}

/// Info pertaining to the CPU clock speed the SNES runs at for this ROM.
/// SlowRom = 2.68MHz
/// FastRom = 3.58MHz
pub enum RomClkSpeed {
    SlowRom,    // 2.68MHz
    FastRom     // 3.58MHz
}

/// ROM Expansion chips which are noted and affect memory map.
/// Note that SuperFX and others have their own memory map which is not covered by this.
pub enum RomExpansions {
    None,
    SA1,
    SDD1
}

/// https://snes.nesdev.org/wiki/Memory_map#LoROM
/// https://sneslab.net/wiki/SNES_ROM_Header
/// Conceptualizing this gives me a bit of a headache.
pub struct RomMemoryMap {
    sram_start: usize,
    sram_len:   usize,
    rom_start:  usize,
    rom_len:    usize,
    rom_mirror_1_start: usize,
    rom_mirror_1_len:   usize,
    rom_mirror_2_start: usize,
    rom_mirror_2_len:   usize
}

impl RomMemoryMap {
    pub fn new() -> Self {
        Self {
            sram_start: 0,
            sram_len: 0,
            rom_start: 0,
            rom_len: 0,
            rom_mirror_1_start: 0,
            rom_mirror_1_len: 0,
            rom_mirror_2_start: 0,
            rom_mirror_2_len: 0
        }
    }
}

/// https://snes.nesdev.org/wiki/ROM_header#Header_Verification
/// https://sneslab.net/wiki/SNES_ROM_Header
/// Struct which contains header data and references to it for use externally.
pub struct HeaderData {
    header_data: [u8; HDR_LEN_BYTES],
    opt_header_data: [u8; OPT_HEADER_LEN_BYTES],
    opt_is_present: bool,
}

impl HeaderData{
    /// Return an empty HeaderData struct.
    pub fn new() -> Self {
        Self {
            header_data: [0; HDR_LEN_BYTES],
            opt_header_data: [0; OPT_HEADER_LEN_BYTES],
            opt_is_present: false
        }
    }
}

/// Struct for a RomReadError if an error occurred on parse.
#[derive(Debug, Clone)]
pub struct RomReadError {
    context: String
}

impl RomReadError {
    pub fn new(ctx: String) -> Self {
        Self {
            context: ctx
        }
    }
}

impl Display for RomReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RomReadError: {}", self.context)
    } 
}

/********************************* ROM Info Functions **************************************************/
/// Check file, attempt to read, and then discern information about it. If possible, populate the memory.
/// # Parameters:
///     - `path`:       Path to file to open.
///     - `memory`:     Pointer to memory to warm up.
/// # Returns:
///     - `Ok(HeaderData)`:     Parsed header data, if it was available.
///     - `Err(RomReadError)`:  Error with context 
pub fn load_rom(path: PathBuf, mut memory: &memory::Memory) -> Result<HeaderData, RomReadError> {
    let retval: Result<(), RomReadError>;
    
    // Check and just immediately flop if an SMC is passed until we manage that.
    if path.extension().unwrap() == "smc" {
        return Err(RomReadError{ context: format!("SMC files contain additional data that is unneeded.\nThis tool does not support them yet. Please use an SFC file.")});
    }

    if let mut file = fs::File::open(&path).unwrap(){
        let mut buf: Vec<u8>;
        if let read_result = file.read_to_end(&mut buf).unwrap_err(){
            retval = Err(RomReadError{ context: format!("{}", read_result) });
        }
        else{
            
        }
    }
    else{
        retval = Err(RomReadError{ context: format!("Failed to open file at {}", &path.display()) });
    }

    retval
}

fn find_header(rom: &Vec<u8>) -> Result<HeaderData, RomReadError> {
    let mut checksum: Wrapping<u16> = Wrapping(0);
    
    // Sum all bytes in the file. overflow is fine.
    rom.iter().map(|x| checksum + Wrapping(*x as u16));
    
    // Check 0: Extract where the header would live in LoRom, and see if checksum in it matches.
    let test_header = &rom[LO_ROM_HEADER_ADDR .. (LO_ROM_HEADER_ADDR + HDR_LEN_BYTES)];

    OK(())
}

/********************************* ROM Info Tests ******************************************************/