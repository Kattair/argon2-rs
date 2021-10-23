use std::env;

use argon2_rs::{check_help, check_mandatory_args};
use config::LocalConfig;
use hasher::Hasher;

mod config;
mod hasher;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    check_mandatory_args(&args);
    check_help(&args[0]);

    let mut config = LocalConfig::from_args(&args);
    let mut input: String = String::new();

    config.read_input(&mut input);

    let config = config;
    // print!("{:#?}", config);
    let hasher = Hasher::new(config);

    match hasher.hash() {
        Ok(hash) => print!("{}", hash),
        Err(why) => {
            print!("Hashing failed: {}", why)
        }
    };
}
