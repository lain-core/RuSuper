use super::{
    ContinueCommand, DebugFn, ExitCommand, HelpCommand, InvalidCommand, PrintCommand,
    VirtualMachine,
};
use crate::debugger::InvalidDbgArgError;
use std::process::exit;

/**************************************** Constant Values ***************************************************************/

/**************************************** DebugFn Implementations **********************************************************/

impl DebugFn for ExitCommand {
    fn debug_op(
        &self, _args: &[&str], _debug: &mut super::DebuggerState, _vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        exit(0);
    }
}

impl DebugFn for HelpCommand {
    fn debug_op(
        &self, _args: &[&str], _debug: &mut super::DebuggerState, _vm: &mut VirtualMachine,
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
}

impl DebugFn for InvalidCommand {
    fn debug_op(
        &self, args: &[&str], debug: &mut super::DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        HelpCommand::debug_op(&HelpCommand, args, debug, vm)?;
        Err(InvalidDbgArgError::from("Invalid Command."))
    }
}

impl DebugFn for ContinueCommand {
    fn debug_op(
        &self, _args: &[&str], _debug: &mut super::DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        vm.is_running = true;
        Ok(())
    }
}

impl DebugFn for PrintCommand {
    fn debug_op(
        &self, _args: &[&str], _debug: &mut super::DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        // TODO: FIXME: Only operates on the table of breakpoints right now. In the future, this should collate a list of all available tables.
        //        match str_to_values(args, &debug.breakpoint_state, vm) {
        //            Ok((_, address)) => {
        //                if let Ok(value) = vm.memory.get_word(address) {
        //                    println!(
        //                        "{:#08X} Byte Value: {:#04X} Word Value: {:#06X}",
        //                        address,
        //                        vm.memory.get_byte(address).expect(""),
        //                        value
        //                    );
        //                }
        //                else {
        //                    return Err(InvalidDbgArgError::from(format!(
        //                        "{:#08X} is out of range of memory.",
        //                        address
        //                    )));
        //                }
        //            }
        //            Err(e) => return Err(e),
        //        }
        //        Ok(())
        //    }
        vm.memory.print_bytes(None);
        Ok(())
    }
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
