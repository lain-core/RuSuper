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
        println!("  \n-----------------------");
    }
    Ok(())
}

/// Delete a breakpoint.
/// Parameters:
///     - `args`:   Arguments from the debugger as a Vec of strings.
///     - `debug`:  Pointer to the debugger to remove a breakpoint/tag from.
fn dbg_breakpoint_delete(
    args: Vec<&str>, debug: &mut DebuggerState, vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    debug.breakpoints.sort();

    match parser::str_to_values(&args, debug, vm) {
        Ok((tags, address)) => {
            if debug.breakpoints.contains(&address) {
                debug.breakpoints.remove_value(address);
                println!("Deleted {:#08X} from breakpoints", address);
            }
            else {
                return Err(InvalidDbgArgError::from(format!(
                    "Breakpoint {:#08X} does not exist",
                    address
                )));
            }

            if let Some(tags) = tags {
                for tag in tags {
                    if let None = debug.tags.remove(&tag) {
                        return Err(InvalidDbgArgError::from(format!(
                            "Tag {} does not exist.",
                            tag
                        )));
                    }
                    else {
                        println!("Deleted {} from tags", &tag);
                    }
                }
            }
        }
        Err(e) => return Err(e),
    }
    Ok(())
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
    let mut test_value: Result<usize, InvalidDbgArgError> = Err(InvalidDbgArgError::from(""));



    // If there were no arguments passed just set a breakpoint at the PC if possible
    if args.len() == 0 {
        test_value = Ok(vm.cpu.get_pc());
    }
    else if token_args.contains_tag() {
        // If the value was constructed purely from literals, or it was made of existing tags, throw it on.
        // Otherwise we need to make a new tag so try to do so.
        test_value = create_new_tag(&token_args, debug, vm);
    }
    else if let Ok((_, value)) = str_to_values(&args, debug, vm) {
        test_value = Ok(value);
    }

    if let Ok(value) = test_value {
        match debug.breakpoints.contains(&value) {
            true => {
                cmd_result = Err(InvalidDbgArgError::from(format!(
                    "{:#08X} already exists in breakpoints.",
                    value
                )))
            }
            false => {
                debug.breakpoints.push(value);
                println!("Breakpoint created at {:#08X}", value);
            }
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
