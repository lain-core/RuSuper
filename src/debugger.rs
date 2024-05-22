mod breakpoints;
mod misc;
mod parser;

use crate::emu::{self, VirtualMachine};
use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, Write},
};

use self::parser::InvalidDbgArgError;
/**************************************** Struct and Type definitions ***************************************************/

pub trait FindKeyInHashMap {
    fn find_key(&self, value: usize) -> Option<&str>;
}

impl FindKeyInHashMap for DebugTagTable {
    /// Given a value, find the first key that matches in a hashmap.
    fn find_key(&self, value: usize) -> Option<&str> {
        self.iter().find_map(|(key, val)| {
            if *val == value {
                Some(key.as_str())
            }
            else {
                None
            }
        })
    }
}

/// Allow deleting a value from any Vector of Equatable value <T>.
pub trait RemoveValueFromVector<T: Eq> {
    fn remove_value(&mut self, value: T);
}

impl<T: Eq> RemoveValueFromVector<T> for Vec<T> {
    /// Delete value from vector wherever found.
    /// # Parameters:
    ///     - `self`: Vector of type T
    ///     - `value`: Value of type T to scan for.
    fn remove_value(&mut self, value: T) {
        let mut del_index: Vec<usize> = vec![];
        for (index, item) in self.into_iter().enumerate() {
            if *item == value {
                del_index.push(index);
            }
        }

        if del_index.len() > 0 {
            for index in del_index {
                self.remove(index);
            }
        }
    }
}


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

trait DebugFn {
    fn debug_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError>;
}

/// Enum representing all of the potential commands for the debugger.
#[derive(Debug, Hash, PartialEq, Eq)]
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

struct HelpCommand;
struct ContinueCommand;
struct PrintCommand;
struct ExitCommand;
struct InvalidCommand;
struct BreakCommand;
struct _StepCommand;
struct _DumpCommand;
struct _WatchCommand;

impl DebugFn for DebugCommandTypes {
    fn debug_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        match self {
            DebugCommandTypes::Help => HelpCommand.debug_op(args, debug, vm),
            DebugCommandTypes::Break => BreakCommand.debug_op(args, debug, vm),
            DebugCommandTypes::Continue => ContinueCommand.debug_op(args, debug, vm),
            DebugCommandTypes::Step => todo!(),
            DebugCommandTypes::Dump => todo!(),
            DebugCommandTypes::Print => PrintCommand.debug_op(args, debug, vm),
            DebugCommandTypes::Watch => todo!(),
            DebugCommandTypes::Exit => ExitCommand.debug_op(args, debug, vm),
            DebugCommandTypes::Invalid => InvalidCommand.debug_op(args, debug, vm),
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

/**************************************** File Scope Functions **********************************************************/

/// Parse the input from the debugger, decode the command, and then call it's associated function.
/// Parameters:
///     - `debug`:      Mutable pointer to the debugger state to utilize,
///     - `vm`:         Pointer to the virtual machine to fetch values from memory or PC.
///     - `debug_cmds`: The assembled table of debugger command->fn pointers.
fn check_dbg_input(debug: &mut DebuggerState, vm: &mut VirtualMachine) {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read stdin");
    input_text = input_text.to_lowercase();
    let trimmed: Vec<&str> = input_text.trim().split_whitespace().collect();

    if trimmed.len() > 0 {
        if let Err(error) = DebugCommandTypes::from(trimmed[0]).debug_op(&trimmed[1..], debug, vm) {
            println!("{}", error);
        }
    }
}

/**************************************** Public Functions **************************************************************/

/// Run the debugger.
/// Runs until `exit` command is received.
/// # Parameters:
///     - `vm`: Mutable Virtual Machine instance to run.
pub fn run(mut vm: VirtualMachine) {
    let mut debugger = DebuggerState::new();

    // Instantiate the table of debugger commands before starting the loop, so we don't churn a ton of memory.
    loop {
        // If the VM is running normally, just continue as usual.
        if vm.is_running && !debugger.is_stepping {
            if debugger.breakpoints.contains(&vm.cpu.get_pc()) {
                vm.is_running = false;
                println!("BREAK: Halted at {:#08X}", &vm.cpu.get_pc());
            }
            else {
                vm.is_running = emu::step_cpu(&mut vm);
            }
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
            check_dbg_input(&mut debugger, &mut vm);
        }
    }
}

/**************************************** Tests *************************************************************************/
