pub enum OutputMode {
    Raw,
    Encoded,
}

pub struct LocalConfig<'a> {
    pub argon_config: argon2::Config<'a>,
    pub input: String,
    pub salt: String,
    pub output_mode: OutputMode,
}
