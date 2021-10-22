use super::{
    arg_checkers::{check_help, check_mandatory_args},
    parser::parse_args,
    LocalConfig,
};
use std::io::{self, Read};

impl LocalConfig<'_> {
    pub fn from_args(args: &[String]) -> LocalConfig {
        check_mandatory_args(args);
        check_help(&args[0]);

        let salt = &args[0];

        let (config, output_mode) = parse_args(&args[1..]);

        let mut input = String::new();

        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read stdin");

        let input = input;

        LocalConfig {
            argon_config: config,
            input: input.clone(),
            salt: salt.clone(),
            output_mode,
        }
    }
}
