use std::{env, process};

use rust_bucket::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}