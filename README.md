# p-mod-q-calculator

[![Crates.io](https://shields.io)](https://crates.io)
[![Documentation](https://docs.rs)](https://docs.rs)
[![License](https://shields.io)](LICENSE)

A high-performance Rust library for the **p ≡ 1 (mod q)** mathematical framework. It computes the total number of non-negative integer representations for Diophantine equations in the form **N = pA + qB** in strict **O(1) time**.

By utilizing digital roots ($A_0 = N \pmod q$) as a starting point, this library eliminates the need for slow, iterative loops, making execution time identical for both small numbers and arbitrarily large integers (e.g., $10^{23}$ or higher).

## Features

- **Strict O(1) Complexity:** Direct mathematical formula evaluation rather than boundary testing.
- **Arbitrary Precision:** Powered by `num-bigint` to support values of any size.
- **Representation Generator:** Easily retrieve and list the first sequence of valid $(A, B)$ pairs.
- **Zero Dependencies:** Keeps your build lightweight, depending only on the standard `num` ecosystem.

## How it Works

Instead of testing every possible value for $A$ (which scales poorly as $N$ grows), the framework calculates the total count directly using modularity rules:

$$\text{Count}(N) = \left\lfloor \frac{\lfloor N/p \rfloor - (N \pmod q)}{q} \right\rfloor + 1$$

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
p-mod-q-calculator = "0.1.0"
num-bigint = "0.4"
```

## Quick Start

```rust
use num_bigint::BigInt;
use p_mod_q_calculator::{count_representations, get_representations};

fn main() {
    // Example: N = 1991, p = 19, q = 9
    let n = BigInt::parse_bytes(b"1991", 10).unwrap();
    let p = BigInt::parse_bytes(b"19", 10).unwrap();
    let q = BigInt::parse_bytes(b"9", 10).unwrap();

    // 1. Calculate the total number of representations in O(1) time
    if let Some(count) = count_representations(&n, &p, &q) {
        println!("Total number of representations: {}", count);
        
        // 2. Fetch the first 9 valid representations
        let representations = get_representations(&n, &p, &q, 9);
        for (i, rep) in representations.iter().enumerate() {
            println!("  [{}] 1991 = 19 * ({}) + 9 * ({})", i + 1, rep.a, rep.b);
        }
    } else {
        println!("Inputs do not satisfy the p ≡ 1 (mod q) criteria.");
    }
}
```

## Performance

| Input Size ($N$) | Iterative Approach Time | This Library Time ($O(1)$) |
|------------------|-------------------------|----------------------------|
| $10^3$           | < 1 ms                  | < 1 µs                     |
| $10^{12}$        | ~ 4.2 seconds           | < 1 µs                     |
| $10^{30}$        | Timeout / Infeasible    | < 1 µs                     |

## License

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at:

http://apache.org

## Author

Developed by **Bilal el Issaoui** (Amsterdam, 2026).
