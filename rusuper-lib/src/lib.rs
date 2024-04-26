use std::fs;
use std::path::Path;

mod cpu;
mod memory;

use cpu::CpuInstruction;
use memory::load;

/// VM Struct which contains the individual pieces of the system.
struct VirtualMachine {
    cpu: cpu::CpuState,
    memory: memory::Memory,
}

/// Main function, initializes and runs core.
/// 
/// # Parameters
/// - `file`: Target file to open
pub fn init(file: &str) {    
    let mut vm =  VirtualMachine {
        cpu: cpu::CpuState::new(),
        memory: memory::Memory::new(),
    };

    // Determine if file exists, open, load to memory
    let path = Path::new(file);
    if let Ok(file) = fs::File::open(path){
        memory::load(&mut vm.memory, file);
    }
    else {
        println!("Failed to read file {}", path.display());
    }

    // Start Running.
    // TODO: find a way to pace this correctly.
    loop {
        cpu::step(&mut vm.cpu, CpuInstruction::NOP);
        break;
    }
}