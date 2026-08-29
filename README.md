# p-mod-q-calculator

A Rust port of the **p ≡ 1 (mod q)** framework calculator. Computes the number of representations **N = pA + qB** in strict **O(1)** time using digital roots as starting points.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
p-mod-q-calculator = "0.1.0"
num-bigint = "0.4"
```

## Example

```rust
use num_bigint::BigInt;
use p_mod_q_calculator::{count_representations, get_representations};

fn main() {
    let n = BigInt::parse_bytes(b"1991", 10).unwrap();
    let p = BigInt::parse_bytes(b"19", 10).unwrap();
    let q = BigInt::parse_bytes(b"9", 10).unwrap();

    if let Some(count) = count_representations(&n, &p, &q) {
        println!("Aantal representaties: {}", count);
        
        let first_reps = get_representations(&n, &p, &q, 9);
        for (i, rep) in first_reps.iter().enumerate() {
            println!("  [{}] N = {} * {} + {} * {}", i + 1, p, rep.a, q, rep.b);
        }
    }
}
