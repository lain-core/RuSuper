use super::{
    parser::{create_new_tag, DebugTokenStream, TokenStreamHelpers},
    VirtualMachine,
};

/// Acts as the controller for all breakpoint functions.
pub fn dbg_breakpoint(
    args: DebugTokenStream, debug: &mut super::DebuggerState, _vm: &mut VirtualMachine,
) {
    // If a tag is left in here, generate a new tag.
    if args.contains_tag() {
        create_new_tag(args, &mut debug.tags);
    }
    // match parser::compute_address_from_args(args, vm) {
    //     Ok(value) => {
    //         debug.breakpoints.push(value);
    //         println!("Breakpoint set at {:#08X}", value);
    //     }
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // }
}
