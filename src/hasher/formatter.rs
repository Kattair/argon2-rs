pub fn format_string(string: &str) -> String {
    format!("{}", string)
}

pub fn format_bytes_as_hex(bytes: &[u8]) -> String {
    let hex_bytes: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    let hash = hex_bytes.concat();

    format!("{}", hash)
}
