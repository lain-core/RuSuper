mod breakpoints;
mod misc;
mod parser;
mod parser_data;
mod step;
mod utils;

use crate::emu::{self, VirtualMachine};
use std::{
    fmt,
    fmt::Debug,
    io::{self, Write},
};

use self::{breakpoints::BreakpointData, step::StepData};
/**************************************** Struct and Type definitions ***************************************************/

/// Error to generate when a bad argument is passed to the debugger.
#[derive(Debug, Clone)]
pub struct InvalidDbgArgError {
    value: String,
}

impl fmt::Display for InvalidDbgArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for InvalidDbgArgError {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string().clone(),
        }
    }
}

impl From<String> for InvalidDbgArgError {
    fn from(value: String) -> Self {
        Self {
            value: value.clone(),
        }
    }
}

/// Struct to track the operation of the debugger.
/// breakpoints: list of breakpoint addresses to stop at,
struct DebuggerState {
    breakpoint_state: BreakpointData,
    step_state: StepData,
    // watch_state: WatchData,
    // etc.
}

impl DebuggerState {
    pub fn new() -> Self {
        Self {
            breakpoint_state: BreakpointData::new(),
            step_state: StepData::new(),
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

            //            "w" => Self::Watch,
            //            "watch" => Self::Watch,
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
struct StepCommand;
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
            DebugCommandTypes::Step => StepCommand.debug_op(args, debug, vm),
            DebugCommandTypes::Dump => todo!(),
            DebugCommandTypes::Print => PrintCommand.debug_op(args, debug, vm),
            DebugCommandTypes::Watch => todo!(),
            DebugCommandTypes::Exit => ExitCommand.debug_op(args, debug, vm),
            DebugCommandTypes::Invalid => InvalidCommand.debug_op(args, debug, vm),
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
    let trimmed: Vec<&str> = input_text.split_whitespace().collect();

    if !trimmed.is_empty() {
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
        if vm.is_running && !debugger.step_state.is_stepping {
            if let Some(value) = debugger.breakpoint_state.get(vm.cpu.get_pc()) {
                vm.is_running = false;
                println!("BREAK: Halted at {:#08X}", value);
            } else {
                vm.is_running = emu::step_cpu(&mut vm);
            }
        }
        // If the debugger is running the VM by stepping for N steps, check for how many steps are remaining.
        else if debugger.step_state.is_stepping {
            vm.is_running = emu::step_cpu(&mut vm);
            debugger.step_state.steps_to_run -= 1;

            // Stop when we are finished running
            if debugger.step_state.steps_to_run == 0 {
                debugger.step_state.is_stepping = false;
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
