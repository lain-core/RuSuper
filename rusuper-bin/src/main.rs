use std::env;
use rusuper_lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() > 1
    {
        rusuper_lib::init(&args[1]);
    }
}
