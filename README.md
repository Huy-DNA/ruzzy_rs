# ruzzy_rs
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/ruzzy.svg
[crates-url]: https://crates.io/crates/ruzzy

A versatile and flexible fuzzy matcher in rust based on Levenshtein Distance

## Installation

```bash
cargo add ruzzy
```

## Usage

This crate performs fuzzy matching based on the Levenshtein distance a.k.a the edit distance. It means that the less string edits it take to transform string `A` to string `B`, the more similar `A` and `B`.

### `fuzzy_match`

The only function that this crate exposes is:

```rust
fn fuzzy_match<'a, Value: 'a>(needle: &'a String, haystack: &'a Vec<(String, Value)>, config: FuzzyConfig) -> Option<&'a Value>;
```

where:

* `needle` is the string to be matched.
* `haystack` is the set of key-value and the key part is what is being matched against `needle`
* `config` allows you to tune the matching process.

This function returns an `Option` that may wraps the corresponding value of the most similar key.

### `FuzzyConfig`

`FuzzyConfig` allows you to tune the matching process. Currently, these configurations are supported:

* `threshold`: If the edit distance is higher than this `threshold`, the key in the `haystack` is unacceptable and is not considered a match.
* `insertion_penalty`: The cost of a character insertion in the `needle` (by default: `1`).
* `deletion_penalty`: The cost of a character deletion in the `needle` (by default: `1`).
* `substitution_penalty`: The cost of a character substition (by default: `2`).
