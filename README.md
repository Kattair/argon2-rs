# argon2_rs
Simple argon2 hashing program using crate rust-argon2.
Usability mimicks usability of argon2 utility on Debian: https://manpages.debian.org/buster/argon2/argon2.1.en.html
Default values are defined by the rust-argon2 crate here: https://docs.sru-systems.com/rust-argon2/0.8.0/argon2/struct.Config.html

# Usage
argon2-rs salt [OPTIONS]

| Option | Parameters | Description                                                        |
| ------ | ---------- | ------------------------------------------------------------------ |
| -h     |            | Display tool usage                                                 |
| -i     |            | Use Argon2i (this is the default)                                  |
| -d     |            | Use Argon2d instead of Argon2i                                     |
| -id    |            | Use Argon2id instead of Argon2i                                    |
| -t     | u_int_32   | Sets the number of iterations to N (default = 3)                   |
| -m     | u_int_32   | Sets the memory usage of 2^N KB (default = 12)                     |
| -k     | u_int_32   | Sets the memory usage of N KB (default = 4096)                     |
| -p     | u_int_32   | Sets parallelism to N threads (default = 1)                        |
| -l     | u_int_32   | Sets hash output length to N bytes (default = 32)                  |
| -e     |            | Output only encoded hash                                           |
| -r     |            | Output only the raw bytes of the hash                              |
| -v     | 10 or 13   | Argon2 version (defaults to the most recent version, currently 13) |
