use super::{parser::*, *};

/**************************************** Struct and Type definitions ***************************************************/

pub type BreakpointData = parser_data::ParserData;

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
        let token_args = parser::str_to_args(args).unwrap();

        // If there were no arguments passed just set a breakpoint at the PC if possible
        if args.is_empty() {
            cmd_result = debug.breakpoint_state.insert(vm.cpu.get_pc());
        }
        else if let Ok((_, value)) = str_to_values(args, &debug.breakpoint_state, vm) {
            cmd_result = debug.breakpoint_state.insert(value);
        }
        else if token_args.contains_tag() {
            // If the value was constructed purely from literals, or it was made of existing tags, throw it on.
            // Otherwise we need to make a new tag so try to do so.
            let test_tag = create_new_tag(&token_args, &debug.breakpoint_state, vm);
            match test_tag {
                Ok((tagname, value)) => match debug.breakpoint_state.insert_tag(&tagname, value) {
                    Some(value) => {
                        println!("Updated tag {} to address {:#08X}", tagname, value);
                    }
                    None => {
                        println!("Set tag {} at address {:#08X}", tagname, value);
                    }
                },
                Err(e) => {
                    cmd_result = Err(e);
                }
            }
        }

        cmd_result
    }
}

impl BreakpointFn for ListOp {
    fn breakpoint_op(
        &self, _args: &[&str], debug: &mut DebuggerState, _vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        debug.breakpoint_state.display();
        Ok(())
    }
}

impl BreakpointFn for DeleteOp {
    fn breakpoint_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        match parser::str_to_values(args, &debug.breakpoint_state, vm) {
            Ok((tags, address)) => {
                if let Some(_value) = debug.breakpoint_state.get(address) {
                    debug.breakpoint_state.delete(address);
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
                        if debug.breakpoint_state.get_tag(&tag).is_none() {
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
                BreakpointSubCommandTypes::Set
                    .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                    .unwrap();
                assert!(test_debug.breakpoint_state.get(expected_result).is_some());
                test_debug.breakpoint_state = BreakpointData::new();
            }
        }

        #[test]
        fn test_dbg_breakpoint_delete() {
            let test_inputs = testconst::literal_string_args();
            let expected_numerics = testconst::literal_numeric_results();

            let mut test_debug = DebuggerState::new();
            let mut test_vm = VirtualMachine::new();

            for (test_input, expected_result) in zip(test_inputs, expected_numerics) {
                BreakpointSubCommandTypes::Set
                    .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                    .unwrap();
                test_debug.breakpoint_state.delete(expected_result);
                assert!(test_debug.breakpoint_state.get(expected_result).is_none());
            }
        }
    }

    mod breakpoint_tag_tests {
        use std::iter::zip;

        use breakpoints::parser::tests::testconst::{
            TEST_BASE_ADDR, TEST_HEX_VALUE, TEST_TAG_NAME, TEST_TAG_NAME2,
        };

        use self::breakpoints::parser::tests::testconst::{self, tag_string_args};

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
                    println!(
                        "Test input for this iteration is {:?} and expected result is {:#08X}",
                        test_input, result
                    );
                    BreakpointSubCommandTypes::Set
                        .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                        .unwrap();
                    assert!(test_debug.breakpoint_state.get(result).is_some());
                }
                else {
                    assert!(BreakpointSubCommandTypes::Set
                        .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                        .is_err());
                }
                test_debug.breakpoint_state = BreakpointData::new();
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
                    .breakpoint_state
                    .insert_tag(TEST_TAG_NAME, TEST_BASE_ADDR);
                test_debug
                    .breakpoint_state
                    .insert_tag(TEST_TAG_NAME2, TEST_HEX_VALUE);
                if let Some(result) = expected_result {
                    BreakpointSubCommandTypes::Set
                        .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                        .unwrap();
                    assert!(test_debug.breakpoint_state.get(result).is_some());
                }
                else {
                    assert!(BreakpointSubCommandTypes::Set
                        .breakpoint_op(test_input.as_slice(), &mut test_debug, &mut test_vm)
                        .is_err());
                }
                test_debug.breakpoint_state = BreakpointData::new();
            }
        }

        #[test]
        fn test_dbg_breakpoint_delete() {
            let string_vectors = tag_string_args();
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

            let mut test_debug = DebuggerState::new();
            let mut test_vm = VirtualMachine::new();

            for (_test_input, expected_result) in zip(string_vectors, numeric_results) {
                if let Some(result) = expected_result {
                    test_debug.breakpoint_state.insert(result).unwrap();
                    test_debug
                        .breakpoint_state
                        .insert_tag(TEST_TAG_NAME, result);
                    assert!(test_debug.breakpoint_state.get(result).is_some());
                    assert_eq!(
                        test_debug.breakpoint_state.get_tag(TEST_TAG_NAME).unwrap(),
                        &result
                    );
                    BreakpointSubCommandTypes::Delete
                        .breakpoint_op(&[TEST_TAG_NAME], &mut test_debug, &mut test_vm)
                        .unwrap();
                    assert!(test_debug.breakpoint_state.get(result).is_none());
                    assert_eq!(test_debug.breakpoint_state.get_tag(TEST_TAG_NAME), None)
                }
            }
        }
    }

    // #[test]
    // fn test_dbg_breakpoint_list() {

    // }
}
