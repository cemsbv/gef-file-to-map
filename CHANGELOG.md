# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2025-10-31

### Bug Fixes

- *Deps*:
    - Update rust crates pyo3, winnow and thiserror
    - Update rust crate pyo3 to 0.27.0
    - Update rust crate pyo3 to 0.26.0
    - Update rust crate pyo3 to 0.25.0
- *Python*:  [**BREAKING**]Bump minimum supported Python version to 3.11

## [0.2.1] - 2025-07-22

### Bug Fixes
- *Header*: Allow empty header values

### Documentation
- *Readme*: Don't link to specific version in PyPi badge

### Miscellaneous Tasks
- *Ci*: Move from dependabot to renovate

### Testing
- *Bench*: Add benchmarks

## [0.2.0] - 2025-03-10

### Bug Fixes
- *Ci*: Manually install Rust pipeline in CI
- *Deps*: Update nom to 8

### Refactor
- *Deps*: Migrate nom to winnow 0.3
- *Parsing*:  [**BREAKING**]Improve parsing of headers and add better error reporting

## [0.1.1] - 2025-03-10

### Bug Fixes
- *Ci*: Fix publishing CI

### Documentation
- Fix error in example
- Add pypi version badge

### Miscellaneous Tasks
- *Ci*: Only run release CI on tags
- Update dependencies
- Update dependencies

## [0.1.0] - 2023-02-22

### Bug Fixes

- *Ci*:
    - Copy wheel generation from @messense
    - Require minimum Python version of 3.8

### Refactor
- Move from pygef repo

<!-- CEMS BV. -->
