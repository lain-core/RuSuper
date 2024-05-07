use std::fs;
use std::path::PathBuf;

mod cpu;
mod memory;

/// VM Struct which contains the individual pieces of the system.
struct VirtualMachine {
    cpu: cpu::CpuState,
    memory: memory::Memory,
}

/// Main function, initializes and runs core.
/// 
/// # Parameters
/// - `file`: Target file to open
pub fn init(path: &mut PathBuf) {
    println!("Initializing library");

    let mut vm =  VirtualMachine {
        cpu: cpu::CpuState::new(),
        memory: memory::Memory::new(),
    };

    if let Ok(file) = fs::File::open(&path){
        println!("Reading file {}", &path.display());
        vm.memory.load_rom(file);
    }
    else {
        println!("Failed to read file {}", path.display());
    }

    // Debugger loop which parses user inputs. 
    loop {
        let mut vm_running = true;

        // Check if the vm is running and step if so.
        // This is not self-contained in a loop because the outside will contain debugger functions in the future.
        if vm_running {
            vm.cpu.step(&mut vm.memory);
        }
    }

}