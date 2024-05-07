use crate::memory;

// Instructions mod declarations

// Generalized ASM Help: https://ersanio.gitbook.io/assembly-for-the-snes
// SFC Dev Wiki: https://wiki.superfamicom.org/learning-65816-assembly
// ASAR Docs: https://rpghacker.github.io/asar/asar_2_beta/arch-65816.html
mod branch;
mod misc;

/// Number of bytes to increment the PC by for an instruction.
const PC_INCREMENT_NO_ARG: u16      = 1; // Instruction is only one byte long.
const PC_INCREMENT_SHORT_ARG: u16   = 2; // Instruction takes an 8-bit parameter.
const PC_INCREMENT_LONG_ARG: u16    = 3; // Instruction takes a  16-bit parameter. 

const INST_PARAM_OFFSET: u16        = 1; // Parameter for an offset is always instruction + 1.


#[derive(Debug)]
pub struct CpuState {
    acc: u16,           /* Accumulator TODO: Union this         */
    pc: u16,            /* Program Counter                      */
    sp: u16,            /* Stack Pointer                        */
    flags: u8,          /* Flags TODO: this should be a union of bits */
    direct_page: u16,   /* Direct page addressing offset        */
    data_bank: u8,      /* Reference to current data bank addr  */
    prog_bank: u8,      /* Reference to current bank of instr   */
    pub cycles_to_pend: u8  /* Number of cycles to pend before running next operation. */
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
    /// ## TODO: Who's responsibility is it to discern the next instruction's width?
    fn fetch_and_decode(&self, p_mem: &memory::Memory) -> CpuInstruction {
        let address = memory::compose_address(self.prog_bank, self.pc);
        CpuInstruction::from(p_mem.get_byte(address))
    }

    fn execute(&mut self, inst: CpuInstruction, mut p_mem: &mut memory::Memory) -> bool {
        let continue_run: bool;
        let parameter_location: usize = memory::compose_address(self.prog_bank, self.pc + INST_PARAM_OFFSET);
        let parameter_value: u16;    // Calculated parameter value, if applicable.
        let pc_addr_increment: u16 = inst.width as u16;  // Number of bytes to increment the PC by after this operation.
        
        match inst.width {
            CpuParamWidth::NO       => { parameter_value = 0 },
            CpuParamWidth::SHORT    => { parameter_value = p_mem.get_byte(parameter_location) as u16 },
            CpuParamWidth::LONG     => { parameter_value = p_mem.get_word(parameter_location) }
        }

        println!("Executing {:#?}", inst);

        if let Some(func) = inst.function {
            // FIXME: is this reference to self actually mutable??
            continue_run = func(self, &mut p_mem, parameter_value);
            self.pc += pc_addr_increment;
        }
        else{
            panic!("Attempted to execute an unimplemented instruction!")
        }


        continue_run
    }
}

#[derive(Debug)]
struct CpuInstruction {
    opcode: CpuOpcode,
    width: CpuParamWidth,
    function: Option<fn(&mut CpuState, &mut memory::Memory, u16) -> bool>,
}

impl CpuInstruction {
    pub fn new() -> Self {
        Self { opcode: CpuOpcode::NOP, width: CpuParamWidth::NO, function: None }
    }
}

impl From<u8> for CpuInstruction {
    fn from(value: u8) -> Self {
        let opcode: CpuOpcode;
        let width: CpuParamWidth;
        let function: Option<fn(&mut CpuState, &mut memory::Memory, u16) -> bool>;

        match value {
            // And many more
            0xDB => { opcode = CpuOpcode::STP; width = CpuParamWidth::NO; function = Some(misc::stp)     },
            0xEA => { opcode = CpuOpcode::NOP; width = CpuParamWidth::NO;    function = Some(misc::nop)     },
            _ =>    { opcode = CpuOpcode::NOP; width = CpuParamWidth::NO;    function = None                }
        }
        Self { opcode: opcode, width: width, function: function }
    }
}

/* https://wiki.superfamicom.org/65816-reference */
#[derive(Debug)]
pub enum CpuOpcode {
    STP,
    NOP,
    // Many More
}

/// Defines width of operation. NO = Bare opcode (e.g. NOP). SHORT = 8bit param. LONG = 16bit param.
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
enum CpuParamWidth {
    NO      = PC_INCREMENT_NO_ARG,
    SHORT   = PC_INCREMENT_SHORT_ARG,
    LONG    = PC_INCREMENT_LONG_ARG
}
