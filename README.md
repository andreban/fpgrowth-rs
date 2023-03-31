# fpgrowth-rs

An implementation of the [FP Growth][1] data mining algorithm in pure Rust.

## Example usage

```rust
use fpgrowth_rs::fp_growth;

fn main() {
    let transactions = vec![
        vec!["E", "A", "D", "B"],
        vec!["D", "A", "C", "E", "B"],
        vec!["C", "A", "B", "E"],
        vec!["B", "A", "D"],
        vec!["D"],
        vec!["D", "B"],
        vec!["A", "D", "E"],
        vec!["B", "C"],
    ];

    fp_growth(transactions, 3, |item_set, occurences| {
        println!("{:?}: {}", item_set, occurences)
    });
}
```

[![Build](https://github.com/andreban/fpgrowth-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/andreban/fpgrowth-rs/actions/workflows/rust.yml)

[1]: https://en.wikipedia.org/wiki/Association_rule_learning#FP-growth_algorithm
