use crate::config::{LocalConfig, OutputMode};
use crate::formatter::as_hex;

pub fn hash(config: &LocalConfig) -> argon2::Result<String> {
    match config.output_mode {
        OutputMode::Encoded => hash_to_encoded(config),
        OutputMode::Raw => hash_to_raw(config),
    }
}

fn hash_to_encoded(config: &LocalConfig) -> argon2::Result<String> {
    let result = argon2::hash_encoded(
        config.input.as_bytes(),
        config.salt.as_bytes(),
        &config.argon_config,
    )?;

    Ok(result)
}

fn hash_to_raw(config: &LocalConfig) -> argon2::Result<String> {
    let result = argon2::hash_raw(
        config.input.as_bytes(),
        config.salt.as_bytes(),
        &config.argon_config,
    )?;

    Ok(as_hex(&result))
}
