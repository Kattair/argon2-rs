use std::error::Error;

use crate::config::{LocalConfig, OutputMode};

use super::formatter::{format_bytes_as_hex, format_string};

pub struct Hasher<'a> {
    config: LocalConfig<'a>,
}

impl Hasher<'_> {
    pub fn new(config: LocalConfig) -> Hasher {
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

#[cfg(test)]
mod tests {

    use argon2::{self, ThreadMode, Variant, Version};

    use super::*;

    #[test]
    fn hash() {
        let argon_config = get_argon2_config();
        let input = "a5cb643ba9a89e85f6fa087263e630aeaab6bc976db6ab4b850544aed2997cb2e3840e2ae91789f2ff3c3c95d112a95305b68f3df2dc92b4e04d48d25ac35a08";
        let salt = "abcdefgh";
        let output_mode = OutputMode::Encoded;

        let config = LocalConfig {
            argon_config,
            input,
            salt,
            output_mode,
        };
        let hasher = Hasher::new(config);

        let expected =
            "$argon2id$v=19$m=8192,t=3,p=1$YWJjZGVmZ2g$SOKfrty30oZCw4PaWBEAuW1ZD2D4HqkDCx2rYRI9xGE";
        let result = hasher.hash().unwrap();

        assert_eq!(expected, result);
    }

    fn get_argon2_config() -> argon2::Config<'static> {
        let mut config = argon2::Config::default();

        config.hash_length = 32;
        config.mem_cost = 8192;
        config.thread_mode = ThreadMode::from_threads(1);
        config.time_cost = 3;
        config.variant = Variant::Argon2id;
        config.version = Version::Version13;

        config
    }
}
