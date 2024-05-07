use std::fs;
use std::path::{Path, PathBuf};
use rusuper_lib::VirtualMachine;

const ASM_TEST_DIR: &str = "./asm/65816/";
const ASM_NOP_STP: &str = "ADC_NOP_STP.sfc";

#[test]
fn test_nop_stp() {
    let mut vm = test_initialize(ASM_TEST_DIR.to_owned() + ASM_NOP_STP);

    vm.cpu.step(&mut vm.memory);    // Run NOP
    assert_eq!(0x0001, vm.cpu.pc)

}

fn test_initialize(path: String) -> VirtualMachine {
    let path = std::fs::canonicalize(Path::new(&path)).expect(
        "File not found"
    );

    let mut vm =  VirtualMachine::new();

    if let Ok(file) = fs::File::open(&path){
        vm.memory.load_rom(file);
    }
    else {
        panic!("Failed to load test file {}", path.display());
    }

    vm
}