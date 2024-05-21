use super::{
    parser::{str_to_values, InvalidDbgArgError},
    VirtualMachine,
};
use std::process::exit;

/**************************************** Constant Values ***************************************************************/

/**************************************** File Scope Functions **********************************************************/
/// Exits the program.
pub fn dbg_exit(
    _args: Vec<&str>, _debug: &mut super::DebuggerState, _vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    exit(0);
}

pub fn dbg_help(
    _args: Vec<&str>, _debug: &mut super::DebuggerState, _vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    println!("==============================");
    println!("======== RuSuper Help ========\n");
    println!("==============================");
    println!("h, help\n\tOpens this menu");
    println!("exit, quit, q\n\tTerminate the program");
    println!("b $XXXXXX\n\tSets a breakpoint for address $XXXXXX");
    println!("c, r\n\tRun the program until a halt is reached, or a breakpoint is hit");
    Ok(())
}

pub fn dbg_invalid(
    _args: Vec<&str>, _debug: &mut super::DebuggerState, _vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    dbg_help(_args, _debug, _vm)?;
    Err(InvalidDbgArgError::from("Invalid Command."))
}

pub fn dbg_continue(
    _args: Vec<&str>, _debug: &mut super::DebuggerState, vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    vm.is_running = true;
    Ok(())
}

pub fn dbg_print(
    args: Vec<&str>, debug: &mut super::DebuggerState, vm: &mut VirtualMachine,
) -> Result<(), InvalidDbgArgError> {
    match str_to_values(&args, debug, vm) {
        Ok((_, address)) => {
            if let Ok(value) = vm.memory.get_word(address) {
                println!(
                    "{:#08X} Byte Value: {:#04X} Word Value: {:#06X}",
                    address,
                    vm.memory.get_byte(address).expect(""),
                    value
                );
            }
            else {
                return Err(InvalidDbgArgError::from(format!(
                    "{:#08X} is out of range of memory.",
                    address
                )));
            }
        }
        Err(e) => return Err(e),
    }
    Ok(())
}

/// Print the value at an absolute memory address.
/// Parameters:
///     - `address`:    Address to read from.
///     - `_debug`:     Debugger State, unused.
///     - `vm`:         Virtual Machine containing memory to read from.
fn _dbg_print_absolute(address: usize, _debug: &mut super::DebuggerState, vm: &mut VirtualMachine) {
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

/**************************************** Tests *************************************************************************/
