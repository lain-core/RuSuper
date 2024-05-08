/********************************* ROM Info Constants **************************************************/

const HEADER_LENGTH_BYTES:          usize = 32;
const OPT_HEADER_EXTENSION_BYTES:   usize = 16;
const LO_ROM_BANK_SIZE_BYTES:       usize = 32 * 1024; // LoRom, ExLoRom Bank size is 32 KiB
const HI_ROM_BANK_SIZE_BYTES:       usize = 64 * 1024; // HiRom, ExHiRom Bank size is 64 KiB
const LO_ROM_BANK_ADDR:             usize = 0x80;      // LoRom starts at bank $808000 and is mirrored to $008000.
const HI_ROM_BANK_ADDR:             usize = 0xC0;      // HiRom starts at bank $C00000 through to $FFFFFF.
const ROM_BASE_ADDR:                usize = 0x8000;    // All LoRom banks, and mirrored banks of both Lo and HiRom fall under $XX8000. E.G.: Bank 0: $808000, Bank 1: $908000


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

impl From<RomSize> for RomMemoryMap {
    fn from(size: RomSize) -> Self {
        match size {
            // TODO: Populate for low/hi/etc
            _ => {
                RomMemoryMap::new()
            }
        }
    }
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
    header_data: [u8; HEADER_LENGTH_BYTES],
    opt_header_data: [u8; OPT_HEADER_EXTENSION_BYTES],
    opt_is_present: bool,

}

impl HeaderData{
    /// Return an empty HeaderData struct.
    pub fn new() -> Self {
        Self {
            header_data: [0; HEADER_LENGTH_BYTES],
            opt_header_data: [0; OPT_HEADER_EXTENSION_BYTES],
            opt_is_present: false
        }
    }
}

/********************************* ROM Info Functions **************************************************/

/********************************* ROM Info Tests ******************************************************/