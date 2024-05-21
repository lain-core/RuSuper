use super::{parser::*, *};
use std::collections::HashMap;

/**************************************** Struct and Type definitions ***************************************************/

#[derive(Clone, Hash, PartialEq, Eq)]
enum BreakpointSubCommandTypes {
    Set,
    List,
    Delete,
}

impl From<&str> for BreakpointSubCommandTypes {
    fn from(value: &str) -> Self {
        match value {
            "show" => Self::List,
            "list" => Self::List,
            "l" => Self::List,
            "ls" => Self::List,

            "delete" => Self::Delete,
            "del" => Self::Delete,
            "rm" => Self::Delete,

            _ => Self::Set,
        }
    }
}

type BreakpointFn = Box<
    dyn Fn(Vec<&str>, &mut DebuggerState, &mut VirtualMachine) -> Result<(), InvalidDbgArgError>,
>;

/**************************************** File Scope Functions **********************************************************/

fn construct_breakpoint_table() -> HashMap<BreakpointSubCommandTypes, BreakpointFn> {
    HashMap::from([
        (
            BreakpointSubCommandTypes::List,
            Box::new(dbg_breakpoint_list) as BreakpointFn,
        ),
        (
            BreakpointSubCommandTypes::Delete,
            Box::new(dbg_breakpoint_delete) as BreakpointFn,
        ),
        (
            BreakpointSubCommandTypes::Set,
            Box::new(dbg_breakpoint_set) as BreakpointFn,
        ),
    ])
}

fn dbg_breakpoint_list(
    _args: Vec<&str>, debug: &mut DebuggerState, _vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    print!("\n");
    println!("  Address  | Tag Name  ");
    println!("-----------------------");
    debug.breakpoints.sort();
    for breakpoint in &debug.breakpoints {
        print!("  ");
        print!("{:#08X} |", breakpoint);
        if let Some(name) = debug.tags.find_key(*breakpoint) {
            print!(" {}", name);
        }
        print!("  \n");
    }
    print!("\n");
    Ok(())
}

/// Delete a breakpoint.
/// Parameters:
///     - `args`:   Arguments from the debugger as a Vec of strings.
///     - `debug`:  Pointer to the debugger to remove a breakpoint/tag from.
fn dbg_breakpoint_delete(
    args: Vec<&str>, debug: &mut DebuggerState, vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    let mut cmd_result: Result<(), InvalidDbgArgError> = Ok(());
    let token_args = parser::str_to_args(&args);
    debug.breakpoints.sort();

    match token_args {
        Ok(tokens) => {
            // Remove this value from the breakpoint list.
            match compute_address_from_args(&tokens, debug, vm) {
                Ok(value) => {
                    debug.breakpoints.remove_value(value);
                }
                Err(e) => cmd_result = Err(e),
            }
            // If there are currently tags set, pass through and see if this value matches any.
            if let Some(tags) = tokens.get_tag_names() {
                for tag in tags {
                    debug.tags.remove(&tag);
                }
            }
        }
        Err(e) => cmd_result = Err(e),
    }
    return cmd_result;
}

/// Set a breakpoint.
/// Parameters:
///     - `args`:   Arguments from the debugger as a Vec of strings.
///     - `debug`:  Pointer to the debugger state to add a breakpoint/tag to.
///     - `vm`:     Pointer to the virtual machine.
fn dbg_breakpoint_set(
    args: Vec<&str>, debug: &mut DebuggerState, vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    let mut cmd_result: Result<(), InvalidDbgArgError> = Ok(());
    let token_args = parser::str_to_args(&args).unwrap();
    let test_value = compute_address_from_args(&token_args, debug, vm);

    // If there were no arguments passed just set a breakpoint at the PC if possible
    if args.len() == 0 {
        if debug.breakpoints.contains(&vm.cpu.get_pc()) {
            cmd_result = Err(InvalidDbgArgError::from(format!(
                "Value {:#08X} already exists in breakpoints.",
                &vm.cpu.get_pc()
            )));
        }
        else {
            debug.breakpoints.push(vm.cpu.get_pc());
            println!("Breakpoint created at {:#08X}", vm.cpu.get_pc());
        }
    }
    else {
        // If the value was constructed purely from literals, or it was made of existing tags, throw it on.
        if let Ok(value) = test_value {
            if debug.breakpoints.contains(&value) {
                cmd_result = Err(InvalidDbgArgError::from(format!(
                    "Value {:#08X} already exists in breakpoints.",
                    value
                )));
            }
            else {
                debug.breakpoints.push(value);
                println!("Breakpoint created at {:#08X}", value);
            }
        }
        // Otherwise we need to make a new tag so try to do so.
        else if token_args.contains_tag() {
            match create_new_tag(&token_args, debug, vm) {
                Ok(value) => {
                    debug.breakpoints.push(value);
                    println!("Breakpoint created at {:#08X}", value);
                }
                Err(e) => cmd_result = Err(e),
            }
        }
        else {
            cmd_result = Err(test_value.unwrap_err());
        }
    }
    return cmd_result;
}

/**************************************** Public Functions **************************************************************/

/// Acts as the controller for all breakpoint functions.
pub fn dbg_breakpoint(
    args: Vec<&str>, debug: &mut super::DebuggerState, vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    let breakpoint_fns = construct_breakpoint_table();
    if args.len() > 0 {
        let subcmd = BreakpointSubCommandTypes::from(args[0]);
        match subcmd {
            BreakpointSubCommandTypes::Set => breakpoint_fns[&subcmd](args, debug, vm),
            _ => breakpoint_fns[&subcmd](args[1..].to_vec(), debug, vm),
        }
    }
    else {
        dbg_breakpoint_set(args, debug, vm)
    }
}

/**************************************** Tests *************************************************************************/
