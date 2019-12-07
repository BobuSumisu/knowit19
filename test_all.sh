#!/bin/bash
for m in luke??/Cargo.toml; do cargo test --release --manifest-path=$m; done
