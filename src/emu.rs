use std::io;
use std::io::Write;
use std::time;

use crate::cpu;
use crate::debugger;
use crate::memory;
use crate::romdata;

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
pub struct VirtualMachine {
    pub cpu: cpu::CpuState,
    pub memory: memory::Memory,
    pub romdata: romdata::RomData,
    pub clocks: ClockState,
    pub is_running: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            cpu: cpu::CpuState::new(),
            memory: memory::Memory::new(),
            romdata: romdata::RomData::new(),
            clocks: ClockState::new(),
            is_running: false,
        }
    }
}

/// Run the system.
/// Also manages timings and delegates to other legs of the system. Might be worth breaking up in the future.
/// # Parameters
///     - `vm`  Object holding CPU state and Memory for this instance.
pub fn run(path: std::path::PathBuf, args: Vec<String>) {
    print!("Opening file {}... ", &path.display());

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
    print!("Success.\n");
    io::stdout().flush().unwrap();

    vm.clocks.clock_speed = match vm.romdata.mode.speed {
        romdata::RomClkSpeed::SlowRom => SLOWROM_CLOCK_CYCLE_TICK_SEC,
        romdata::RomClkSpeed::FastRom => FASTROM_CLOCK_CYCLE_TICK_SEC,
    };

    // If the user wants to use the debugger let it delegate the run loop.
    let debugger_enabled = true;
    if debugger_enabled {
        debugger::run(vm);
    }
    else {
        vm.is_running = true;
        while vm.is_running {
            // TODO: Should CPU be threaded or should this file be the king?
            // TODO: Spin off thread for SPC700(?)
            // TODO: Spin off thread for PPU(?)
            // If the debugger is enabled, allow it to delegate the runtime.
            // If the debugger is not enabled, just run as normal.
            vm.is_running = step_cpu(&mut vm);
        }
    }
}

/// Request the CPU to step one operation, and then pend for the number of cycles it (should) take for those operations to run.
/// # Parameters:
///     - `vm`:         Pointer to VM containing state for the emulator.
/// # Returns:
///     - `vm_running`: Whether the VM is running or has stopped.
pub fn step_cpu(vm: &mut VirtualMachine) -> bool {
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
