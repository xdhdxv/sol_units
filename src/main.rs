use std::env;
use std::process;

use sol_units::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = sol_units::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}