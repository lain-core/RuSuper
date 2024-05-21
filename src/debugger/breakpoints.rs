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

type BreakpointFn = Box<dyn Fn(Vec<&str>, &mut DebuggerState, &mut VirtualMachine)>;

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

fn dbg_breakpoint_list(_args: Vec<&str>, debug: &mut DebuggerState, _vm: &mut VirtualMachine) {
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
}

fn dbg_breakpoint_delete(args: Vec<&str>, debug: &mut DebuggerState, vm: &mut VirtualMachine) {
    let token_args = parser::str_to_args(&args);
    debug.breakpoints.sort();

    if let Ok(tokens) = token_args {
        if let Some(tags) = tokens.get_tag_names() {
            for tag in tags {
                if let Some(value) = debug.tags.remove(&tag) {
                    debug.breakpoints.remove_value(value);
                }
            }
        }
        else if let Ok(value) = compute_address_from_args(&tokens, debug, vm) {
            debug.breakpoints.remove_value(value);
        }
    }
}

fn dbg_breakpoint_set(args: Vec<&str>, debug: &mut DebuggerState, vm: &mut VirtualMachine) {
    let token_args = parser::str_to_args(&args).unwrap();
    let test_value = compute_address_from_args(&token_args, debug, vm);

    if let Ok(value) = test_value {
        println!("value was constructed from literals");
        debug.breakpoints.push(value);
        println!("Breakpoint created at {:#08X}", value);
    }
    else if token_args.contains_tag() {
        match create_new_tag(&token_args, debug, vm) {
            Ok(value) => {
                debug.breakpoints.push(value);
                println!("Breakpoint created at {:#08X}", value);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    else {
        println!("{}", test_value.unwrap_err());
    }
}

/**************************************** Public Functions **************************************************************/

/// Acts as the controller for all breakpoint functions.
pub fn dbg_breakpoint(args: Vec<&str>, debug: &mut super::DebuggerState, vm: &mut VirtualMachine) {
    let breakpoint_fns = construct_breakpoint_table();
    if args.len() > 0 {
        let subcmd = BreakpointSubCommandTypes::from(args[0]);
        match subcmd {
            BreakpointSubCommandTypes::Set => {
                breakpoint_fns[&subcmd](args, debug, vm);
            }
            _ => {
                breakpoint_fns[&subcmd](args[1..].to_vec(), debug, vm);
            }
        }
    }
}

/**************************************** Tests *************************************************************************/
