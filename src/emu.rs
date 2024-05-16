use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::time;

use crate::cpu;
use crate::memory;
use crate::romdata;

mod breakpoints;
mod misc;

/*******
 * Brainstorming: command ideas:
 *  - h             help
 *  - help          help
 *  - tag $XXXXXX   Assign variable name to address 0xXXXXXX
 *  - b             Set breakpoint at current PC
 *      - b +N          Set breakpoint at memory address PC + N
 *      - b $XXXXXX     Set breakpoint at absolute address 0xXXXXXX
 *      - b tag
 *      - b tag+N
 *      - b show        Show breakpoints
 *      - b del X       Delete breakpoint X
 *  - r             Run until breakpoint or termination
 *      - c             alias for R
 *  - s             Step 1 instruction
 *      - s N           Step N instructions
 *      - s tag         Continue running until target tag is reached.
 *  - w $XXXXXX     Watch value, break on modification at 0xXXXXXX
 *      - w tag         Watch value, break on modification at tag
 *  - pb $XXXXXX  Print byte value at absolute address $XXXXXX
 *      - pb tag      ""
 *      - pw $XXXXXX  Print word value at absolute address $XXXXXX
 *      - pw tag      ""
 *  - dump  Dump current state (all sub-options) to working dir.
 *      - dump loram    Dump memory from 0x000000 - 0x3F1FFF+0x7E0000 - 0x7E1FFF (SNES LoRAM) to loram.bin in working dir.
 *      - dump ppu      Dump memory from 0x002000 - 0x3F3FFF (SNES PPU/APU) to apu.bin in working dir.
 *      - dump controller   Dump Memory from 0x004000 - 0x3F41FF to controller.bin
 *      - dump cpu          Dump memory from 0x004200 - 0x3F5FFF to cpu.bin
 *      - dump expansion    Dump memory from 0x006000 - 0x3F7FFF to expansion.bin
 *      - dump ram          Dump memory from 0x7E0000 - 0x7FFFFF to ram.bin (includes slice of loram)
 *      - dump tags         Dump tags to tags.txt (tags.toml?)
 *      - dump b            Dump breakpoints to breakpoints.txt (breakpoints.toml?). Include tags if possible
 */

/***** Timing related constants *****/
// Check if the vm is running and step if so.
// This is not self-contained in a loop because the outside will contain debugger functions in the future.
// The SNES master clock runs at about 21.477MHz NTSC (theoretically 1.89e9/88 Hz).
// The SNES CPU runs at either 2.68MHz or 3.58MHz based on what a rom requests.
// https://wiki.superfamicom.org/timing
const MASTER_CLOCK_CYCLE_TICK_SEC: f64 = 1.0 / (21.477 * 1000.0 * 1000.0);
const SLOWROM_CLOCK_CYCLE_TICK_SEC: f64 = 1.0 / (2.68 * 1000.0 * 1000.0);
const FASTROM_CLOCK_CYCLE_TICK_SEC: f64 = 1.0 / (3.58 * 1000.0 * 1000.0);

/// Number of cycles between draw of scanline.
const CYCLES_PER_SCANLINE: usize = 1364;

/// Every other frame in non-interlaced, 4 less cycles per frame. This is "extra credit".
const NON_INTERLACE_MODE_ALTERNATE_CYCLES_PER: usize = 1360;

/// Struct to track the operation of the debugger.
struct DebuggerState {
    is_running: bool,
    steps_to_run: usize,
    breakpoints: Vec<usize>,
    watched_vars: Vec<usize>,
    tags: HashMap<String, usize>,
}

impl DebuggerState {
    pub fn new() -> Self {
        Self {
            is_running: false,
            steps_to_run: 0,
            watched_vars: Vec::new(),
            breakpoints: Vec::new(),
            tags: HashMap::new(),
        }
    }
}

/// Struct to manage count of clocks.
struct ClockState {
    clock_speed: f64,
    master_clock_cycles_elapsed: usize,
    cpu_clock_cycles_elapsed: usize,
    ppu_clock_cycles_elapsed: usize,
    // TODO: maybe more later.
}

impl ClockState {
    pub fn new() -> Self {
        Self {
            clock_speed: 0.0,
            master_clock_cycles_elapsed: 0,
            cpu_clock_cycles_elapsed: 0,
            ppu_clock_cycles_elapsed: 0,
        }
    }
}

/// VM Struct which contains the individual pieces of the system.
struct VirtualMachine {
    cpu: cpu::CpuState,
    memory: memory::Memory,
    romdata: romdata::RomData,
    clocks: ClockState,
    debugger: DebuggerState,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            cpu: cpu::CpuState::new(),
            memory: memory::Memory::new(),
            romdata: romdata::RomData::new(),
            clocks: ClockState::new(),
            debugger: DebuggerState::new(),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum DebugCommandTypes {
    Help,
    Break,
    Continue,
    Step,
    Tag,
    Dump,
    Print,
    Watch,
    Exit,
    Invalid,
}

impl From<&str> for DebugCommandTypes {
    fn from(value: &str) -> Self {
        match value {
            "b" => Self::Break,
            "break" => Self::Break,
            "h" => Self::Help,
            "help" => Self::Help,
            "c" => Self::Continue,
            "r" => Self::Continue,
            "q" => Self::Exit,
            "quit" => Self::Exit,
            "exit" => Self::Exit,
            "p" => Self::Print,
            "print" => Self::Print,
            _ => Self::Invalid,
        }
    }
}

type DebugFn = Box<dyn Fn(Vec<&str>, &mut VirtualMachine)>;

/// Run the system.
/// Also manages timings and delegates to other legs of the system. Might be worth breaking up in the future.
/// # Parameters
///     - `vm`  Object holding CPU state and Memory for this instance.
pub fn run(path: std::path::PathBuf, args: Vec<String>) {
    print!("Opening file {}... ", &path.display());

    let debug_cmd_table = construct_cmd_table();
    let mut vm = VirtualMachine::new();
    // TODO: find a better way to do this
    if args.len() > 2 {
        if args[2] == "--no-check" {
            vm.romdata = romdata::load_rom(path, &mut vm.memory, true).unwrap();
        }
        else {
            vm.romdata = romdata::load_rom(path, &mut vm.memory, false).unwrap();
        }
    }
    else {
        // Initialize the VM and then load the ROM into memory.
        vm.romdata = romdata::load_rom(path, &mut vm.memory, false).unwrap();
    }

    vm.clocks.clock_speed = match vm.romdata.mode.speed {
        romdata::RomClkSpeed::SlowRom => SLOWROM_CLOCK_CYCLE_TICK_SEC,
        romdata::RomClkSpeed::FastRom => FASTROM_CLOCK_CYCLE_TICK_SEC,
    };
    print!("Success.\n");
    io::stdout().flush().unwrap();
    loop {
        // TODO: Should CPU be threaded or should this file be the king?
        // TODO: Spin off thread for SPC700(?)
        // TODO: Spin off thread for PPU(?)
        if vm.debugger.is_running {
            vm.debugger.is_running = step_cpu(&mut vm);
        }
        else {
            print!(">> ");
            io::stdout().flush().unwrap();
            check_dbg_input(&mut vm, &debug_cmd_table);
        }
    }
}

/// Request the CPU to step one operation, and then pend for the number of cycles it (should) take for those operations to run.
/// # Parameters:
///     - `vm`:         Pointer to VM containing state for the emulator.
/// # Returns:
///     - `vm_running`: Whether the VM is running or has stopped.
fn step_cpu(vm: &mut VirtualMachine) -> bool {
    let mut vm_running = true;
    // If there is no need to pend on another cycle, then go ahead and run an operation.
    if vm.cpu.cycles_to_pend == 0 {
        vm_running = vm.cpu.step(&mut vm.memory);
        println!(
            "Next instruction stalled by {} cycles",
            vm.cpu.cycles_to_pend
        );
    }
    // Otherwise, punt on operating for however long we need to.
    else if vm.cpu.cycles_to_pend > 0 {
        // We have to round because rust does not implement fractional nanoseconds (how unbelievable!!)
        std::thread::sleep(time::Duration::from_secs_f64(vm.clocks.clock_speed));
        vm.clocks.cpu_clock_cycles_elapsed += 1;
        vm.cpu.cycles_to_pend -= 1;
    }
    return vm_running;
}

fn construct_cmd_table() -> HashMap<DebugCommandTypes, DebugFn> {
    HashMap::from([
        (DebugCommandTypes::Help, Box::new(misc::dbg_help) as DebugFn),
        (
            DebugCommandTypes::Continue,
            Box::new(misc::dbg_continue) as DebugFn,
        ),
        (
            DebugCommandTypes::Invalid,
            Box::new(misc::dbg_invalid) as DebugFn,
        ),
        (DebugCommandTypes::Exit, Box::new(misc::dbg_exit) as DebugFn),
        (
            DebugCommandTypes::Break,
            Box::new(breakpoints::dbg_breakpoint) as DebugFn
        ),
        (
            DebugCommandTypes::Print,
            Box::new(misc::dbg_print) as DebugFn
        )
    ])
}

fn check_dbg_input(vm: &mut VirtualMachine, debug_cmd_map: &HashMap<DebugCommandTypes, DebugFn>) {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read stdin");
    let trimmed: Vec<&str> = input_text.trim().split_whitespace().collect();
    if trimmed.capacity() > 0 {
        let command: DebugCommandTypes = DebugCommandTypes::from(trimmed[0]);
        debug_cmd_map[&command](trimmed[1..].to_vec(), vm);
    }
}
