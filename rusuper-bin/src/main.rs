use std::env;
use std::path::Path;
use rusuper_lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() > 1
    {
        let mut file = std::fs::canonicalize(Path::new(&args[1])).expect(
            "File not found"
        );
        rusuper_lib::init(&mut file);
    }
}
