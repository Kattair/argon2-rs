use std::env;

use config::LocalConfig;
use hasher::Hasher;

mod config;
mod hasher;

fn main() {
    let args: Vec<String> = env::args().collect();
    // remove program name from args
    let args = &args[1..];

    let config = LocalConfig::from_args(args);
    let hasher = Hasher::new(&config);

    match hasher.hash() {
        Ok(hash) => print!("{}", hash),
        Err(why) => {
            print!("Hashing failed: {}", why)
        }
    };
}
