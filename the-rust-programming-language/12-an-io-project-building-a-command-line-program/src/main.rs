use the_rust_programming_language_12_an_io_project_building_a_command_line_program as minigrep;

use std::env;
use std::process;

fn main () {
    let config = minigrep::Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Oh no: {error}");
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Oh no again: {error}");
        process::exit(1);
    }
}
