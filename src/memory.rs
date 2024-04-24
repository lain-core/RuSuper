/* https://en.wikibooks.org/wiki/Super_NES_Programming/SNES_memory_map */
pub struct Memory {
    addr: u8,
}

impl Memory {
    pub const fn empty() -> Self {
        Self { addr: 0 }
    }
}

pub fn load(memory: &mut Memory) {
    memory.addr = 0;
}
