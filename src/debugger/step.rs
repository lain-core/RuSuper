use super::*;

/**************************************** Struct and Type definitions ***************************************************/
pub struct StepData {
    pub _addresses: parser_data::ParserData,
    pub is_stepping: bool,
    pub steps_to_run: usize,
}

impl StepData {
    pub fn new() -> Self {
        Self {
            _addresses: parser_data::ParserData::new(),
            is_stepping: false,
            steps_to_run: 0,
        }
    }
}

trait StepFn {
    fn step_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError>;
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum StepSubCommandTypes {
    Step,
}

impl From<&str> for StepSubCommandTypes {
    fn from(value: &str) -> Self {
        match value {
            _ => Self::Step,
        }
    }
}

impl StepFn for StepSubCommandTypes {
    fn step_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        match self {
            StepSubCommandTypes::Step => StepOp.step_op(args, debug, vm),
        }
    }
}

struct StepOp;

/**************************************** Subcommand implementations **********************************************************/

/// Map the breakpoint function to the subcommand received from the user.
impl DebugFn for StepCommand {
    fn debug_op(
        &self, args: &[&str], debug: &mut DebuggerState, vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        // If the user asked to step with no argument pass it to step once.
        if args.is_empty() {
            StepSubCommandTypes::Step.step_op(args, debug, vm)
        }
        else {
            let sub = StepSubCommandTypes::from(args[0]);
            match sub {
                StepSubCommandTypes::Step => sub.step_op(args, debug, vm),
                // Commented to suppress warning. Unreachable with only one command.
                // _ => sub.step_op(&args[1..], debug, vm),
            }
        }
    }
}

impl StepFn for StepOp {
    fn step_op(
        &self, _args: &[&str], debug: &mut DebuggerState, _vm: &mut VirtualMachine,
    ) -> Result<(), InvalidDbgArgError> {
        debug.step_state.steps_to_run = 1;
        debug.step_state.is_stepping = true;
        Ok(())
    }
}
