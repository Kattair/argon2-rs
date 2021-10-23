#!/bin/bash

cargo test
cargo build --release --target x86_64-unknown-linux-musl
cross build --release --target x86_64-pc-windows-gnu