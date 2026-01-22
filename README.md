# Molecular formulas

[![CI](https://github.com/earth-metabolome-initiative/molecular-formulas/actions/workflows/rust.yml/badge.svg)](https://github.com/earth-metabolome-initiative/molecular-formulas/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/molecular-formulas/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/molecular-formulas)

A Rust crate for parsing, manipulating, and analyzing molecular formulas.

It validates correctly against **123,455,852** compounds from [PubChem](https://pubchem.ncbi.nlm.nih.gov/) (99.46% mass accuracy, **~475 ns/compound**) and is **fuzzed** for over **1 billion iterations** (see the [`fuzz`](https://github.com/earth-metabolome-initiative/molecular-formulas/tree/main/fuzz) crate) to ensure we handle all sorts of textual input.

We have **high test coverage** and aim to provide doctests examples for all methods to ensure the documentation is always up-to-date and working.

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
- **Integration**: Optional features for [`serde`](https://crates.io/crates/serde) serialization.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
molecular-formulas = "0.1.0"
```

## Usage

### Basic Parsing and Mass Calculation

```rust
use molecular_formulas::MolecularFormula;

// Parse a simple formula
let water: MolecularFormula = "H2O".parse().unwrap();
assert_eq!(water.to_string(), "H₂O");
assert!((water.molar_mass() - 18.015).abs() < 0.001);
assert!((water.isotopologue_mass_without_charge() - 18.010).abs() < 0.001);

// Parse a complex formula with groups and charge
// Note: Use standard charge notation (e.g., "2-", "-2" or "²⁻")
let sulfate: MolecularFormula = "SO4-2".parse().unwrap();
assert_eq!(sulfate.charge(), -2.0);

// Supports standard notation for ions
let hydronium: MolecularFormula = "H3O+".parse().unwrap();
assert_eq!(hydronium.charge(), 1.0);
```

### Worked Examples

#### Molar Mass vs. Monoisotopic Mass

- **Average Molar Mass**: The mass calculated using the standard atomic weight (weighted average of natural isotopes). Useful for macroscopic stoichiometry.
- **Monoisotopic Mass**: The mass calculated using the mass of the most abundant isotope for each element. Useful for high-resolution mass spectrometry.

```rust
use molecular_formulas::MolecularFormula;

let water: MolecularFormula = "H2O".parse().unwrap();

// Standard atomic weight: H (1.008) * 2 + O (15.999) = 18.015
assert!((water.molar_mass() - 18.015).abs() < 0.001);

// Monoisotopic mass: ¹H (1.00783) * 2 + ¹⁶O (15.99491) = 18.010565
assert!((water.isotopologue_mass_without_charge() - 18.01056).abs() < 0.001);
```

#### Isotopes

You can specify specific isotopes using standard notation.

```rust
use molecular_formulas::MolecularFormula;

// Standard Methane
let methane: MolecularFormula = "CH4".parse().unwrap();

// 13C-labeled Methane (using superscript ¹³C or [13C])
let c13_methane: MolecularFormula = "¹³CH4".parse().unwrap();

let mass_methane = methane.isotopologue_mass_without_charge();
let mass_c13_methane = c13_methane.isotopologue_mass_without_charge();

// The mass difference is exactly the neutron difference (approx 1.00335 Da)
assert!((mass_c13_methane - mass_methane - 1.00335).abs() < 0.001);
```

#### Mixtures

Mixtures are separated by a dot `.` and allow representing adducts, salts, or complex systems.

```rust
use molecular_formulas::MolecularFormula;

// A salt hydrate
let system: MolecularFormula = "CuSO4.5H2O".parse().unwrap();

// Inspect the components
let parts: Vec<String> = system.subformulas()
    .map(|f| f.to_string())
    .collect();

assert_eq!(parts, vec!["CuSO₄", "H₂O", "H₂O", "H₂O", "H₂O", "H₂O"]);
assert_eq!(system.number_of_mixtures(), 6); // 1 CuSO4 + 5 H2O
```

#### Mass over Charge (m/z)

For ions, the m/z ratio is critical in mass spectrometry.

```rust
use molecular_formulas::MolecularFormula;

// Sulfate ion with charge -2
let sulfate: MolecularFormula = "SO4-2".parse().unwrap();

assert_eq!(sulfate.charge(), -2.0);

// For anions, electron mass is added. For cations, electron mass is subtracted.
// m/z = (Mass + (Charge * ElectronMass)) / |Charge|
// Note: This library handles the electron mass adjustment correctly.
let mz = sulfate.isotopologue_mass_over_charge();
```

### Property Checks

You can verify chemical formatting conventions and properties:

```rust
use molecular_formulas::MolecularFormula;

// Hill Sorting
let glucose: MolecularFormula = "C6H12O6".parse().unwrap();
assert!(glucose.is_hill_sorted()); // True: C then H then alphabetical

let wrong: MolecularFormula = "H12O6C6".parse().unwrap();
assert!(!wrong.is_hill_sorted()); // False

// Noble Gases
let neon: MolecularFormula = "Ne".parse().unwrap();
assert!(neon.is_noble_gas_compound());
```

### Handling Unknown Structures (Residuals)

If you are dealing with generic chemistry where parts of the molecule are unknown (e.g., "R" groups), use `ResidualFormula`.

```rust
use molecular_formulas::{ResidualFormula, MolecularFormula};

// 'R' represents a generic residual group
let structure: ResidualFormula = "C6H5R".parse().unwrap();
assert_eq!(structure.to_string(), "C₆H₅R");
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

You can find a report of the worst mismatches in [worst_mismatches.md](worst_mismatches.md).

## Error Handling

The parser includes detailed error detection for invalid formulas and provides descriptive error messages, distinguishing between tokenization errors, invalid valences (where applicable), and structure mismatches.

```rust
use molecular_formulas::MolecularFormula;

let result = "H2((O)".parse::<MolecularFormula>();
assert!(result.is_err());
// Error: Token error: Unexpected terminator `(` while parsing tokens.
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
