use num_bigint::{BigUint, ToBigUint};

/// Computes the nth Fibonacci number using BigInt.
pub fn fibonacci(n: u32) -> BigUint {
    let mut a = 0.to_biguint().unwrap();
    let mut b = 1.to_biguint().unwrap();

    if n == 0 {
        a
    } else if n == 1 {
        b
    } else {
        for _ in 2..=n {
            let next = &a + &b;
            a = b;
            b = next;
        }

        b
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(
            fibonacci(100),
            BigUint::from_str("354224848179261915075").unwrap()
        );
    }
}