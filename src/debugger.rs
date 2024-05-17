mod breakpoints;
mod misc;
mod utils;

use crate::emu::{self, VirtualMachine};
use std::{
    collections::HashMap,
    io::{self, Write},
    iter::Map,
};

use self::utils::{collect_args, InvalidValueError};
/**************************************** Struct and Type definitions ***************************************************/

/// Struct to track the operation of the debugger.
struct DebuggerState {
    pub is_stepping: bool,
    pub steps_to_run: usize,
    breakpoints: Vec<usize>,
    watched_vars: Vec<usize>,
    tags: DebuggerTags,
}

impl DebuggerState {
    pub fn new() -> Self {
        Self {
            is_stepping: false,
            steps_to_run: 0,
            watched_vars: Vec::new(),
            breakpoints: Vec::new(),
            tags: DebuggerTags::new(),
        }
    }
}

struct DebuggerTags {
    tags: HashMap<String, usize>,
}
impl DebuggerTags {
    fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Result<usize, InvalidValueError> {
        match self.tags.get(key) {
            Some(value) => Ok(*value),
            None => Err(InvalidValueError::from(format!(
                "Tag {} does not exist!",
                key
            ))),
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

#[derive(Debug, PartialEq, Eq)]
enum TokenSeparators {
    HexValue,
    Offset,
    Value(String),
    Tag(String),
    Invalid,
}

impl From<&str> for TokenSeparators {
    fn from(value: &str) -> Self {
        match value {
            "$" => Self::HexValue,
            "+" => Self::Offset,
            _ => Self::Invalid,
        }
    }
}

type DebugFn = Box<dyn Fn(Vec<TokenSeparators>, &mut DebuggerState, &mut VirtualMachine)>;

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

fn check_dbg_input(
    debug: &mut DebuggerState,
    vm: &mut VirtualMachine,
    debug_cmds: &HashMap<DebugCommandTypes, DebugFn>,
) {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read stdin");
    let trimmed: Vec<&str> = input_text.trim().split_whitespace().collect();

    if trimmed.len() > 0 {
        let command: DebugCommandTypes =
            DebugCommandTypes::from(trimmed[0].to_lowercase().as_ref());
        let mut arguments: Vec<TokenSeparators> = vec![];
        if trimmed.len() > 1 {
            arguments = collect_args(trimmed[1..].concat()).unwrap();
        }
        // args = utils::collect_args(trimmed).unwrap();
        debug_cmds[&command](arguments, debug, vm);
    }
}

pub fn run(mut vm: VirtualMachine) {
    let mut debugger = DebuggerState::new();
    let debug_cmds: HashMap<DebugCommandTypes, DebugFn> = construct_cmd_table();
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
            check_dbg_input(&mut debugger, &mut vm, &debug_cmds);
        }
    }
}

/**************************************** Tests *************************************************************************/
