use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

mod cpu;
mod memory;

use cpu::CpuInstruction;
use memory::load;

struct VirtualMachine {
    cpu: cpu::CpuState,
    memory: memory::Memory,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let mut vm: VirtualMachine = init(&args[1]);
    loop {
        cpu::step(&mut vm.cpu, CpuInstruction::NOP);
        break;
    }
}

fn init(file: &str) -> VirtualMachine {
    let new_instance: VirtualMachine = VirtualMachine {
        cpu: cpu::CpuState::empty(),
        memory: memory::Memory::empty(),
    };

    /* Determine if file exists, open, load to memory */
    let path = Path::new(file);
    let file = fs::File::open(path);

    println!("Opening file {}", path.display());
    if let Ok(mut file) = file {
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => println!("Contents:\n{}", contents),
            Err(e) => println!("Error occurred: {}", e),
        }
    } else {
        println!("Failed to open file: {}", file.unwrap_err());
    }

    new_instance
}

/* fn example(s: String) -> u32
{
    // Return statement doesn't need any "return" or semis
    0
} */
