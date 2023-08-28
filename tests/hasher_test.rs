use argon2::{self, Variant, Version};
use argon2_rs::{
    config::{LocalConfig, OutputMode},
    hasher,
};

#[test]
fn hash() {
    let argon_config = get_argon2_config();
    let input = "a5cb643ba9a89e85f6fa087263e630aeaab6bc976db6ab4b850544aed2997cb2e3840e2ae91789f2ff3c3c95d112a95305b68f3df2dc92b4e04d48d25ac35a08";
    let salt = "abcdefgh";
    let output_mode = OutputMode::Encoded;

    let config = LocalConfig {
        argon_config,
        input: input.to_owned(),
        salt: salt.to_owned(),
        output_mode,
    };

    let expected =
        "$argon2id$v=19$m=8192,t=3,p=1$YWJjZGVmZ2g$SOKfrty30oZCw4PaWBEAuW1ZD2D4HqkDCx2rYRI9xGE";
    let result = hasher::hash(&config).unwrap();

    assert_eq!(expected, result);
}

fn get_argon2_config() -> argon2::Config<'static> {
    let mut config = argon2::Config::default();

    config.hash_length = 32;
    config.mem_cost = 8192;
    config.time_cost = 3;
    config.lanes = 1;
    config.variant = Variant::Argon2id;
    config.version = Version::Version13;

    config
}
