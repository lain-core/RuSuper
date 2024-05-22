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

/// Print the value that is stored in a tag or address.
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

/**************************************** Tests *************************************************************************/

//TODO:
// mod tests{
//     use rand::RngCore;

//     use crate::{debugger::DebuggerState, memory::MEMORY_SIZE};

//     use super::*;

//     #[test]
//     fn test_dbg_print() {

//         let mut test_debug = DebuggerState::new();
//         let mut test_vm = VirtualMachine::new();
//         let mut random_data: Box<[u8; MEMORY_SIZE]> =
//             vec![0 as u8; MEMORY_SIZE].into_boxed_slice().try_into().unwrap();
//         rand::thread_rng().fill_bytes(&mut *random_data);
//         for (index, byte) in random_data.iter().enumerate() {
//             test_vm.memory.put_byte(index, *byte).unwrap();
//         }
//     }
// }