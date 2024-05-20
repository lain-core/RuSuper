use super::{
    parser::{self, DebugTokenStream},
    VirtualMachine,
};

/// Acts as the controller for all breakpoint functions.
pub fn dbg_breakpoint(
    args: DebugTokenStream, debug: &mut super::DebuggerState, vm: &mut VirtualMachine,
) {
    match parser::compute_address_from_args(args, vm) {
        Ok(value) => {
            debug.breakpoints.push(value);
            println!("Breakpoint set at {:#08X}", value);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
