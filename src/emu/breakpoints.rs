use super::VirtualMachine;

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
fn dbg_breakpoint(args: Vec<&str>, vm: &mut VirtualMachine) {
    println!("unimplemented");
}

fn dbg_breakpoint_here(args: Vec<&str>, vm: &mut VirtualMachine) {
    println!("unimplemented");
}

fn dbg_breakpoint_offset(args: Vec<&str>, vm: &mut VirtualMachine) {
    println!("unimplemented");
}

fn dbg_breakpoint_tag(args: Vec<&str>, vm: &mut VirtualMachine) {
    println!("unimplemented");
}

fn dbg_breakpoint_show(args: Vec<&str>, vm: &mut VirtualMachine) {
    println!("unimplemented");
}

fn dbg_breakpoint_remove(args: Vec<&str>, vm: &mut VirtualMachine) {
    println!("unimplemented");
}
