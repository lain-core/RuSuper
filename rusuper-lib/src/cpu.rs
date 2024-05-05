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
}

/* https://wiki.superfamicom.org/65816-reference */
pub enum CpuInstruction {
    ADD { x: u16 },
    NOP,
}

pub fn step(state: &mut CpuState, inst: CpuInstruction) {
    execute(state, inst);
}

fn execute(state: &mut CpuState, inst: CpuInstruction) {
    match inst {
        CpuInstruction::ADD { x } => state.acc += x, // CpuState.acc += inst.x,
        CpuInstruction::NOP => (),
        _ => {
            panic!("Unimplemented instruction found!\n");
        },
    }
}
