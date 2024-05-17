use super::{utils, TokenSeparators, VirtualMachine};

/**
 *  - b             Set breakpoint at current PC
 *      - b +N          Set breakpoint at memory address PC + N
 *      - b $XXXXXX     Set breakpoint at absolute address 0xXXXXXX
 *      - b tag
 *      - b tag+N
 *      - b show        Show breakpoints
 *      - b del X       Delete breakpoint X
 */

/// Acts as the controller for all breakpoint functions.
pub fn dbg_breakpoint(
    args: Vec<TokenSeparators>,
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
