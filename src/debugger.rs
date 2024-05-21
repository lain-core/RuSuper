mod breakpoints;
mod misc;
mod parser;

use crate::emu::{self, VirtualMachine};
use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, Write},
};

use self::parser::{str_to_args, DebugTokenStream, InvalidDbgArgError};
/**************************************** Struct and Type definitions ***************************************************/

pub type DebugTagTable = HashMap<String, usize>;

/// Struct to track the operation of the debugger.
/// is_stepping: if the debugger is running,
/// steps_to_run: steps until next break,
/// breakpoints: list of breakpoint addresses to stop at,
/// watched_vars: variables being watched,
/// tags:   hashmap of tagged addresses as a HashMap<tag_name, tag_address>.
struct DebuggerState {
    pub is_stepping: bool,
    pub steps_to_run: usize,
    breakpoints: Vec<usize>,
    _watched_vars: Vec<usize>,
    tags: DebugTagTable,
}

impl DebuggerState {
    pub fn new() -> Self {
        Self {
            is_stepping: false,
            steps_to_run: 0,
            _watched_vars: Vec::new(),
            breakpoints: Vec::new(),
            tags: HashMap::new(),
        }
    }
}

/// Enum representing all of the potential commands for the debugger.
#[derive(Hash, PartialEq, Eq)]
enum DebugCommandTypes {
    Help,
    Break,
    Continue,
    Step,
    Dump,
    Print,
    Watch,
    Exit,
    Invalid,
}

impl From<&str> for DebugCommandTypes {
    fn from(value: &str) -> Self {
        match value {
            "h" => Self::Help,
            "help" => Self::Help,

            "c" => Self::Continue,
            "r" => Self::Continue,

            "q" => Self::Exit,
            "quit" => Self::Exit,
            "exit" => Self::Exit,

            "b" => Self::Break,
            "break" => Self::Break,

            "p" => Self::Print,
            "print" => Self::Print,

            "d" => Self::Dump,
            "dump" => Self::Dump,

            "s" => Self::Step,
            "step" => Self::Step,

            "w" => Self::Watch,
            "watch" => Self::Watch,

            _ => Self::Invalid,
        }
    }
}

/// Parseable tokens in debugger inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
enum TokenSeparator {
    HexValue,
    Offset,
    Divider,       // Represents general divider character.
    Value(String), // Represents all numeric values (decimal and hex).
    Tag(String),   // Represents all non-value values (tag strings).
    Invalid,
}

impl From<&str> for TokenSeparator {
    fn from(value: &str) -> Self {
        match value {
            "$" => Self::HexValue,
            "+" => Self::Offset,
            " " => Self::Divider,
            _ => Self::Invalid,
        }
    }
}

type DebugFn = Box<dyn Fn(Vec<&str>, &mut DebuggerState, &mut VirtualMachine)>;

/**************************************** File Scope Functions **********************************************************/

/// Construct the hash map of debugger commands.
/// Returns:
///     - HashMap<DebugCommandTypes, DebugFn>:  The mapping of enums to their function pointer.
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
    ])
}

/// Parse the input from the debugger, decode the command, and then call it's associated function.
/// Parameters:
///     - `debug`:      Mutable pointer to the debugger state to utilize,
///     - `vm`:         Pointer to the virtual machine to fetch values from memory or PC.
///     - `debug_cmds`: The assembled table of debugger command->fn pointers.
fn check_dbg_input(
    debug: &mut DebuggerState, vm: &mut VirtualMachine,
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

            // Call the debugger function.
            debug_cmds[&command](trimmed[1..].to_vec(), debug, vm);
    }
}

/// Run the debugger.
/// Runs until `exit` command is received.
/// # Parameters:
///     - `vm`: Mutable Virtual Machine instance to run.
pub fn run(mut vm: VirtualMachine) {
    let mut debugger = DebuggerState::new();

    // Instantiate the table of debugger commands before starting the loop, so we don't churn a ton of memory.
    let debug_cmds: HashMap<DebugCommandTypes, DebugFn> = construct_cmd_table();
    loop {
        // If the VM is running normally, just continue as usual.
        if vm.is_running && !debugger.is_stepping {
            vm.is_running = emu::step_cpu(&mut vm);
        }
        // If the debugger is running the VM by stepping for N steps, check for how many steps are remaining.
        else if debugger.is_stepping {
            vm.is_running = emu::step_cpu(&mut vm);
            debugger.steps_to_run -= 1;

            // Stop when we are finished running
            if debugger.steps_to_run == 0 {
                debugger.is_stepping = false;
                vm.is_running = false;
            }
        }
        // If the VM is not currently running, then prompt the user on the debugger.
        else {
            print!(">> ");
            io::stdout().flush().unwrap();
            check_dbg_input(&mut debugger, &mut vm, &debug_cmds);
        }
    }
}

/**************************************** Tests *************************************************************************/
