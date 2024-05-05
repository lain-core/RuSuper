use std::fs;
use std::path::PathBuf;

mod cpu;
mod memory;

use cpu::CpuInstruction;

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

    vm.memory.dump_bank(0x00);

    // Start Running.
    // TODO: find a way to pace this correctly.
    // loop {
    //     cpu::step(&mut vm.cpu, CpuInstruction::NOP);
    //     break;
    // }
}