use super::{
    parser::{compute_address_from_args, create_new_tag, DebugTokenStream, TokenStreamHelpers},
    VirtualMachine,
};

/// Acts as the controller for all breakpoint functions.
pub fn dbg_breakpoint(
    args: DebugTokenStream, debug: &mut super::DebuggerState, vm: &mut VirtualMachine,
) {
    // If a tag is left in here, generate a new tag.
    if args.contains_tag() {
        match create_new_tag(args, vm, &mut debug.tags) {
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
        match compute_address_from_args(args, vm){
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
