use std::{
    io::{self, Write},
    process,
};

use argon2::{Config, Variant, Version};

use crate::config::OutputMode;

pub fn parse_args(args: &[String]) -> (Config, String, OutputMode) {
    check_mandatory_args(args);
    check_help(&args[0]);

    let salt = args[0].to_owned();
    let mut config = Config::default();
    let mut output_mode = OutputMode::Encoded;

    for i in 0..args.len() {
        let arg = &args[i];

        match arg.as_str() {
            "-i" => config.variant = Variant::Argon2i,
            "-d" => config.variant = Variant::Argon2d,
            "-id" => config.variant = Variant::Argon2id,
            "-t" => config.time_cost = args[i + 1].parse().unwrap(),
            "-m" => config.mem_cost = 2_u32.pow(args[i + 1].parse().unwrap()),
            "-k" => config.mem_cost = args[i + 1].parse().unwrap(),
            "-p" => config.lanes = args[i + 1].parse().unwrap(),
            "-l" => config.hash_length = args[i + 1].parse().unwrap(),
            "-e" => output_mode = OutputMode::Encoded,
            "-r" => output_mode = OutputMode::Raw,
            "-v" => match args[i + 1].parse().unwrap() {
                10 => config.version = Version::Version10,
                13 => config.version = Version::Version13,
                _ => (),
            },
            _ => (),
        }
    }

    (config, salt, output_mode)
}

fn check_mandatory_args(args: &[String]) {
    if args.is_empty() {
        print!("Missing mandatory arguments. Usage: argon2-rs [-h] salt [OPTIONS]");
        process::exit(1)
    }
}

fn check_help(arg: &str) {
    if "-h".eq(arg) {
        print_help();
        process::exit(0)
    }
}

fn print_help() {
    io::stdout()
        .write_all(include_bytes!("../README.md"))
        .expect("Writing to standard output failed.");
}
