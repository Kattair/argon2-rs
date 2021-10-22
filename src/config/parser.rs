use argon2::{self, Config, ThreadMode, Variant, Version};

use super::OutputMode;

pub fn parse_args(args: &[String]) -> (Config, OutputMode) {
    let mut config = Config::default();
    let mut output_mode = OutputMode::Encoded;

    if args.is_empty() {
        return (config, output_mode);
    }

    for i in 0..args.len() {
        let arg = &args[i];

        if "-i".eq(arg) {
            config.variant = Variant::Argon2i;
        } else if "-d".eq(arg) {
            config.variant = Variant::Argon2d;
        } else if "-id".eq(arg) {
            config.variant = Variant::Argon2id;
        } else if "-t".eq(arg) {
            config.time_cost = args[i + 1].parse().unwrap();
        } else if "-m".eq(arg) {
            config.mem_cost = 2_u32.pow(args[i + 1].parse().unwrap());
        } else if "-k".eq(arg) {
            config.mem_cost = args[i + 1].parse().unwrap();
        } else if "-p".eq(arg) {
            config.thread_mode = ThreadMode::from_threads(args[i + 1].parse().unwrap());
        } else if "-l".eq(arg) {
            config.hash_length = args[i + 1].parse().unwrap();
        } else if "-e".eq(arg) {
            output_mode = OutputMode::Encoded;
        } else if "-r".eq(arg) {
            output_mode = OutputMode::Raw;
        } else if "-v".eq(arg) {
            let version: u8 = args[i + 1].parse().unwrap();

            if version.eq(&10) {
                config.version = Version::Version10;
            } else if version.eq(&13) {
                config.version = Version::Version13;
            }
        }
    }

    (config, output_mode)
}

pub fn _parse_file(_filename: &str) -> Config {
    Config::default()
}
