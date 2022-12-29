#!/bin/bash
cargo build --release &&strip ./target/release/salat_mv
cp ./src/islands.csv ./target/release/
cp ./src/atolls.csv ./target/release/
cp ./src/ptdata.csv ./target/release/
ln -sf "$(realpath ./target/release/salat_mv)" ./target/pt
