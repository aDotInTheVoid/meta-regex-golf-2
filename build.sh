#!/usr/bin/env bash

# Each folder has different cargo configs, so we can't use a workspace

mkdir -p out

RUSTFLAGS="-C target-cpu=native"
cargo build --release
for i in v*
do    
    cp target/release/$i ./out
done

for i in x*
do
    cp target/release/$i ./out
 done
