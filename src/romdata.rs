use core::fmt;
use std::{fmt::Display, fs, io::Read, num::Wrapping, path::PathBuf};
use crate::memory::{self, IntoWord};

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
const HDR_TEST_VALUE:                   u16 = 0xFFFF; // A test checksum + the complement value should equal this value.

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
const LO_ROM_BANK_ADDR:                 u8 = 0x80;      // LoRom starts at bank $808000 and is mirrored to $008000.
const LO_ROM_BANK_SIZE_BYTES:           usize = 32 * 1024; // LoRom, ExLoRom Bank size is 32 KiB
const LO_ROM_EXC_VECTOR_ADDR:           usize = LO_ROM_BANK_SIZE_BYTES - EV_LEN_BYTES;
const LO_ROM_HEADER_ADDR:               usize = LO_ROM_EXC_VECTOR_ADDR - HDR_LEN_BYTES;
const LO_ROM_EXT_HEADER_ADDR:           usize = LO_ROM_HEADER_ADDR - OPT_HEADER_LEN_BYTES;

/// HiRom specific values
const HI_ROM_BANK_ADDR:                 u8 = 0xC0;      // HiRom starts at bank $C00000 through to $FFFFFF.
const HI_ROM_BANK_SIZE_BYTES:           usize = 64 * 1024; // HiRom, ExHiRom Bank size is 64 KiB
const HI_ROM_EXC_VECTOR_ADDR:           usize = LO_ROM_BANK_SIZE_BYTES - EV_LEN_BYTES;
const HI_ROM_HEADER_ADDR:               usize = HI_ROM_EXC_VECTOR_ADDR - HDR_LEN_BYTES;
const HI_ROM_EXT_HEADER_ADDR:           usize = HI_ROM_HEADER_ADDR - OPT_HEADER_LEN_BYTES;

/// Map mode values
const MAP_HIROM_MASK:                   u8 = 0b00000001;
const MAP_SA1_MASK:                     u8 = 0b00000010;
const MAP_EXHIROM_MASK:                 u8 = 0b00000100;
// Unused Bit                                0b00001000 
const MAP_FASTROM_MASK:                 u8 = 0b00010000;
const MAP_BASE_MASK:                    u8 = 0b00100000;
/// Public constants
pub const ROM_BASE_ADDR:                u16 = 0x8000;    // All LoRom banks, and mirrored banks of both Lo and HiRom fall under $XX8000. E.G.: Bank 0: $808000, Bank 1: $908000

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

/// Cart type doesn't really map linearly, which is kind of a headache.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CartType {
    ROMOnly             = 0x00,
    ROMSram             = 0x01,
    ROMSramBattery      = 0x02,
    ROMCoCpu            = 0x03,
    ROMCoCpuSram        = 0x04,
    ROMCoCpuSramBattery = 0x05,
    ROMCoCpuBattery     = 0x06,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum RomCoProcessor {
    DSP                 = 0x00,
    SuperFX             = 0x01,
    OBC1                = 0x02,
    S_DD1               = 0x03,
    S_RTC               = 0x04,
    Other               = 0x05,
    Custom              = 0xFF
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CustomCoProcessor {
    SPC7110             = 0x00,
    ST010_ST011         = 0x01,
    ST018               = 0x02,
    CX4                 = 0x03,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum RomRegion {
    Japan               = 0x00,
    USA                 = 0x01,
    Europe              = 0x02,
    Sweden              = 0x03,
    Japan2              = 0x04,
    Denmark             = 0x05,
    France              = 0x06,
    Netherlands         = 0x07,
    Spain               = 0x08,
    Germany             = 0x09,
    Italy               = 0x0A,
    China               = 0x0B,
    Indonesia           = 0x0C,
    SouthKorea          = 0x0D,
    International       = 0x0E,
    Canada              = 0x0F,
    Brazil              = 0x10,
    Australia           = 0x11,
}

type Header = [u8; HDR_LEN_BYTES];
type OptionalHeader = [u8; OPT_HEADER_LEN_BYTES];
type ExceptionVectorTable = [u8; EV_LEN_BYTES];

pub struct RomModeMapping {
    mem_map: RomSize,
    speed: RomClkSpeed,
    cart_type: CartType,
    region:  RomRegion,
    expansion_present: bool
}

impl RomModeMapping {
    fn new() -> Self {
        Self {
            mem_map: RomSize::LoRom,
            speed: RomClkSpeed::SlowRom,
            cart_type: CartType::ROMOnly,
            region: RomRegion::Japan,
            expansion_present: false
        }
    }
}

/// https://snes.nesdev.org/wiki/ROM_header#Header_Verification
/// https://sneslab.net/wiki/SNES_ROM_Header
/// Struct which contains header data and references to it for use externally.
pub struct RomData {
    header: Header,
    opt_header: OptionalHeader,
    exception_vectors: ExceptionVectorTable,
    opt_is_present: bool,
}

impl RomData{
    /// Return an empty HeaderData struct.
    pub fn new() -> Self {
        Self {
            header: [0; HDR_LEN_BYTES],
            opt_header: [0; OPT_HEADER_LEN_BYTES],
            exception_vectors: [0; EV_LEN_BYTES],
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

/// Perform all of the startup operations and fetch all the data necessary before runtime.
/// # Parameters:
///     - `path`:       Path to file to open
///     - `memory`:     Memory to prepare.
pub fn load_rom(path: PathBuf, mut memory: &memory::Memory) -> Result<(), RomReadError> {
    // Attempt to read to buffer.
    let rom = read_rom_to_buf(path)?;
    let mut header = fetch_header(&rom)?;

    Ok(())
}

/// Check file, attempt to read, and then discern information about it. If possible, populate the memory.
/// # Parameters:
///     - `path`:       Path to file to open.
/// # Returns:
///     - `Ok(Vec<u8>)`:        Vector containing all ROM data if successful.
///     - `Err(RomReadError)`:  Error with context 
fn read_rom_to_buf(path: PathBuf) -> Result<Vec<u8>, RomReadError> {
    let retval: Result<Vec<u8>, RomReadError>;
    
    // Check and just immediatelyVirtualMachine flop if an SMC is passed until we manage that.
    if path.extension().unwrap() == "smc" {
        return Err(RomReadError{ context: format!("SMC files contain additional data that is unneeded.\nThis tool does not support them yet. Please use an SFC file.")});
    }

    // Attempt to open the file, and load it into a buffer.
    let file = fs::File::open(&path);
    
    if file.is_ok()
    {
        let mut buf: Vec<u8> = vec![];
        let _read_result = match file.unwrap().read_to_end(&mut buf) {
            Ok(_) => { retval = Ok(buf); },
            Err(e) => {retval = Err(RomReadError{ context: format!("Failed to read file: {}", e)});},
        };
    }
    else{
        retval = Err(RomReadError{ context: format!("Failed to open file at {}", &path.display()) });
    }

    // If the file was read successfully, operate on it.


    retval
}

/// Find and grab the header from target rom if available.
/// # Parameters: 
///     - `rom`:    Pointer to rom data to analyze.
/// # Returns:
///     - `HeaderData` struct with copies of the Header, Exception Vector, and Optional Header (if present) values.
///     - `RomReadError` if the 
fn fetch_header(rom: &Vec<u8>) -> Result<RomData, RomReadError> {
    let retval: Result<RomData, RomReadError>;
    // Sum all bytes in the file. overflow is fine.
    let checksum: Wrapping<u16> = Wrapping(0);
    rom.iter().map(|x| checksum + Wrapping(*x as u16));

    let mut header: Header = [0; HDR_LEN_BYTES];
    let mapping: Result<RomModeMapping, RomReadError>;
    
    // Check the bounds, then excise where a header would be and test for the mapping.
    if rom.capacity() > HI_ROM_BANK_SIZE_BYTES
    {
        header.clone_from_slice(&rom[HI_ROM_HEADER_ADDR .. (HI_ROM_HEADER_ADDR + HDR_LEN_BYTES)]);
        match test_checksum(checksum.0, &header){
            Ok(_t) => { mapping = gather_mapping(&header); },
            Err(e) => { retval = Err(e); }
        };
    }
    else if rom.capacity() > LO_ROM_BANK_SIZE_BYTES
    {
        header.clone_from_slice(&rom[LO_ROM_HEADER_ADDR .. (LO_ROM_HEADER_ADDR + HDR_LEN_BYTES)]);
        match test_checksum(checksum.0, &header){
            Ok(_t) => { mapping = gather_mapping(&header); },
            Err(e) => { retval = Err(e); }
        };
    }
    else {
        retval = Err(RomReadError{ context: "File was too small to contain header".to_string() });
    }

    Err(RomReadError{context: "Function implementation is not complete".to_string() })
}

fn gather_mapping(header: &Header) -> Result<RomModeMapping, RomReadError> {
    Ok(RomModeMapping::new())
}

fn test_checksum(checksum: u16, header: &Header) -> Result<(), RomReadError> {
    let test_checksum: [u8; 2] = header[HDR_CHECKSUM_INDEX .. HDR_CHECKSUM_INDEX + 1].try_into().unwrap();
    let test_compare: [u8; 2] = header[HDR_COMPLEMENT_CHECK_INDEX .. HDR_COMPLEMENT_CHECK_INDEX + 1].try_into().unwrap();


    if checksum == test_checksum.to_word() && 
        ((Wrapping(checksum) + Wrapping(test_compare.to_word())).0 == HDR_TEST_VALUE) {
        // This rom looks good. Actually fetch the values for this.
        println!("Checksum was good!");
    }

    Ok(())
}

/********************************* ROM Info Tests ******************************************************/