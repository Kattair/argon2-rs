pub fn as_hex(bytes: &[u8]) -> String {
    let hex_bytes: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();

    hex_bytes.concat()
}
