#!/bin/bash
for day in $(seq 1 24); do
    name=$(printf "day%02d" $day)
    if [[ -d $name ]]; then
        cargo test --release --manifest-path=$name/Cargo.toml
    fi
done
