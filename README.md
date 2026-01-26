# Molecular formulas

[![CI](https://github.com/earth-metabolome-initiative/molecular-formulas/actions/workflows/rust.yml/badge.svg)](https://github.com/earth-metabolome-initiative/molecular-formulas/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/molecular-formulas/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/molecular-formulas)
[![Crates.io](https://img.shields.io/crates/v/molecular-formulas.svg)](https://crates.io/crates/molecular-formulas)
[![Docs.rs](https://docs.rs/molecular-formulas/badge.svg)](https://docs.rs/molecular-formulas)

A Rust crate for parsing, manipulating, and analyzing molecular formulas.

It validates correctly against 120M compounds from [PubChem](https://pubchem.ncbi.nlm.nih.gov/) (99.46% mass accuracy) and is fuzzed for over 1 billion iterations (see the [`fuzz`](https://github.com/earth-metabolome-initiative/molecular-formulas/tree/main/fuzz) crate) to ensure we handle all sorts of textual input.

## Features

- **Standard Parsing**: Supports nested groups (e.g., `C6H5(CH2)2OH`), hydrates, salts, isotopes (e.g., `[13C]H4` or `¹³CH₄`), and flexible charge notation (e.g., `SO4^2-`, `Fe+3`, `[OH]-`).
- **Modular AST**: The internal representation allows selecting integer types (`u8`, `u16`, `u32`) and enabling or disabling support for "Residuals" (wildcards) via types like `MolecularFormula` vs `ResidualFormula`. If something is missing, make a PR and we can modularly add it!
- **Chemical Properties**:
  - Check [**Hill System**](https://en.wikipedia.org/wiki/Hill_system) sorting conformity.
  - Identify chemical classes (noble gas compounds).
  - **Charge**: Calculate and inspect total charge.
- **Composition Analysis**:
  - **Isotopes**: Check for presence of specific isotopes.
  - **Mixtures**: Handle and inspect molecular mixtures.
- **Mass Calculations**:
  - **Monoisotopic Mass** (Isotopologue mass).
  - **Average Molar Mass**.
  - **Mass over Charge** (m/z) ratio.
- **Validation**: Tested against the entire [PubChem](https://pubchem.ncbi.nlm.nih.gov/) compound database (123M+ entries).
- **Ecosystem**:
  - Built on [`elements-rs`](https://github.com/earth-metabolome-initiative/elements-rs) for accurate element and isotope data.
  - Uses [`thiserror`](https://crates.io/crates/thiserror) for ergonomic error handling.
  - Optional [`serde`](https://crates.io/crates/serde) support for serialization/deserialization.
- **Embedded Compatible**: `#![no_std]` capable (requires `alloc`), making it suitable for WASM and embedded applications.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
molecular-formulas = "0.1.0"
```

## Usage

Here are some examples of how to use the library:

### Basic Parsing and Properties

```rust
use std::str::FromStr;
use molecular_formulas::prelude::*;

// efficient u16 counters and i16 charge
// Note: You can use u32 or u64 for larger molecules.
let formula: ChemicalFormula = ChemicalFormula::from_str("C6H12O6").unwrap();

println!("Formula: {}", formula);
println!("Monoisotopic Mass: {} Da", formula.isotopologue_mass());
println!("Average Mass: {} Da", formula.molar_mass());
println!("Charge: {}", formula.charge());
```

### Complex Formulas, Hydrates and Ions

The parser handles parentheses, brackets, hydrates (dots), and charges with ease.

```rust
use std::str::FromStr;
use molecular_formulas::prelude::*;

// Copper(II) sulfate pentahydrate
let hydrate: ChemicalFormula = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();
assert_eq!(hydrate.to_string(), "CuSO₄.5H₂O");

// An ion with unicode charge notation
let ion: ChemicalFormula = ChemicalFormula::from_str("SO₄²⁻").unwrap();
assert_eq!(ion.charge(), -2.0);

// Recursively nested groups
let complex: ChemicalFormula = ChemicalFormula::from_str("[Co(NH3)5Cl]Cl2").unwrap();
```

### Isotopes

You can specify isotopes using standard notation (superscripts or square brackets).

```rust
use std::str::FromStr;
use molecular_formulas::prelude::*;

// Carbon-13 labeled methane
let labeled: ChemicalFormula = ChemicalFormula::from_str("[13C]H4").unwrap();
// or
let labeled_unicode: ChemicalFormula = ChemicalFormula::from_str("¹³CH₄").unwrap();

assert_eq!(labeled, labeled_unicode);

// Check if it contains specific isotopes
let c13 = Isotope::try_from((Element::C, 13_u16)).unwrap();
assert!(labeled.contains_isotope(c13));
```

### OCR-Resistant Parsing

The parser is designed to be robust against common OCR errors and unicode variations, handling multiple types of hyphens, dashes, and dots seamlessly.

```rust
use std::str::FromStr;
use molecular_formulas::prelude::*;

// Standard notation
let f1: ChemicalFormula = ChemicalFormula::from_str("CuSO4.5H2O").unwrap();
// OCR error: '｡' (Halfwidth Ideographic Full Stop) instead of '.'
let f2: ChemicalFormula = ChemicalFormula::from_str("CuSO4｡5H2O").unwrap();
assert_eq!(f1, f2);

// Standard charge
let c1: ChemicalFormula = ChemicalFormula::from_str("SO4-2").unwrap();
// OCR error: Using En Dash '–' instead of Minus '-'
let c2: ChemicalFormula = ChemicalFormula::from_str("SO4–2").unwrap();
assert_eq!(c1, c2);
```

### InChI Formula Validation

The library supports strictly validated InChI-style formulas, which enforce Hill notation sorting (C first, H second, then alphabetical).

```rust
use std::str::FromStr;
use molecular_formulas::errors::ParserError;
use molecular_formulas::prelude::*;

// Valid Hill-sorted formula
let valid: InChIFormula = InChIFormula::from_str("C2H5O").unwrap();

// Invalid: Not Hill-sorted (O comes before H)
let invalid: Result<InChIFormula, _> = InChIFormula::from_str("C2OH5");
assert_eq!(invalid.unwrap_err(), ParserError::NotHillOrdered);
```

## Validation against PubChem

This library is tested against the [PubChem](https://pubchem.ncbi.nlm.nih.gov/) database, which contains over 123 million compounds. This ensures correctness when parsing real-world chemical data.

We validate both `ChemicalFormula` (mass analysis) and `InChIFormula` (the formula layer of InChI).

Specifically, we download the `CID-Mass.gz` and `CID-InChI-Key.gz` documents, which can be found [in the `Extras` FTP directory of PubChem](https://ftp.ncbi.nlm.nih.gov/pubchem/Compound/Extras/).

You can run the validation suites yourself:

```bash
# Validate Mass Calculation (ChemicalFormula), takes about 55 seconds
cargo test --release --test test_pubchem_validation -- --ignored --nocapture

# Validate InChI Parsing (InChIFormula), takes about 45 seconds
cargo test --release --test test_pubchem_inchi_validation -- --ignored --nocapture
```

### Validation Results (January 2026)

| Metric                       | Value            |
|------------------------------|------------------|
| Total processed              | 123,455,852      |
| Total time required          | 58.68 s          |
| Processing speed             | 2,103,788 cmp/s  |
| Exact matches                | 66,465           |
| Within tolerance             | 122,720,777      |
| Mismatches                   | 668,610          |
|   - Ion mismatches           | 106,525          |
|   - Neutral mismatches       | 562,085          |
| Mass accuracy (within 0.001) | 99.46%           |

*Note: The remaining ~0.5% mismatches are largely attributed to inconsistencies or errors in the source PubChem records rather than parsing errors.*

You can find a report of the worst mismatches in [worst_mismatches.md](https://github.com/earth-metabolome-initiative/molecular-formulas/blob/main/worst_mismatches.md).

## Benchmarks

This crate includes benchmarks to measure parsing performance for both `InChIFormula` and `ChemicalFormula`.

To run the benchmarks:

```bash
cargo bench
```

Current benchmarks cover:

- **InChIFormula**: Parsing a large mixture string with 76 components (**~3.75 µs**).
- **ChemicalFormula**: Parsing a complex formula with unicode subscripts, charges, and multiple elements (`C₃₉₀H₄₀₄B₂Br₂ClCs₂F₁₁K₂MnN₂₆Na₂O₁₀₀OsPdS₃W₂³⁻`) (**~801 ns**).

## Current Limitations

At this time, the parser does not support and might support in the future:

- Fractional counts (e.g., `C1.5H3`).

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/earth-metabolome-initiative/molecular-formulas/blob/main/LICENSE) file for details.
