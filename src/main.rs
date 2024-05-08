use std::env;
use std::fs;
use std::path::Path;

mod cpu;
mod memory;
mod romdata;

/// VM Struct which contains the individual pieces of the system.
pub struct VirtualMachine {
    pub cpu: cpu::CpuState,
    pub memory: memory::Memory,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            cpu: cpu::CpuState::new(),
            memory: memory::Memory::new()
        }
    }
}

/// Main function, initializes and runs core.
/// 
/// # Parameters
/// - `file`: Target file to open
pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let path = std::fs::canonicalize(Path::new(&args[1])).expect(
            "File not found"
        );

            // Initialize the VM and then load the ROM into memory.
            let mut vm = VirtualMachine::new();
            romdata::load_rom(path, &mut vm.memory);

            // Start Running.
            cpu::run(vm);
    }
    else {
        println!("You must specify a *.sfc file to run!");
    }
}
