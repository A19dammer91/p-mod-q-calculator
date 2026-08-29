use num_bigint::BigInt;
use num_traits::{Zero, One};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Representation {
    pub a: BigInt,
    pub b: BigInt,
}

/// Computes the number of representations N = pA + qB in strict O(1) time within the p ≡ 1 (mod q) framework.
///
/// # Conditions
/// * `p % q == 1`
/// * `p`, `q`, and `n` must be greater than 0.
///
/// # Returns
/// Returns `None` if the input does not satisfy the framework conditions (`p % q != 1`).
/// Otherwise, returns the total `BigInt` count of valid representations.
pub fn count_representations(n: &BigInt, p: &BigInt, q: &BigInt) -> Option<BigInt> {
    if p <= &BigInt::zero() || q <= &BigInt::zero() || n <= &BigInt::zero() {
        return None;
    }
    
    // Check the framework condition: p ≡ 1 (mod q)
    if (p % q) != BigInt::one() {
        return None;
    }

    let a0 = n % q;
    let max_p_term = n / p;

    if max_p_term < a0 {
        return Some(BigInt::zero());
    }

    let count = ((max_p_term - a0) / q) + BigInt::one();
    Some(count)
}

/// Generates the first `limit` representations for the equation N = pA + qB.
/// 
/// The starting value for A begins at A₀ = N mod q and increases by steps of q.
pub fn get_representations(n: &BigInt, p: &BigInt, q: &BigInt, limit: usize) -> Vec<Representation> {
    let mut results = Vec::new();
    
    if count_representations(n, p, q).unwrap_or_else(BigInt::zero) == BigInt::zero() {
        return results;
    }

    let mut a = n % q;
    let max_a = n / p;

    while a <= max_a && results.len() < limit {
        let rem = n - (&a * p);
        if &rem % q == BigInt::zero() {
            let b = rem / q;
            results.push(Representation { a: a.clone(), b });
        }
        a += q;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::FromPrimitive;

    #[test]
    fn test_example_case() {
        let n = BigInt::from_u32(1991).unwrap();
        let p = BigInt::from_u32(19).unwrap();
        let q = BigInt::from_u32(9).unwrap();

        let count = count_representations(&n, &p, &q).unwrap();
        assert_eq!(count, BigInt::from_u32(12).unwrap());

        let reps = get_representations(&n, &p, &q, 2);
        assert_eq!(reps.len(), 2);
        // First representation: A0 = 1991 mod 9 = 2. B = (1991 - 19*2) / 9 = 217
        assert_eq!(reps[0].a, BigInt::from_u32(2).unwrap());
        assert_eq!(reps[0].b, BigInt::from_u32(217).unwrap());
    }
}
