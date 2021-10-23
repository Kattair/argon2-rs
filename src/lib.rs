use std::process;
use std::io::{self, Write};

pub fn check_mandatory_args(args: &[String]) {
    if args.is_empty() {
        print!("Missing mandatory arguments. Usage: argon2-rs [-h] salt [OPTIONS]");
        process::exit(1)
    }
}

pub fn check_help(arg: &str) {
    if "-h".eq(arg) {
        print_help();
        process::exit(0)
    }
}

pub fn print_help() {
    io::stdout()
        .write_all(include_bytes!("../README.md"))
        .expect("Writing to standard output failed.");
}