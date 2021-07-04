#![allow(dead_code)]
use cargo_snippet::snippet;

pub mod binary_search;
pub mod fenwick_tree;
pub mod modint;
pub mod ncr;
pub mod union_find;
pub mod segment_tree;

#[snippet("io")]
pub mod io {
    use std::io;

    pub fn read<T>() -> Vec<T>
        where T:
        std::str::FromStr,
        T::Err: std::fmt::Debug {

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }

    pub fn read_one<T>() -> T
        where T:
        std::str::FromStr,
        T::Err: std::fmt::Debug {

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
    }
}

#[snippet("gcd")]
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a }
    else { gcd(b, a % b) }
}

#[snippet("lcm")]
#[snippet(include = "gcd")]
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

// returns whether or not it can be divided.
// if possible, it returns Some(divider) or None.
#[snippet("divide")]
fn divide(n: usize) -> Option<usize> {
    for i in 2..(n as f64).sqrt().ceil() as usize + 1 {
        if n % i == 0 {
            return Some(i);
        }
    }

    None
}

/// returns vec of prime factors of n
/// it also contains 1 and n itself
#[snippet("prime_factor")]
#[snippet(include = "divide")]
pub fn prime_factor(n: usize) -> Vec<usize> {
    let mut x = n;
    let mut factors = Vec::new();
    while let Some(f) = divide(x) {
        x /= f;
        factors.push(f);
    }
    factors.push(x);

    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prime_factor() {
        assert_eq!(prime_factor(12), vec![2, 2, 3]);
        assert_eq!(prime_factor(57), vec![3, 19]);
    }
}
