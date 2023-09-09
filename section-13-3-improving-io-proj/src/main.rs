use std::env;
use std::process;

use section_13_3_improving_io_proj::{run, Config};

fn main() {
    // let args = env::args().collect::<Vec<String>>();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}\n", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
