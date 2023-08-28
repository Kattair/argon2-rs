use crate::parser::parse_args;
use std::io::{self, Read};

#[derive(Debug)]
pub enum OutputMode {
    Raw,
    Encoded,
}

#[derive(Debug)]
pub struct LocalConfig<'a> {
    pub argon_config: argon2::Config<'a>,
    pub input: String,
    pub salt: String,
    pub output_mode: OutputMode,
}

impl<'a> LocalConfig<'a> {
    fn new(
        argon_config: argon2::Config<'a>,
        input: String,
        salt: String,
        output_mode: OutputMode,
    ) -> LocalConfig<'a> {
        LocalConfig {
            argon_config,
            input,
            salt,
            output_mode,
        }
    }

    pub fn from_args(args: &[String]) -> LocalConfig {
        let (argon_config, salt, output_mode) = parse_args(args);

        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read standard input");

        LocalConfig::new(argon_config, input, salt, output_mode)
    }
}
