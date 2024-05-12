// #![allow(unused)]
// There are a lot of currently unused const values in this file, but they are important to structural understanding
//  and may become more useful in the future. This is disabled while still implementing functions.

use core::fmt;
use std::{fmt::Display, fs, io::Read, num::Wrapping, path::PathBuf};

use crate::memory::{self, compose_address};

/********************************* ROM Info Constants **************************************************/

/// Layout in memory is low->high [Optional Header, Header, Exception Vectors]. Always lives at the end of the first bank of a ROM.
/// Optional Header Breakdown (16 bytes)
const OPT_MAKER_CODE_LEN:               usize = 2;
const OPT_GAME_CODE_LEN:                usize = 4;
const OPT_FIXED_VAL_LEN:                usize = 7;
const OPT_EXPANSION_RAM_LEN:            usize = 1;
const OPT_SPECIAL_VERSION_LEN:          usize = 1;
const OPT_SUB_CART_TYPE_LEN:            usize = 1;
const OPT_HEADER_LEN_BYTES:             usize = 16;

const OPT_MAKER_CODE_INDEX:             usize = 0;
const OPT_GAME_CODE_INDEX:              usize = OPT_MAKER_CODE_LEN;
const OPT_FIXED_VAL_INDEX:              usize = OPT_GAME_CODE_INDEX + OPT_GAME_CODE_LEN;
const OPT_EXPANSION_RAM_INDEX:          usize = OPT_FIXED_VAL_INDEX + OPT_FIXED_VAL_LEN;
const OPT_SPECIAL_VERSION_INDEX:        usize = OPT_EXPANSION_RAM_INDEX + OPT_EXPANSION_RAM_LEN;
const OPT_SUB_CART_TYPE_INDEX:          usize = OPT_SPECIAL_VERSION_INDEX + OPT_SUB_CART_TYPE_LEN;

/// Header Breakdown (32 bytes)
const HDR_TITLE_LEN:                    usize = 21;
const HDR_MAP_MODE_LEN:                 usize = 1;
const HDR_CART_TYPE_LEN:                usize = 1;
const HDR_ROM_SIZE_LEN:                 usize = 1;
const HDR_RAM_SIZE_LEN:                 usize = 1;
const HDR_DEST_CODE_LEN:                usize = 1;  // Used to determine the region.
const HDR_FIXED_VAL_LEN:                usize = 1;  // Used to determine if the optional header is present.
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
const HDR_OPT_PRESENT:                  u8  = 0x33;   // Value in HDR_FIXED_VAL_INDEX if the optional header is present.
const HDR_SUBTYPE_PRESENT:              u8  = 0x00;   // Value in HDR_FIXED_VAL_INDEX if only the chipset subtype is present. 

/// Exception Vector Breakdown (16 bytes)
const _EV_NATIVE_UNUSED_1_LEN:          usize = 4;
const EV_NATIVE_COP_LEN:                usize = 2;
const EV_NATIVE_BRK_LEN:                usize = 2;
const EV_NATIVE_ABORT_LEN:              usize = 2;
const EV_NATIVE_NMI_LEN:                usize = 2;
const EV_NATIVE_UNUSED_2_LEN:          usize = 2;
const EV_NATIVE_IRQ_LEN:                usize = 2;
const EV_EMU_UNUSED_1_LEN:             usize = 4;
const EV_EMU_COP_LEN:                   usize = 2;
const EV_EMU_UNUSED_2_LEN:             usize = 2;
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
const EV_NATIVE_IRQ_INDEX:              usize = EV_NATIVE_NMI_INDEX + EV_NATIVE_UNUSED_2_LEN;
// EMU UNUSED 1
const EV_EMU_COP_INDEX:                 usize = EV_NATIVE_IRQ_INDEX + EV_EMU_UNUSED_2_LEN;
// EMU UNUSED 2
const EV_EMU_ABORT_INDEX:               usize = EV_EMU_COP_INDEX + EV_EMU_UNUSED_2_LEN;
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
const HI_ROM_EXC_VECTOR_ADDR:           usize = HI_ROM_BANK_SIZE_BYTES - EV_LEN_BYTES;
const HI_ROM_HEADER_ADDR:               usize = HI_ROM_EXC_VECTOR_ADDR - HDR_LEN_BYTES;
const HI_ROM_EXT_HEADER_ADDR:           usize = HI_ROM_HEADER_ADDR - OPT_HEADER_LEN_BYTES;

/// ExHiRom specific values
const EX_HI_ROM_BANK_ADDR:              usize = HI_ROM_BANK_ADDR as usize;
const EX_HI_ROM_EXC_VECTOR_ADDR:        usize = (4 * 1024 * 1024) + (HI_ROM_BANK_SIZE_BYTES - EV_LEN_BYTES); // Header starts at the end of the first bank after 4MiB.
const EX_HI_ROM_HEADER_ADDR:            usize = EX_HI_ROM_EXC_VECTOR_ADDR - HDR_LEN_BYTES;
const EX_HI_ROM_EXT_HEADER_ADDR:        usize = EX_HI_ROM_HEADER_ADDR - OPT_HEADER_LEN_BYTES;

/// Map mode values
const MAP_HIROM_MASK:                   u8 = 0b00000001;
const MAP_SA1_MASK:                     u8 = 0b00000010;
const MAP_EXHIROM_MASK:                 u8 = 0b00000100;
// Unused Bit                                0b00001000 
const MAP_FASTROM_MASK:                 u8 = 0b00010000;
const MAP_BASE_MASK:                    u8 = 0b00100000;
/// Public constants
pub const ROM_BASE_ADDR:                u16 = 0x8000;    // All LoRom banks, and mirrored banks of both Lo and HiRom fall under $XX8000. E.G.: Bank 0: $808000, Bank 1: $908000
pub const TOTAL_HDR_BYTES:              usize = OPT_HEADER_LEN_BYTES + HDR_LEN_BYTES + EV_LEN_BYTES;
/********************************* ROM Info Enums & Struct Definitions *********************************/
/// Info pertaining to the memory map and size of the ROM.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum RomSize {
    LoRom   = LO_ROM_HEADER_ADDR,
    HiRom   = HI_ROM_HEADER_ADDR,
    ExHiRom = EX_HI_ROM_HEADER_ADDR
}
const ROM_SIZE_NUM: usize = 3; // You cannot enumerate an enum in rust without an additional library

/// Info pertaining to a rom bank size.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum BankSize {
    Lo      = LO_ROM_BANK_SIZE_BYTES,
    Hi      = HI_ROM_BANK_SIZE_BYTES
}

/// Info pertaining to the CPU clock speed the SNES runs at for this ROM.
/// SlowRom = 2.68MHz
/// FastRom = 3.58MHz
#[derive(Debug, Clone, Copy)]
pub enum RomClkSpeed {
    SlowRom,    // 2.68MHz
    FastRom     // 3.58MHz
}

/// ROM Expansion chips which are noted and affect memory map.
/// Note that SuperFX and others have their own memory map which is not covered by this.
#[derive(Debug, Clone, Copy)]
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
    SDD1               = 0x03,
    SRTC               = 0x04,
    Other               = 0x05,
    Custom              = 0xFF
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CustomCoProcessor {
    SPC7110             = 0x00,
    ST010_11            = 0x01,
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
    mem_map: RomSize
}

impl RomData{
    /// Return an empty HeaderData struct.
    pub fn new() -> Self {
        Self {
            header: [0; HDR_LEN_BYTES],
            opt_header: [0; OPT_HEADER_LEN_BYTES],
            exception_vectors: [0; EV_LEN_BYTES],
            opt_is_present: false,
            mem_map: RomSize::LoRom
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
pub fn load_rom(path: PathBuf, mut memory: &memory::Memory) -> Result<RomData, RomReadError> {
    // Attempt to read to buffer.
    let rom = read_rom_to_buf(path)?;
    let mut data = fetch_header(&rom)?;
    let customProc = fetch_opt_header(&rom, &mut data);
    fetch_exception_vectors(&rom, &mut data);

    Ok(data)
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
        return Err(RomReadError::new(
            format!("SMC files contain additional data that is unneeded.\nThis tool does not support them yet. Please use an SFC file.").to_string()
        ));
    }

    // Attempt to open the file, and load it into a buffer.
    let file = fs::File::open(&path);
    
    if file.is_ok()
    {
        let mut buf: Vec<u8> = vec![];
        let _read_result = match file.unwrap().read_to_end(&mut buf) {
            Ok(_) => { retval = Ok(buf); },
            Err(e) => {
                retval = Err(RomReadError::new(format!("Failed to read file: {}", e)));
            },
        };
    }
    else{
        retval = Err(RomReadError::new(
            format!("Failed to open file at {}", &path.display())
        ));
    }

    // If the file was read successfully, operate on it.


    retval
}

/// Actually place the rom into `memory`.
/// https://snes.nesdev.org/wiki/Memory_map
/// 
/// # Parameters:
///     - `rom`:        Pointer to the ROM to read.
///     - `mem_map`:    Size of the rom, used to where to place the ROM.
///     - `mem_ptr`:    Pointer to memory to populate.
/// # Returns:
///     - `Ok()`:           If file wrote successfully,
///     - `RomReadError`:   If an error ocurred in the process.
fn write_rom_to_memory(rom: &Vec<u8>, mem_map: RomSize, mem_ptr: &mut memory::Memory) -> Result<(), RomReadError> {
    let num_banks: usize;
    let bank_size: BankSize;
    let base_addr: usize;
    match mem_map {
        RomSize::ExHiRom => {
            // Populate 0xC00000 - 0xFFFFFF, then wrap around and populate 0x3E8000 - 0x7DFFFF
            // TODO: this is largely a special case and is not that pressing.
            return write_ex_hi_rom(&rom, mem_ptr);
        }
        RomSize::HiRom => {
            num_banks = rom.capacity() / HI_ROM_BANK_SIZE_BYTES;
            bank_size = BankSize::Hi;
            base_addr = compose_address(HI_ROM_BANK_ADDR as u8, 0);
        }
        RomSize::LoRom => {
            // Populate 0x808000 - 0xFF8000, then mirror to 0x008000 - 0x7DFFFF
            num_banks = rom.capacity() / LO_ROM_BANK_SIZE_BYTES;
            bank_size = BankSize::Lo;
            base_addr = compose_address(LO_ROM_BANK_ADDR as u8, ROM_BASE_ADDR);
        }
    }

    let mut write_result: Result<(), RomReadError> = Ok(());
    for bank in 0 .. num_banks {
        let offset: usize = bank * bank_size as usize;
        match mem_ptr.put_bank(bank_size, base_addr + offset, &rom[offset .. offset + bank_size as usize - 1]) {
            Ok(_t) => write_result = Ok(()),
            Err(e) => write_result = Err(RomReadError::new(e.to_string()))
        }
    }
    return write_result;
} 

/// Write rom mirror to correct location.
/// https://snes.nesdev.org/wiki/Memory_map
/// 
/// # Parameters:
///     - `rom`:        Rom to read data from.
///     - `mem_map`:    Type of rom to write mirror for.
///     - `mem_ptr`:    Memory to populate.
/// # Returns:
///     - `Ok()`:           If written successfully,
///     - `RomReadError`:   If an error was encountered in the process.
fn write_rom_mirror(rom: &Vec<u8>, mem_map: RomSize, mem_ptr: &mut memory::Memory) -> Result<(), RomReadError> {
    let bank_clusters: Vec<usize>;
    let base_addrs: Vec<usize>;
    match mem_map {
        RomSize::ExHiRom => return write_ex_hi_rom_mirror(rom, mem_ptr),
        RomSize::HiRom => {
            // HiRom gets split across 2 areas of half-size banks.
            bank_clusters = vec![rom.capacity() / HI_ROM_BANK_SIZE_BYTES, rom.capacity() / HI_ROM_BANK_SIZE_BYTES];
            base_addrs = vec![memory::MEMORY_START, memory::compose_address(LO_ROM_BANK_ADDR, ROM_BASE_ADDR)]
        }
        RomSize::LoRom => {
            bank_clusters = vec![rom.capacity() / LO_ROM_BANK_SIZE_BYTES];
            base_addrs = vec![memory::MEMORY_START];
        }
    }

    let mut write_result: Result<(), RomReadError> = Ok(());
    
    // Between each bank, the location in memory may move, but the rom remains contiguous,
    //      So we need to keep track of where it is.
    let mut rom_cluster_offset: usize = 0;
    for cluster in 0..bank_clusters.len() {
        for bank in 0..bank_clusters[cluster] {
            // The ROM offset is 
            let mem_offset: usize = bank * LO_ROM_BANK_SIZE_BYTES as usize;
            let rom_offset: usize = (bank * LO_ROM_BANK_SIZE_BYTES as usize) + (rom_cluster_offset * LO_ROM_BANK_SIZE_BYTES as usize);
            match mem_ptr.put_bank(BankSize::Lo, base_addrs[cluster] + mem_offset, &rom[rom_offset .. rom_offset + LO_ROM_BANK_SIZE_BYTES - 1]) {
                Ok(_t) => write_result = Ok(()),
                Err(e) => write_result = Err(RomReadError::new(e.to_string()))
            }
        }

        rom_cluster_offset += bank_clusters[cluster];
    }

    return write_result;
}

/// Write an exhirom to memory.
/// https://snes.nesdev.org/wiki/Memory_map#ExHiROM
/// 
/// # Parameters:
///     - `rom`:        Rom to read data from.
///     - `mem_ptr`:    Memory to modify.
/// # Returns:
///     - `Ok()`:           If written Ok.
///     - `RomReadError`:   If process failed.
fn write_ex_hi_rom(rom: &Vec<u8>, mem_ptr: &mut memory::Memory) -> Result<(), RomReadError> {
    return Err(RomReadError::new("Unimplemented for ExHiRom".to_string()));
}

/// Write an exhirom mirror into memory.
/// https://snes.nesdev.org/wiki/Memory_map#ExHiROM
/// 
/// # Parameters:
///     - `rom`:        Rom to read data from.
///     - `mem_ptr`:    Memory to modify.
/// # Returns:
///     - `Ok()`:           If written Ok.
///     - `RomReadError`:   If process failed.
fn write_ex_hi_rom_mirror(rom: &Vec<u8>, mem_ptr: &mut memory::Memory) -> Result<(), RomReadError> {
    return Err(RomReadError::new("Unimplemented for ExHiRom".to_string()));
}

/// Find and grab the header from target rom if available.
/// # Parameters: 
///     - `rom`:    Pointer to rom data to analyze.
/// # Returns:
///     - `Header`:      The header for this rom, if found.
///     - `RomReadError` If the header was unparseable.
fn fetch_header(rom: &Vec<u8>) -> Result<RomData, RomReadError> {
    let mut retval: Result<RomData, RomReadError> = Err(RomReadError::new("".to_string()));
    let mut test_header: Header = [0; HDR_LEN_BYTES];

    // Sum all bytes in the file. overflow is fine.
    let mut checksum: Wrapping<u16> = Wrapping(0);
    let _ = rom.iter().map(|x| checksum += Wrapping(*x as u16));
    

    const ROM_OPTIONS: [RomSize; ROM_SIZE_NUM] = [RomSize::ExHiRom, RomSize::HiRom, RomSize::LoRom];
    for size in ROM_OPTIONS.iter() {
        test_header.clone_from_slice(&rom[*size as usize .. *size as usize + HDR_LEN_BYTES]);

        match test_checksum(checksum.0, &test_header) {
            Ok(tested_size) => {
                return Ok(RomData {
                    header: test_header,
                    opt_header: [0; OPT_HEADER_LEN_BYTES],
                    exception_vectors: [0; EV_LEN_BYTES],
                    opt_is_present: false,
                    mem_map: tested_size
                });
            }
            Err(e) => {
                retval = Err(e);
            }
        }
    }
    return retval;
}

/// Take a RomData object, see if this rom has an optional header, and if so, populate those values.
/// # Parameters:
///     - `rom`:        A Rom to pull data from.
///     - `data`:       A RomData object.
/// # Returns:
///     - `Some(CustomCoProcessor)`:    If the rom indicates coprocessor but not complete.
///     - `None`:                       Otherwise.
fn fetch_opt_header(rom: &Vec<u8>, data: &mut RomData) -> Option<CustomCoProcessor>{
    let header_addr: usize;
    match &data.mem_map {
        RomSize::LoRom => header_addr = LO_ROM_EXT_HEADER_ADDR,
        RomSize::HiRom => header_addr = HI_ROM_EXT_HEADER_ADDR,
        RomSize::ExHiRom => header_addr = EX_HI_ROM_EXT_HEADER_ADDR
    }

    if data.header[HDR_FIXED_VAL_INDEX] == HDR_OPT_PRESENT {
        data.opt_is_present = true;
        data.opt_header.clone_from_slice(
            &rom[header_addr .. header_addr + OPT_HEADER_LEN_BYTES]
        );
    }
    else if data.header[HDR_FIXED_VAL_INDEX] == HDR_SUBTYPE_PRESENT {
        return match &rom[header_addr + OPT_SUB_CART_TYPE_INDEX] {
            0x00 => Some(CustomCoProcessor::SPC7110),
            0x01 => Some(CustomCoProcessor::ST010_11),
            0x02 => Some(CustomCoProcessor::ST018),
            0x03 => Some(CustomCoProcessor::CX4),
            _    => None
        };
    }
    
    return None;
}

/// Fetch the exception vector table from a rom.
/// # Parameters:
///     - `rom`:    Rom to read from.
///     - `data`:   Pointer to RomData to populate.
fn fetch_exception_vectors(rom: &Vec<u8>, data: &mut RomData) {
    let header_addr: usize;
    match &data.mem_map {
        RomSize::LoRom => header_addr = LO_ROM_EXC_VECTOR_ADDR,
        RomSize::HiRom => header_addr = HI_ROM_EXC_VECTOR_ADDR,
        RomSize::ExHiRom => header_addr = EX_HI_ROM_EXC_VECTOR_ADDR
    }
    data.exception_vectors.clone_from_slice(&rom[header_addr .. header_addr + EV_LEN_BYTES]);
}

/// Test if the checksum for this file is valid, and if so, check the map byte and return the result.
/// # Parameters:
///     - `checksum`:   u16 sum of all bytes in the file, with overflow discarded.
///     - `header`:     The header to analyze.
/// # Returns:
///     - `RomSize`:        If the ROM checksum was valid,
///     - `RomReadError`:   If the ROM checksum was invalid, with both the calculated and internal values printed.
fn test_checksum(checksum: u16, header: &Header) -> Result<RomSize, RomReadError> {
    let test_checksum: u16 = u16::from_le_bytes([
        header[HDR_CHECKSUM_INDEX],
        header[HDR_CHECKSUM_INDEX+1]
    ]);
    let test_compare:  u16 = u16::from_le_bytes([
        header[HDR_COMPLEMENT_CHECK_INDEX], 
        header[HDR_COMPLEMENT_CHECK_INDEX + 1]
    ]);

    
    let mut retval: Result<RomSize, RomReadError> = Err(RomReadError::new(
        format!("ROM Checksum was invalid.\nCalculated Checksum: {:#06X}\nROM Checksum: {:#06X}", checksum, test_checksum)
    ));
    if checksum == test_checksum && 
        ((Wrapping(checksum) + Wrapping(test_compare)).0 == HDR_TEST_VALUE) {
        // This rom looks good. Get the size of the rom and throw it back out.
        let rom_map_mode: u8 = header[HDR_MAP_MODE_INDEX];
        if (rom_map_mode & MAP_EXHIROM_MASK) != 0 {
            retval = Ok(RomSize::ExHiRom);
        }
        else if (rom_map_mode & MAP_HIROM_MASK) != 0 {
            retval = Ok(RomSize::HiRom);
        }
        else if (rom_map_mode & MAP_BASE_MASK) != 0 {
            retval = Ok(RomSize::LoRom);
        }
        else { 
            retval = Err(RomReadError::new(
                "ROM Checksum was valid, but mapping mode was unreadable".to_string()
            ));
        }
    }

    return retval;
}

/********************************* ROM Info Tests ******************************************************/
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, RngCore};

    const LOROM_VALUE: u8   = 0x20;
    const HIROM_VALUE: u8   = 0x21;
    const EXHIROM_VALUE: u8 = 0x25;
    const INVALID_MAP_VALUE: u8 = 0xC0;

    mod lorom_tests {
        use super::*;
        #[test]
        fn test_lorom_checksum() {
            assert_eq!(RomSize::LoRom, test_checksum_result(RomSize::LoRom).unwrap());
        }

        #[test]
        fn test_fetch_optional_lorom() {
            test_fetch_optional_header(RomSize::LoRom);
        }

        #[test]
        fn test_fetch_exception_headers_lorom() {
            test_fetch_exception_headers(RomSize::LoRom);
        }
    }

    mod hirom_tests {
        use super::*;

        #[test]
        fn test_hirom_checksum()  {
            assert_eq!(RomSize::HiRom, test_checksum_result(RomSize::HiRom).unwrap());
        }

        #[test]
        fn test_fetch_optional_hirom() {
            test_fetch_optional_header(RomSize::HiRom);
        }

        #[test]
        fn test_fetch_exception_headers_hirom() {
            test_fetch_exception_headers(RomSize::HiRom);
        }
    }

    mod exhirom_tests {
        use super::*;

        #[test]
        fn test_exhirom_checksum() {
            assert_eq!(RomSize::ExHiRom, test_checksum_result(RomSize::ExHiRom).unwrap());
        }

        #[test]
        fn test_fetch_optional_exhirom() {
            test_fetch_optional_header(RomSize::ExHiRom);
        }

        #[test]
        fn test_fetch_exception_headers_exhirom() {
            test_fetch_exception_headers(RomSize::ExHiRom);
        }
    }

    #[test]
    #[should_panic]
    /// Test if a header with a good checksum, but a bad memory mapping value fails.
    fn test_valid_checksum_with_bad_map() {
        // Fill up a random array.
        let mut test_header: Header = rand::thread_rng().gen();
        let mut checksum: u16 = 0;
        
        // Test LoRom Detection.
        test_header[HDR_MAP_MODE_INDEX] = INVALID_MAP_VALUE;
        for byte in test_header.iter() {
            checksum += *byte as u16;
        }
        let compare_value: u16 = HDR_TEST_VALUE - checksum;
        test_header[HDR_CHECKSUM_INDEX] = checksum.to_le_bytes()[0];
        test_header[HDR_CHECKSUM_INDEX+1] = checksum.to_le_bytes()[1];
        test_header[HDR_COMPLEMENT_CHECK_INDEX] = compare_value.to_le_bytes()[0];
        test_header[HDR_COMPLEMENT_CHECK_INDEX + 1] = compare_value.to_le_bytes()[1];

        test_checksum(checksum, &test_header).unwrap();
    }

    /// Provided a target size, construct a header with a valid checksum for that value, and return the outcome.
    /// # Parameters:
    ///     - `expected_result`: Type of ROM to test.
    /// # Returns:
    ///     - `Ok(RomSize)`:     Matching ROM size to expected_result if test is OK,
    ///     - `Err(RomReadErr)`: If `test_checksum()` is broken.
    fn test_checksum_result(expected_result: RomSize) -> Result<RomSize, RomReadError> {
        // Generate a randomized header.
        let mut test_header: Header = rand::thread_rng().gen();
        let mut checksum: u16 = 0;


        // Set the map value to match the expected result.
        let header_map_value: u8;
        match expected_result {
            RomSize::LoRom => header_map_value = LOROM_VALUE,
            RomSize::HiRom => header_map_value = HIROM_VALUE,
            RomSize::ExHiRom => header_map_value = EXHIROM_VALUE
        }
        test_header[HDR_MAP_MODE_INDEX] = header_map_value;

        // Calculate the checksum and complement value.
        for byte in test_header.iter() {
            checksum += *byte as u16;
        }
        let compare_value: u16 = HDR_TEST_VALUE - checksum;
        test_header[HDR_COMPLEMENT_CHECK_INDEX] = compare_value.to_le_bytes()[0];
        test_header[HDR_COMPLEMENT_CHECK_INDEX + 1] = compare_value.to_le_bytes()[1];
        test_header[HDR_CHECKSUM_INDEX] = checksum.to_le_bytes()[0];
        test_header[HDR_CHECKSUM_INDEX + 1] = checksum.to_le_bytes()[1];

        test_checksum(checksum, &test_header)
    }

    #[test]
    #[should_panic]
    /// Test if a header with a bad checksum fails.
    fn test_invalid_checksum() {
        // Fill up a random array.
        let mut test_header: Header = rand::thread_rng().gen();
        let mut checksum: u16 = 0;
        
        test_header[HDR_MAP_MODE_INDEX] = LOROM_VALUE;
        for byte in test_header.iter() {
            checksum += *byte as u16;
        }
        checksum += 1;
        test_checksum(checksum, &test_header).unwrap();
    }

    fn test_fetch_optional_header(expected_type: RomSize){
        // This will generate a huge ExHiRom (4 MiB + 64KiB) and take a while.
        let mut test_rom: Box<[u8; EX_HI_ROM_EXC_VECTOR_ADDR + EV_LEN_BYTES]> = vec![0; EX_HI_ROM_EXC_VECTOR_ADDR + EV_LEN_BYTES].into_boxed_slice().try_into().unwrap();
        rand::thread_rng().fill_bytes(&mut *test_rom);
        let mut data: RomData = RomData::new();
        data.mem_map = expected_type;
        data.header[HDR_FIXED_VAL_INDEX] = HDR_OPT_PRESENT; 

        let header_location: usize;
        match expected_type {
            RomSize::LoRom => header_location = LO_ROM_EXT_HEADER_ADDR,
            RomSize::HiRom => header_location = HI_ROM_EXT_HEADER_ADDR,
            RomSize::ExHiRom => header_location = EX_HI_ROM_EXT_HEADER_ADDR
        }

        fetch_opt_header(&test_rom.to_vec(), &mut data);

        for byte in 0..OPT_HEADER_LEN_BYTES {
            assert_eq!(test_rom[header_location + byte], data.opt_header[byte]);
        }
    }

    fn test_fetch_exception_headers(expected_type: RomSize) {
        // This will generate a huge ExHiRom (4 MiB + 64KiB) and take a while.
        let mut test_rom: Box<[u8; EX_HI_ROM_EXC_VECTOR_ADDR + EV_LEN_BYTES]> = vec![0; EX_HI_ROM_EXC_VECTOR_ADDR + EV_LEN_BYTES].into_boxed_slice().try_into().unwrap();
        rand::thread_rng().fill_bytes(&mut *test_rom);
        let mut data: RomData = RomData::new();
        data.mem_map = expected_type;

        let header_location: usize;
        match expected_type {
            RomSize::LoRom => header_location = LO_ROM_EXC_VECTOR_ADDR,
            RomSize::HiRom => header_location = HI_ROM_EXC_VECTOR_ADDR,
            RomSize::ExHiRom => header_location = EX_HI_ROM_EXC_VECTOR_ADDR
        }

        fetch_exception_vectors(&test_rom.to_vec(), &mut data);

        for byte in 0..OPT_HEADER_LEN_BYTES {
            assert_eq!(test_rom[header_location + byte], data.exception_vectors[byte]);
        }
    }

    #[test]
    fn test_fetch_optional_coprocessors() {
        // This will generate a huge ExHiRom (4 MiB + 64KiB) and take a while.
        let mut test_rom: Box<[u8; LO_ROM_BANK_SIZE_BYTES]> = vec![0; LO_ROM_BANK_SIZE_BYTES].into_boxed_slice().try_into().unwrap();
        rand::thread_rng().fill_bytes(&mut *test_rom);
        let mut data: RomData = RomData::new();
        data.header[HDR_FIXED_VAL_INDEX] = HDR_SUBTYPE_PRESENT;

        for i in 0x00..=0x05 {
            test_rom[LO_ROM_EXT_HEADER_ADDR + OPT_SUB_CART_TYPE_INDEX] = i as u8;
            match i {
                0x00 => assert_eq!(CustomCoProcessor::SPC7110, fetch_opt_header(&test_rom.to_vec(), &mut data).unwrap()),
                0x01 => assert_eq!(CustomCoProcessor::ST010_11, fetch_opt_header(&test_rom.to_vec(), &mut data).unwrap()),
                0x02 => assert_eq!(CustomCoProcessor::ST018, fetch_opt_header(&test_rom.to_vec(), &mut data).unwrap()),
                0x03 => assert_eq!(CustomCoProcessor::CX4, fetch_opt_header(&test_rom.to_vec(), &mut data).unwrap()),
                _ => assert!(fetch_opt_header(&test_rom.to_vec(), &mut data).is_none())
            }
        } 
    }
}