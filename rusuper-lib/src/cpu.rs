use crate::memory::Memory;

// Instructions mod declarations
mod branch;

#[derive(Debug)]
pub struct CpuState {
    acc: u16,         /* Accumulator TODO: Union this         */
    pc: u16,          /* Program Counter                      */
    sp: u16,          /* Stack Pointer                        */
    flags: u8,        /* Flags TODO: this should be a union of bits */
    direct_page: u16, /* Direct page addressing offset        */
    data_bank: u8,    /* Reference to current data bank addr  */
    prog_bank: u8,    /* Reference to current bank of instr   */
}

/* Associated Functions */
impl CpuState {
    pub const fn new() -> Self {
        Self {
            acc: 0x0000,
            pc: 0x0000,
            sp: 0x0000,
            flags: 0x00,
            direct_page: 0x0000,
            data_bank: 0x00,
            prog_bank: 0x00,
        }
    }

    fn example(&self) -> u16 {
        self.pc
    }

    // We can overload a name too
    fn pc(&self) -> u16 {
        self.pc
    }

    pub fn step(&self, mem: &mut Memory) {
        let next_instruction = self.fetch(&mem);
        let next_decoded_instruction = self.decode(next_instruction);
        self.execute(next_decoded_instruction);
    }

    /// Fetch an instruction from memory.
    /// An instruction can be an 8-bit or a 16-bit one. We will just widen if it is 8-bit.
    /// # Parameters
    ///     - `self`
    ///     - `memory`: Pointer to memory object to read data from.
    /// ## TODO: Who's responsibility is it to discern the next instruction's width?
    fn fetch(&self, memory: &Memory) -> u16 {
        //TODO:
        0x0000
    }
    
    /// Given a byte, perform a decode to determine which logical instruction this data represents.
    /// # Parameters
    ///     - `self`
    ///     - `data`: 16-bit data to decode instruction from.
    /// # Returns
    ///     - Constructed CpuInstruction with the code and parameter data for execute to use.
    fn decode(&self, data: u16) -> CpuInstruction {
        // TODO: 
        CpuInstruction::new()
    }

    fn execute(&self, inst: CpuInstruction) {
        match inst.opcode {
            CpuOpcode::ADD { x } => self.acc += x, // CpuState.acc += inst.x,
            CpuOpcode::NOP => (),
            _ => {
                panic!("Unimplemented instruction found!\n");
            },
        }
    }
}

struct CpuInstruction {
    opcode: CpuOpcode,
    parameters: u16
}

impl CpuInstruction {
    pub const fn new() -> Self {
        Self { opcode: CpuOpcode::NOP, parameters: 0 }
    }

    pub const fn set_opcode(&self, data: u16) {
        match data {
            0x0000 => self.opcode = CpuOpcode::NOP,
            _ => {
                panic!("Target opcode {:#06X} is not implemented or is invalid!", data);
            }
        }
    }
}

/* https://wiki.superfamicom.org/65816-reference */
pub enum CpuOpcode {
    ADD { x: u16 },
    NOP,
}