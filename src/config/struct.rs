#[derive(Debug)]
pub enum OutputMode {
    Raw,
    Encoded,
}

#[derive(Debug)]
pub struct LocalConfig<'a> {
    pub argon_config: argon2::Config<'a>,
    pub input: &'a str,
    pub salt: &'a str,
    pub output_mode: OutputMode,
}
