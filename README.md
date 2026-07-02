# p-mod-q-calculator

Calculator for the **p ≡ 1 (mod q)** framework. Computes the number of representations **N = pA + qB** in strict **O(1)** time using digital roots (dr) as starting point **A₀ = N mod q**. Supports arbitrarily large numbers via BigInt.

---

## What it does

Given a number **N** and coefficients **p, q** where **p ≡ 1 (mod q)**, this calculator:

1. Computes the **digital root** of N as the starting point
2. Calculates the **total number** of representations N = pA + qB in O(1) time
3. Displays the **first 9 representations**
4. Shows the **digital root reduction chain**

## Why O(1)?

Instead of testing every possible A value (slow for large N), the formula computes the count directly:

```
R₂(N) = ⌊(⌊N/p⌋ − (N mod q)) / q⌋ + 1
```

This means **N = 10²³** takes the same time as **N = 100**.

## How to use

1. Open the [live calculator](https://a19dammer91.github.io/-p-mod-q-calculator/)
2. Enter coefficients **p** and **q** (default: 19, 9)
3. Enter any number **N** (supports arbitrarily large integers)
4. Click **COMPUTE REPRESENTATIONS**

## Example

| Input | Output |
|-------|--------|
| N = 1991, p = 19, q = 9 | 12 representations |
| N = 7737736464664643332445667778, p = 19, q = 9 | ~45 quintillion representations |

## Tech

- Pure HTML/CSS/JavaScript — no dependencies
- **BigInt** for arbitrary-precision arithmetic
- Works on any device with a browser

## Author

**Bilal el Issaoui** · Amsterdam · 2026
