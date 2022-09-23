use std::env;
use std::process;

use rusty_grep::Config;

fn main() {
    // ingest command line args
    let args: Vec<String> = env::args().collect();

    // store our query and file args, ignoring the program's name in index 0
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = rusty_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}

