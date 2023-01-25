#!/bin/bash
cargo build --release &&strip ./target/release/salat_mv
ln -sf "$(realpath ./target/release/salat_mv)" ./target/pt
