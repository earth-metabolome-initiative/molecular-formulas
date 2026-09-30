# Molecular Formulas Fuzzing

This directory contains harnesses for **fuzz testing** the `molecular_formulas` crate.

## What is Fuzzing?

[Fuzzing](https://rust-fuzz.github.io/book/) is an automated testing technique that feeds random, invalid, or unexpected inputs into your program to find bugs, crashes, or security vulnerabilities (like panics or infinite loops). We use [libFuzzer](https://llvm.org/docs/LibFuzzer.html) through [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz), and [ClusterFuzzLite](https://google.github.io/clusterfuzzlite/) runs the target on every pull request and daily on `main`.

## How it works

We utilize **Structure-Aware Fuzzing**. Instead of generating purely random strings (which would mostly just test the "invalid character" error handler), we use the [`Arbitrary`](https://crates.io/crates/arbitrary) trait. This generates syntactically plausible sequences of tokens (elements, isotopes, brackets) to deeply exercise the parser's logic for nested structures and complex formulas.

## Getting Started

1. Install cargo-fuzz (needs a nightly toolchain)

```bash
cargo install cargo-fuzz
```

1. Run the Fuzzer

The `from_str` target tests parsing consistency, round-trip serialization, and method safety across millions of generated inputs.

```bash
cargo +nightly fuzz run from_str
```

1. Debugging Crashes

If a crash is found, the input is saved in `fuzz/artifacts/from_str/`. Crashes should be included in your test suite so to avoid potential future regressions. You can replay one to investigate the issue:

```bash
cargo +nightly fuzz run from_str fuzz/artifacts/from_str/crash-<hash>
```
