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
///     - false (stop running).
pub(super) fn stp(state: &mut CpuState, _memory: &mut memory::Memory, _param: u16) -> bool {
    state.cycles_to_pend = 3;
    false
}

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
pub(super) fn nop(state: &mut CpuState, _memory: &mut memory::Memory, _param: u16) -> bool {
    state.cycles_to_pend = 2;
    true
}