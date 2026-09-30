#!/bin/bash
set -eu

cd "$SRC/molecular-formulas"

targets=$(cargo fuzz list --fuzz-dir fuzz)
if [[ -z "$targets" ]]; then
    echo "cargo fuzz list named no target" >&2
    exit 1
fi

for name in $targets; do
    if ! compgen -G "fuzz/seeds/$name/*" >/dev/null; then
        echo "fuzz target $name has no seeds in fuzz/seeds/$name" >&2
        exit 1
    fi
done

cargo fuzz build -O --debug-assertions --fuzz-dir fuzz

target_dir=fuzz/target/x86_64-unknown-linux-gnu/release
for name in $targets; do
    cp "$target_dir/$name" "$OUT/"
    zip -q -j "$OUT/${name}_seed_corpus.zip" "fuzz/seeds/$name"/*
done
