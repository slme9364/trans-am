use std::fs;
use std::env;
use std::path::Path;

pub fn open_file() -> fs::File {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\t{} <rust file>", args[0]);
        println!("Example:\n\t{} examples/ex_3.rs", args[0]);
        panic!("Exiting");
    }

    let reader = fs::File::open(Path::new(&args[1]));
    reader.ok().expect("Unable to open file")
}
