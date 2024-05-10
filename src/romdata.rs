// #![allow(unused)]
// There are a lot of currently unused const values in this file, but they are important to structural understanding
//  and may become more useful in the future. This is disabled while still implementing functions.

use core::fmt;
use std::{fmt::Display, fs, io::Read, num::Wrapping, path::PathBuf};

use crate::memory;

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
const HI_ROM_EXC_VECTOR_ADDR:           usize = LO_ROM_BANK_SIZE_BYTES - EV_LEN_BYTES;
const HI_ROM_HEADER_ADDR:               usize = HI_ROM_EXC_VECTOR_ADDR - HDR_LEN_BYTES;
const HI_ROM_EXT_HEADER_ADDR:           usize = HI_ROM_HEADER_ADDR - OPT_HEADER_LEN_BYTES;

/// ExHiRom specific values
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

/********************************* ROM Info Enums & Struct Definitions *********************************/
/// Info pertaining to the memory map and size of the ROM.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RomSize {
    LoRom,
    HiRom,
    ExHiRom
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

#[derive(Debug, Clone, Copy)]
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

/// Find and grab the header from target rom if available.
/// # Parameters: 
///     - `rom`:    Pointer to rom data to analyze.
/// # Returns:
///     - `HeaderData` struct with copies of the Header, Exception Vector, and Optional Header (if present) values.
///     - `RomReadError` if the 
fn fetch_header(rom: &Vec<u8>) -> Result<RomData, RomReadError> {
    let mut retval: Result<RomData, RomReadError> = Err(RomReadError::new("".to_string()));
    let mut data = RomData::new();

    // Sum all bytes in the file. overflow is fine.
    let mut checksum: Wrapping<u16> = Wrapping(0);
    let _ = rom.iter().map(|x| checksum += Wrapping(*x as u16));
    
    let mut checksum_valid: Result<RomSize, RomReadError>;

    // Check the bounds, then excise where a header would be and test for the mapping.
    // TODO: There is a much prettier way to manage this I'm sure.

    // A ROM can only be an ExHiRom if it is > 4MiB + 1 bank in size.
    if rom.capacity() > EX_HI_ROM_EXC_VECTOR_ADDR {
        data.header.clone_from_slice(&rom[EX_HI_ROM_EXT_HEADER_ADDR .. (EX_HI_ROM_EXC_VECTOR_ADDR + EV_LEN_BYTES)]);
        checksum_valid = test_checksum(checksum.0, &data.header);
    }
    // If the ROM is > HI_ROM_BANK_SIZE_BYTES in length, it could be a hirom or a lorom.
    else if rom.capacity() >= HI_ROM_BANK_SIZE_BYTES {
        // Test if this is a HiRom.
        data.header.clone_from_slice(&rom[HI_ROM_EXT_HEADER_ADDR .. (HI_ROM_BANK_SIZE_BYTES - 1)]);
        checksum_valid = test_checksum(checksum.0, &data.header);

        if checksum_valid.is_err() {
            // Test if this is a LoRom.
            data.header.clone_from_slice(&rom[LO_ROM_EXT_HEADER_ADDR .. (LO_ROM_BANK_SIZE_BYTES - 1)]);
            checksum_valid = test_checksum(checksum.0, &data.header);
        }
    }
    // Otherwise it could only possibly be a LoRom.
    else if rom.capacity() >= LO_ROM_BANK_SIZE_BYTES
    {
        // Test if this is a LoRom.
        data.header.clone_from_slice(&rom[LO_ROM_EXT_HEADER_ADDR .. (LO_ROM_BANK_SIZE_BYTES)]);
        checksum_valid = test_checksum(checksum.0, &data.header);
    }
    else {
        checksum_valid = Err(RomReadError{ context: "File was too small to contain header".to_string() });
    }

    // If we got a valid checksum, use the returned size to populate a new RomData object.
    if checksum_valid.is_ok() {
        if data.header[HDR_FIXED_VAL_INDEX] == HDR_OPT_PRESENT {
            data.opt_is_present = true;
            // data.opt_header = fetch_opt_header(&rom, &checksum_valid.unwrap());
        }

        // data.exception_vectors = fetch_exception_vectors(&rom, &checksum_valid.unwrap());        
    }

    return retval;
}

/// Test if the checksum for this file is valid, and if so, check the map byte and return the result.
/// # Parameters:
///     - `checksum`:   u16 sum of all bytes in the file, with overflow discarded.
///     - `header`:     The header to analyze.
/// # Returns:
///     - `RomSize`:        If the ROM checksum was valid,
///     - `RomReadError`:   If the ROM checksum was invalid, with both the calculated and internal values printed.
fn test_checksum(checksum: u16, header: &Header) -> Result<RomSize, RomReadError> {
    let test_checksum: u16 = memory::u16_from_bytes(
        header[HDR_CHECKSUM_INDEX],
        header[HDR_CHECKSUM_INDEX+1]
    );
    let test_compare:  u16 = memory::u16_from_bytes(
        header[HDR_COMPLEMENT_CHECK_INDEX], 
        header[HDR_COMPLEMENT_CHECK_INDEX + 1]
    );

    
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
    use rand::Rng;

    const LOROM_VALUE: u8   = 0x20;
    const HIROM_VALUE: u8   = 0x21;
    const EXHIROM_VALUE: u8 = 0x25;
    const INVALID_MAP_VALUE: u8 = 0xC0;

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
    fn test_lorom_checksum() {
        assert_eq!(RomSize::LoRom, test_checksum_result(RomSize::LoRom).unwrap());
    }

    #[test]
    fn test_hirom_checksum()  {
        assert_eq!(RomSize::HiRom, test_checksum_result(RomSize::HiRom).unwrap());
    }

    #[test]
    fn test_exhirom_checksum() {
        assert_eq!(RomSize::ExHiRom, test_checksum_result(RomSize::ExHiRom).unwrap());
    }

    #[test]
    #[should_panic]
    /// Test if a header with a bad checksum fails.
    fn test_invalid_checksum() {
        // Fill up a random array.
        let mut test_header: Header = rand::thread_rng().gen();
        let mut checksum: u16 = 0;
        
        // Test LoRom Detection.
        test_header[HDR_MAP_MODE_INDEX] = LOROM_VALUE;
        for byte in test_header.iter() {
            checksum += *byte as u16;
        }
        checksum += 1;
        test_checksum(checksum, &test_header).unwrap();
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
}