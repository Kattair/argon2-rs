use std::{env, process};

use argon2_rs::{config::LocalConfig, hasher};

fn main() {
    let args: Vec<String> = env::args().skip(1)
                                       .collect();
    let config = LocalConfig::from_args(&args);

    let unused_variable = "hehe";

    // print!("{:#?}", config);

    match hasher::hash(&config) {
        Ok(hash) => print!("{}", hash),
        Err(why) => {
            print!("Hashing failed: {}", why);
            process::exit(1);
        }
    };
}
