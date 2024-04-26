use std::fs;
use std::io::Read;
use std::path::PathBuf;

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
pub fn init(path: &mut PathBuf) {    
    let mut vm =  VirtualMachine {
        cpu: cpu::CpuState::new(),
        memory: memory::Memory::new(),
    };

    if let Ok(file) = fs::File::open(&path){
        memory::load(&mut vm.memory, file);
    }
    else {
        println!("Failed to read file {}", path.display());
    }

    // vm.memory.dump();

    // Start Running.
    // TODO: find a way to pace this correctly.
    loop {
        cpu::step(&mut vm.cpu, CpuInstruction::NOP);
        break;
    }
}