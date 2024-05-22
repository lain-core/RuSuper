use super::{parser::*, *};

/**************************************** Struct and Type definitions ***************************************************/
trait BreakpointFn {
    fn breakpoint_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError>;
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum BreakpointSubCommandTypes {
    Set,
    List,
    Delete,
}

impl From<&str> for BreakpointSubCommandTypes {
    fn from(value: &str) -> Self {
        match value {
            "show" => Self::List,
            "list" => Self::List,
            "l" => Self::List,
            "ls" => Self::List,

            "delete" => Self::Delete,
            "del" => Self::Delete,
            "rm" => Self::Delete,

            _ => Self::Set,
        }
    }
}

impl BreakpointFn for BreakpointSubCommandTypes {
    fn breakpoint_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        match self {
            BreakpointSubCommandTypes::Set => SetOp.breakpoint_op(args, debug, vm),
            BreakpointSubCommandTypes::List => ListOp.breakpoint_op(args, debug, vm),
            BreakpointSubCommandTypes::Delete => DeleteOp.breakpoint_op(args, debug, vm),
        }
    }
}

struct SetOp;
struct ListOp;
struct DeleteOp;

/**************************************** Subcommand implementations **********************************************************/

impl DebugFn for BreakCommand {
    fn debug_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        let sub = BreakpointSubCommandTypes::from(args[0]);
        match sub {
            BreakpointSubCommandTypes::Set => sub.breakpoint_op(args, debug, vm),
            _ => sub.breakpoint_op(&args[1..], debug, vm),
        }
    }
}


impl BreakpointFn for SetOp {
    fn breakpoint_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        let mut cmd_result: Result<(), InvalidDbgArgError> = Ok(());
        let token_args = parser::str_to_args(&args).unwrap();
        let mut test_value: Result<usize, InvalidDbgArgError> = Err(InvalidDbgArgError::from(""));


        // If there were no arguments passed just set a breakpoint at the PC if possible
        if args.len() == 0 {
            test_value = Ok(vm.cpu.get_pc());
        }
        else if let Ok((_, value)) = str_to_values(&args, debug, vm) {
            test_value = Ok(value);
        }
        else if token_args.contains_tag() {
            // If the value was constructed purely from literals, or it was made of existing tags, throw it on.
            // Otherwise we need to make a new tag so try to do so.
            if cmd_result.is_ok() {
                println!("Creating new tag");
                test_value = create_new_tag(&token_args, debug, vm);
            }
        }

        if let Ok(value) = test_value {
            if cmd_result.is_ok() {
                match debug.breakpoints.contains(&value) {
                    true => {
                        cmd_result = Err(InvalidDbgArgError::from(format!(
                            "{:#08X} already exists in breakpoints.",
                            value
                        )))
                    }
                    false => {
                        debug.breakpoints.push(value);
                        println!("Breakpoint created at {:#08X}", value);
                    }
                }
            }
        }
        else {
            cmd_result = Err(test_value.unwrap_err());
        }

        return cmd_result;
    }
}

impl BreakpointFn for ListOp {
    fn breakpoint_op(
        &self, _args: &[&str], debug: &mut DebuggerState, _vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        print!("\n");
        println!("  Address  | Tag Name  ");
        println!("-----------------------");
        debug.breakpoints.sort();
        for breakpoint in &debug.breakpoints {
            print!("  ");
            print!("{:#08X} |", breakpoint);
            if let Some(name) = debug.tags.find_key(*breakpoint) {
                print!(" {}", name);
            }
            println!("  \n-----------------------");
        }
        Ok(())
    }
}

impl BreakpointFn for DeleteOp {
    fn breakpoint_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        debug.breakpoints.sort();

        match parser::str_to_values(&args, debug, vm) {
            Ok((tags, address)) => {
                if debug.breakpoints.contains(&address) {
                    debug.breakpoints.remove_value(address);
                    println!("Deleted {:#08X} from breakpoints", address);
                }
                else {
                    return Err(InvalidDbgArgError::from(format!(
                        "Breakpoint {:#08X} does not exist",
                        address
                    )));
                }

                if let Some(tags) = tags {
                    for tag in tags {
                        if let None = debug.tags.remove(&tag) {
                            return Err(InvalidDbgArgError::from(format!(
                                "Tag {} does not exist.",
                                tag
                            )));
                        }
                        else {
                            println!("Deleted {} from tags", &tag);
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

/**************************************** Public Functions **************************************************************/

/**************************************** Tests *************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    mod breakpoint_literal_tests {
        use std::iter::zip;

        use parser::tests::testconst;

        use super::*;

        #[test]
        fn test_dbg_breakpoint_set() {
            let test_inputs = testconst::literal_string_args();
            let expected_numerics = testconst::literal_numeric_results();

            let mut test_debug = DebuggerState::new();
            let mut test_vm = VirtualMachine::new();

            for (test_input, expected_result) in zip(test_inputs, expected_numerics) {
                assert_eq!(
                    (),
                    BreakpointSubCommandTypes::Set
                        .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                        .unwrap()
                );
                assert!(test_debug.breakpoints.contains(&expected_result));
                test_debug.breakpoints.clear();
            }
        }

        // #[test]
        // fn test_dbg_breakpoint_delete() {

        // }
    }

    mod breakpoint_tag_tests {
        use std::iter::zip;

        use breakpoints::parser::tests::testconst::{
            TEST_BASE_ADDR, TEST_HEX_VALUE, TEST_TAG_NAME, TEST_TAG_NAME2,
        };

        use self::breakpoints::parser::tests::testconst;

        use super::*;
        #[test]
        fn test_dbg_breakpoint_set() {
            let mut test_debug = DebuggerState::new();
            let mut test_vm = VirtualMachine::new();

            let test_inputs = testconst::tag_string_args();
            let numeric_results: Vec<Option<usize>> = vec![
                Some(0x808000), // tag $808000
                Some(50),       // tag 50
                Some(0x80800A), // tag $808000 + $0A
                Some(0x808032), // tag $808000 + 50
                None,           // tag +$0A
                None,           // tag +50
                /* Value before tag */
                Some(0x808000), // $808000 tag
                Some(50),       // 50 tag
                Some(0x80800A), // $808000 + $0A tag
                Some(0x808032), // $808000 + 50 tag
                None,           // will fail  // $0A + tag
                None,           // will fail  // 50 + tag
                Some(0x80800A), // +$0A tag
                Some(0x808032), // +50 tag
                /* Other */
                Some(0x808000), // tag
                None,           // will fail, // tag + tag
                None,           // will fail, // tag + tag2
                None,           // will fail, // tag + tag2 tag3
                None,           // will fail  // tag3 tag + tag2
            ];

            for (test_input, expected_result) in zip(test_inputs, numeric_results) {
                if let Some(result) = expected_result {
                    assert_eq!(
                        (),
                        BreakpointSubCommandTypes::Set
                            .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                            .unwrap()
                    );
                    assert!(test_debug.breakpoints.contains(&result));
                    test_debug.breakpoints.clear();
                    test_debug.tags.clear();
                }
                else {
                    assert!(BreakpointSubCommandTypes::Set
                        .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                        .is_err());
                }
            }
        }

        #[test]
        fn test_dbg_breakpoint_set_with_existing() {
            let mut test_debug = DebuggerState::new();
            let mut test_vm = VirtualMachine::new();

            let test_inputs = testconst::tag_string_args();
            let numeric_results: Vec<Option<usize>> = vec![
                None,           // tag $808000
                None,           // tag 50
                None,           // tag $808000 + $0A
                None,           // tag $808000 + 50
                Some(0x80800A), // tag +$0A
                Some(0x808032), // tag +50
                /* Value before tag */
                None,           // $808000 tag
                None,           // 50 tag
                None,           // $808000 + $0A tag
                None,           // $808000 + 50 tag
                Some(0x80800A), // $0A + tag
                Some(0x808032), // 50 + tag
                None,           // +$0A tag
                None,           // +50 tag
                /* Other */
                Some(0x808000), // tag
                None,           // tag + tag
                Some(0x80800A), // tag + tag2
                Some(0x80800A), // tag + tag2 tag3
                Some(0x80800A), // tag3 tag + tag2
            ];

            for (test_input, expected_result) in zip(test_inputs, numeric_results) {
                test_debug
                    .tags
                    .insert(TEST_TAG_NAME.to_string(), TEST_BASE_ADDR);
                test_debug
                    .tags
                    .insert(TEST_TAG_NAME2.to_string(), TEST_HEX_VALUE);
                if let Some(result) = expected_result {
                    assert_eq!(
                        (),
                        BreakpointSubCommandTypes::Set
                            .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                            .unwrap()
                    );
                    assert!(test_debug.breakpoints.contains(&result));
                }
                else {
                    assert!(BreakpointSubCommandTypes::Set
                        .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                        .is_err());
                }
                test_debug.breakpoints.clear();
                test_debug.tags.clear();
            }
        }

        // #[test]
        // fn test_dbg_breakpoint_delete() {

        // }
    }

    // #[test]
    // fn test_dbg_breakpoint_list() {

    // }
}
