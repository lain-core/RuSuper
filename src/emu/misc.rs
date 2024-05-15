use super::VirtualMachine;
use std::process::exit;

/// Exits the program.
pub fn dbg_exit(_args: Vec<&str>, _vm: &mut VirtualMachine) {
    exit(0);
}

pub fn dbg_help(_args: Vec<&str>, _vm: &mut VirtualMachine) {
    println!("==============================");
    println!("======== RuSuper Help ========\n");
    println!("==============================");
    println!("h, help\n\tOpens this menu");
    println!("exit, quit, q\n\tTerminate the program");
    println!("b $XXXXXX\n\tSets a breakpoint for address $XXXXXX");
    println!("c, r\n\tRun the program until a halt is reached, or a breakpoint is hit");
}

pub fn dbg_invalid(_args: Vec<&str>, _vm: &mut VirtualMachine) {
    dbg_help(_args, _vm);
}

pub fn dbg_continue(_args: Vec<&str>, vm: &mut VirtualMachine) {
    vm.debugger.is_running = true;
}
