use super::*;

/// Implementation of STP instruction
/// https://undisbeliever.net/snesdev/65816-opcodes.html#stp-stop-the-processor
///
/// # Parameters
///     - `state`   Pointer to modifiable CPU State.
///     - `memory`  Pointer to modifable memory (unused).
///     - `param`   Unused.
///
/// # Returns
///     - None (stop running).
pub(super) fn stp(_arg: &mut CpuInstructionFnArguments) -> Option<u8> { None }

/// Implementation of NOP instruction
/// https://undisbeliever.net/snesdev/65816-opcodes.html#nop-no-operation
///
/// Parameters
///     - `state`   Pointer to modifiable CPU State (unused).
///     - `memory`  Pointer to modifiable memory (unused).
///     - `param`   Unused.
///
/// # Returns
///     - true (continue running).
pub(super) fn nop(_arg: &mut CpuInstructionFnArguments) -> Option<u8> { Some(2) }
