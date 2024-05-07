use std::env;
use std::fs;
use std::path::Path;

mod cpu;
mod memory;

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
        if let Ok(file) = fs::File::open(&path){
            println!("Reading file {}", &path.display());
            vm.memory.load_rom(file);
        }
        else {
            println!("Failed to read file {}", path.display());
        }
    
        // Start Running.
        run(vm);
    }
    else {
        println!("You must specify a *.sfc file to run!");
    }

}

/// Run the core.
/// 
/// # Parameters
///     - `vm`: Virtual Machine in ownership of all states.
fn run(mut vm: VirtualMachine) {
    // TODO: Spin off thread for debugger
    // TODO: Spin off thread for SPC700
    // TODO: Spin off thread for PPU(?)

    // Debugger loop which parses user inputs. 
    let mut vm_running = true;
    loop {
        // Check if the vm is running and step if so.
        // This is not self-contained in a loop because the outside will contain debugger functions in the future.
        if vm_running && (vm.cpu.cycles_to_pend == 0) {
            vm_running = vm.cpu.step(&mut vm.memory);
            println!("Next instruction stalled by {} cycles", vm.cpu.cycles_to_pend);
        }
        else if vm.cpu.cycles_to_pend > 0 {
            vm.cpu.cycles_to_pend -= 1;
        }
        else{
            break;
        }
    }
}