use std::error::Error;

use crate::config::{LocalConfig, OutputMode};

use super::formatter::{format_bytes_as_hex, format_string};

pub struct Hasher<'a> {
    config: &'a LocalConfig<'a>,
}

impl Hasher<'_> {
    pub fn new<'a>(config: &'a LocalConfig) -> Hasher<'a> {
        Hasher { config }
    }

    pub fn hash(&self) -> Result<String, Box<dyn Error>> {
        match self.config.output_mode {
            OutputMode::Encoded => self.hash_to_encoded(),
            OutputMode::Raw => self.hash_to_raw(),
        }
    }

    fn hash_to_encoded(&self) -> Result<String, Box<dyn Error>> {
        let result = argon2::hash_encoded(
            self.config.input.as_bytes(),
            self.config.salt.as_bytes(),
            &self.config.argon_config,
        )?;

        Ok(format_string(&result))
    }

    fn hash_to_raw(&self) -> Result<String, Box<dyn Error>> {
        let result = argon2::hash_raw(
            self.config.input.as_bytes(),
            self.config.salt.as_bytes(),
            &self.config.argon_config,
        )?;

        Ok(format_bytes_as_hex(&result))
    }
}
