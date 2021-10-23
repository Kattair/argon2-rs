use super::{
    arg_checkers::{check_help, check_mandatory_args},
    parser::parse_args,
    LocalConfig, OutputMode,
};
use std::io::{self, Read};

impl<'a> LocalConfig<'a> {
    fn new(
        argon_config: argon2::Config<'a>,
        salt: &'a str,
        output_mode: OutputMode,
    ) -> LocalConfig<'a> {
        LocalConfig {
            argon_config,
            input: "",
            salt,
            output_mode,
        }
    }

    pub fn from_args(args: &'a [String]) -> LocalConfig<'a> {
        check_mandatory_args(&args);
        check_help(&args[0]);

        let salt = &args[0];

        let (argon_config, output_mode) = parse_args(&args[1..]);

        LocalConfig::new(argon_config, salt, output_mode)
    }

    pub fn read_input(&mut self, buffer: &'a mut String) {
        io::stdin()
            .read_to_string(buffer)
            .expect("Failed to read standard input");

        self.input = buffer;
    }
}
