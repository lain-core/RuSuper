use super::{utils::HexOperators, VirtualMachine};
use crate::debugger::utils;
use std::process::exit;

/**************************************** Constant Values ***************************************************************/
const NUM_ADDR_BYTES: usize = 3;

/**************************************** File Scope Functions **********************************************************/
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
    vm.is_running = true;
}

pub fn dbg_print(args: Vec<&str>, vm: &mut VirtualMachine) {
    let mut token = args.concat();
    if token.contains("+") {
        dbg_print_offset();
    }
    else {
        match token.to_hex() {
            Ok(address) => dbg_print_absolute(address, vm),
            Err(_e) => {
                println!("Error: {} was an invalid address value.", token);
            }
        }
    }
}

fn dbg_print_absolute(address: usize, vm: &mut VirtualMachine) {
    let byte_value = vm.memory.get_byte(address);
    let word_value = vm.memory.get_word(address);

    if byte_value.is_ok() {
        print!(
            "{:#08X}: {:#04X} ",
            address,
            byte_value.expect("Value was error despite checking")
        );
    }
    else {
        print!("Byte Value: Invalid ")
    }

    if word_value.is_ok() {
        print!(
            "Word Value: {:#04X} ",
            word_value.expect("Word value was error despite checking")
        );
    }
    else {
        print!("Word Value: Invalid");
    }

    print!("\n");
}

fn dbg_print_offset() {
    println!("Unimplemented");
}

/**************************************** Tests *************************************************************************/
