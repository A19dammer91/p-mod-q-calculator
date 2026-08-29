use num_bigint::BigInt;
use num_traits::{Zero, One};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Representation {
    pub a: BigInt,
    pub b: BigInt,
}

/// Berekent het aantal representaties N = pA + qB in strict O(1) tijd binnen het p ≡ 1 (mod q) framework.
///
/// # Voorwaarden
/// * `p % q == 1`
/// * `p`, `q` en `n` moeten groter zijn dan 0.
///
/// # Returns
/// Geeft `None` als de invoer niet voldoet aan de framework-voorwaarden (`p % q != 1`).
/// Geeft anders de totale `BigInt` telling van geldige representaties terug.
pub fn count_representations(n: &BigInt, p: &BigInt, q: &BigInt) -> Option<BigInt> {
    if p <= &BigInt::zero() || q <= &BigInt::zero() || n <= &BigInt::zero() {
        return None;
    }
    
    // Controleer de framework voorwaarde: p ≡ 1 (mod q)
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

/// Genereert de eerste `limit` representaties voor de vergelijking N = pA + qB.
/// 
/// De startwaarde voor A begint bij A₀ = N mod q en stijgt telkens met stappen van q.
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
        // Eerste representatie: A0 = 1991 mod 9 = 2. B = (1991 - 19*2) / 9 = 217
        assert_eq!(reps[0].a, BigInt::from_u32(2).unwrap());
        assert_eq!(reps[0].b, BigInt::from_u32(217).unwrap());
    }
}
