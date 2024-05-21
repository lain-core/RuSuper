use std::collections::HashMap;

use super::{
    parser::{self, compute_address_from_args, create_new_tag, DebugTokenStream, TokenStreamHelpers}, DebuggerState, VirtualMachine
};

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

            "delete" => Self::Delete,
            "del" => Self::Delete,

            _ => Self::Set
        }
    }
}

type BreakpointFn = Box<dyn Fn(Vec<&str>, &mut DebuggerState, &mut VirtualMachine)>;

/**************************************** File Scope Functions **********************************************************/

fn construct_breakpoint_table() -> HashMap<BreakpointSubCommandTypes, BreakpointFn> {
    HashMap::from([
        (BreakpointSubCommandTypes::List, Box::new(dbg_breakpoint_list) as BreakpointFn),
        (BreakpointSubCommandTypes::Delete, Box::new(dbg_breakpoint_delete) as BreakpointFn),
        (BreakpointSubCommandTypes::Set, Box::new(dbg_breakpoint_set) as BreakpointFn)
    ])
}

/// Acts as the controller for all breakpoint functions.
pub fn dbg_breakpoint(
    args: Vec<&str>, debug: &mut super::DebuggerState, vm: &mut VirtualMachine,
) {
    let breakpoint_fns = construct_breakpoint_table();
    if args.len() > 0 {
        let subcmd = BreakpointSubCommandTypes::from(args[0]);
        match subcmd {
            BreakpointSubCommandTypes::Set => {
                breakpoint_fns[&subcmd](args, debug, vm);
            },
            _ => {
                breakpoint_fns[&subcmd](args[1..].to_vec(), debug, vm);
            }
        }
    }
}

fn dbg_breakpoint_list(args: Vec<&str>, debug: &mut DebuggerState, vm: &mut VirtualMachine){
    //
}

fn dbg_breakpoint_delete(args: Vec<&str>, debug: &mut DebuggerState, vm: &mut VirtualMachine){

}

fn dbg_breakpoint_set(args: Vec<&str>, debug: &mut DebuggerState, vm: &mut VirtualMachine){
    let token_args = parser::str_to_args(args, debug).unwrap();

    // If a tag is left in here, generate a new tag.
    if token_args.contains_tag() {
        match create_new_tag(token_args, vm, &mut debug.tags) {
            Ok(value) => {
                debug.breakpoints.push(value);
                println!("Breakpoint created at {:#08X}", value);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    else{
        match compute_address_from_args(token_args, vm){
            Ok(value) => {
                debug.breakpoints.push(value);
                println!("Breakpoint created at {:#08X}", value);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

/**************************************** Tests *************************************************************************/