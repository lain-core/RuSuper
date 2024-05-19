use super::{utils, TokenSeparator, VirtualMachine};

/// Acts as the controller for all breakpoint functions.
pub fn dbg_breakpoint(
    args: Vec<TokenSeparator>,
    debug: &mut super::DebuggerState,
    vm: &mut VirtualMachine,
) {
    match utils::compute_address_from_args(args, vm) {
        Ok(value) => {
            debug.breakpoints.push(value);
            println!("Breakpoint set at {:#08X}", value);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
