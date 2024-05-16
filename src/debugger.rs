mod breakpoints;
mod misc;
mod utils;

use crate::emu::{self, VirtualMachine};
use std::{
    collections::HashMap,
    io::{self, Write},
};

/*******
 * Brainstorming: command ideas:
 *  - h             help
 *  - help          help
 *  - tag $XXXXXX   Assign variable name to address 0xXXXXXX
 *  - b             Set breakpoint at current PC
 *      - b +N          Set breakpoint at memory address PC + N
 *      - b $XXXXXX     Set breakpoint at absolute address 0xXXXXXX
 *      - b tag
 *      - b tag+N
 *      - b show        Show breakpoints
 *      - b del X       Delete breakpoint X
 *  - r             Run until breakpoint or termination
 *      - c             alias for R
 *  - s             Step 1 instruction
 *      - s N           Step N instructions
 *      - s tag         Continue running until target tag is reached.
 *  - w $XXXXXX     Watch value, break on modification at 0xXXXXXX
 *      - w tag         Watch value, break on modification at tag
 *  - pb $XXXXXX  Print byte value at absolute address $XXXXXX
 *      - pb tag      ""
 *      - pw $XXXXXX  Print word value at absolute address $XXXXXX
 *      - pw tag      ""
 *  - dump  Dump current state (all sub-options) to working dir.
 *      - dump loram    Dump memory from 0x000000 - 0x3F1FFF+0x7E0000 - 0x7E1FFF (SNES LoRAM) to loram.bin in working dir.
 *      - dump ppu      Dump memory from 0x002000 - 0x3F3FFF (SNES PPU/APU) to apu.bin in working dir.
 *      - dump controller   Dump Memory from 0x004000 - 0x3F41FF to controller.bin
 *      - dump cpu          Dump memory from 0x004200 - 0x3F5FFF to cpu.bin
 *      - dump expansion    Dump memory from 0x006000 - 0x3F7FFF to expansion.bin
 *      - dump ram          Dump memory from 0x7E0000 - 0x7FFFFF to ram.bin (includes slice of loram)
 *      - dump tags         Dump tags to tags.txt (tags.toml?)
 *      - dump b            Dump breakpoints to breakpoints.txt (breakpoints.toml?). Include tags if possible
 */

/// Struct to track the operation of the debugger.
struct DebuggerState {
    pub is_stepping: bool,
    pub steps_to_run: usize,
    breakpoints: Vec<usize>,
    watched_vars: Vec<usize>,
    tags: HashMap<String, usize>,
    pub debug_cmds: HashMap<DebugCommandTypes, DebugFn>,
}

impl DebuggerState {
    pub fn new() -> Self {
        Self {
            is_stepping: false,
            steps_to_run: 0,
            watched_vars: Vec::new(),
            breakpoints: Vec::new(),
            tags: HashMap::new(),
            debug_cmds: construct_cmd_table(),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum DebugCommandTypes {
    Help,
    Break,
    Continue,
    Step,
    Tag,
    Dump,
    Print,
    Watch,
    Exit,
    Invalid,
}

impl From<&str> for DebugCommandTypes {
    fn from(value: &str) -> Self {
        match value {
            "b" => Self::Break,
            "break" => Self::Break,
            "h" => Self::Help,
            "help" => Self::Help,
            "c" => Self::Continue,
            "r" => Self::Continue,
            "q" => Self::Exit,
            "quit" => Self::Exit,
            "exit" => Self::Exit,
            "p" => Self::Print,
            "print" => Self::Print,
            _ => Self::Invalid,
        }
    }
}

type DebugFn = Box<dyn Fn(Vec<&str>, &mut VirtualMachine)>;

fn construct_cmd_table() -> HashMap<DebugCommandTypes, DebugFn> {
    HashMap::from([
        (DebugCommandTypes::Help, Box::new(misc::dbg_help) as DebugFn),
        (
            DebugCommandTypes::Continue,
            Box::new(misc::dbg_continue) as DebugFn,
        ),
        (
            DebugCommandTypes::Invalid,
            Box::new(misc::dbg_invalid) as DebugFn,
        ),
        (DebugCommandTypes::Exit, Box::new(misc::dbg_exit) as DebugFn),
        (
            DebugCommandTypes::Break,
            Box::new(breakpoints::dbg_breakpoint) as DebugFn,
        ),
        (
            DebugCommandTypes::Print,
            Box::new(misc::dbg_print) as DebugFn,
        ),
    ])
}

fn check_dbg_input(debug: &mut DebuggerState, vm: &mut VirtualMachine) {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read stdin");
    let trimmed: Vec<&str> = input_text.trim().split_whitespace().collect();
    if trimmed.capacity() > 0 {
        let command: DebugCommandTypes = DebugCommandTypes::from(trimmed[0]);
        debug.debug_cmds[&command](trimmed[1..].to_vec(), vm);
    }
}

pub fn run(mut vm: VirtualMachine) {
    let mut debugger = DebuggerState::new();
    loop {
        if vm.is_running && !debugger.is_stepping {
            vm.is_running = emu::step_cpu(&mut vm);
        }
        else if debugger.is_stepping {
            vm.is_running = emu::step_cpu(&mut vm);
            debugger.steps_to_run -= 1;

            // When we are done running for N steps,
            if debugger.steps_to_run == 0 {
                debugger.is_stepping = false;
                vm.is_running = false;
            }
        }
        else {
            print!(">> ");
            io::stdout().flush().unwrap();
            check_dbg_input(&mut debugger, &mut vm);
        }
    }
}
