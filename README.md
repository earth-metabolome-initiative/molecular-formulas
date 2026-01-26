# Molecular formulas

[![CI](https://github.com/earth-metabolome-initiative/molecular-formulas/actions/workflows/rust.yml/badge.svg)](https://github.com/earth-metabolome-initiative/molecular-formulas/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/molecular-formulas/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/molecular-formulas)

A Rust crate for parsing, manipulating, and analyzing molecular formulas.

It validates correctly against **123,455,852** compounds from [PubChem](https://pubchem.ncbi.nlm.nih.gov/) (99.46% mass accuracy, **~475 ns/compound**) and is **fuzzed** for over **1 billion iterations** (see the [`fuzz`](https://github.com/earth-metabolome-initiative/molecular-formulas/tree/main/fuzz) crate) to ensure we handle all sorts of textual input.

TODO: Include examples of OCR-failure-resistant parsing

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
  - Optional [`serde`](https://crates.io/crates/serde) support for serialization.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
molecular-formulas = "0.1.0"
```

## Validation against PubChem

This library is rigorously tested against the [PubChem](https://pubchem.ncbi.nlm.nih.gov/) database, which contains over 123 million compounds. This ensures correctness when parsing real-world chemical data.

Specifically, we downloaded the `CID-Mass.gz` document you can find [in the `Extras` FTP directory of PubChem](https://ftp.ncbi.nlm.nih.gov/pubchem/Compound/Extras/).

You can run the validation suite yourself:

```bash
cargo test --release --test test_pubchem_validation -- --ignored --nocapture
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

## Error Handling

The parser includes detailed error detection for invalid formulas and provides descriptive error messages, distinguishing between tokenization errors, invalid valences (where applicable), and structure mismatches.

## Current Limitations

At this time, the parser does not support and might support in the future:

- Fractional counts (e.g., `C1.5H3`).

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/earth-metabolome-initiative/molecular-formulas/blob/main/LICENSE) file for details.
