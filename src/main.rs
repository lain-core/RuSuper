use std::env;
use std::path::Path;

mod cpu;
mod memory;
mod romdata;

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
        // Start Running.
        cpu::run(path);
    }
    else {
        println!("You must specify a *.sfc file to run!");
    }
}
