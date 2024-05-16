mod breakpoints;
mod misc;
mod utils;

use crate::emu::{self, VirtualMachine};
use std::{
    collections::HashMap,
    io::{self, Write},
};
/**************************************** Struct and Type definitions ***************************************************/

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

enum TokenSeparators {
    HexValue,
    Offset,
    Invalid,
}

impl From<&str> for TokenSeparators {
    fn from(value: &str) -> Self {
        match value {
            "$" => Self::HexValue,
            "0x" => Self::HexValue,
            "+" => Self::Offset,
            _ => Self::Invalid,
        }
    }
}

type DebugFn = Box<dyn Fn(Vec<&str>, &mut VirtualMachine)>;

/**************************************** File Scope Functions **********************************************************/
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

/**************************************** Tests *************************************************************************/
