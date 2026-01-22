# Fuzzing harnesses for molecular formulas

This directory contains fuzzing harnesses to test the robustness and correctness of the `molecular_formulas` crate using [honggfuzz](https://github.com/google/honggfuzz).

## Prerequisites

To run the fuzzers, you need a Linux system (WSL works too) and the following dependencies:

1. **System Dependencies**:

    ```bash
    sudo apt install build-essential binutils-dev libunwind-dev
    ```

2. **Honggfuzz**:

    ```bash
    cargo install honggfuzz
    ```

## Directory Structure

* `fuzz_targets/`: Contains the actual source code for the fuzz targets (e.g., `from_str.rs`).
* `hfuzz_workspace/`: Directory where `honggfuzz` stores its corpus (inputs), crashes, and reports. Created automatically when you run the fuzzer.
* `hfuzz_target/`: Build artifacts for the fuzz targets.

## Available Harnesses

### `from_str`

This harness fuzzes the `FromStr` implementation of `MolecularFormula`. It generates random strings and attempts to parse them.

**What it checks:**

* **Panic Freedom**: Parsing should never panic, only return `Ok` or `Err`.
* **Round-Trip Consistency**: If a string parses successfully, converting the result back to a string (`to_string()`) and re-parsing it should yield an equivalent formula.
* **Property Accessors**: Calls various methods (e.g., `mass`, `charge`, `elements`) on valid formulas to ensure they don't panic.
* **Performance**: Checks for unusually slow parsing times (timeout check).

**To run:**

```bash
cargo hfuzz run from_str
```

**To debug a crash:**

If the fuzzer finds a crash, it will save the crashing input in `hfuzz_workspace/from_str/`. You can reproduce it with:

```bash
# Verify a specific crash file
cargo hfuzz run-debug from_str hfuzz_workspace/from_str/SIGABRT.PC.7ffff7c9eb2c.STACK...fuzz
```

## Interpretation of Results

* **SIGABRT / Panic**: Found a bug! Check the backtrace in the report file in `hfuzz_workspace`.
* **Timeout**: The parser might have stuck in an infinite loop or is strictly too slow for the given input.

## Cleaning Up

To clean up all fuzzing artifacts (corpus, crashes, build files):

```bash
cargo hfuzz clean
```
