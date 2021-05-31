use num_bigint::BigUint;
use num_integer::Integer;
use num_primes::{Factorization, Verification};
use num_traits::One;

//This naive solution runs infinitely long, so I used http://factordb.com
//Also this solution is a fixed version of the num_primes::Factorization::prime_factor
//This method fails to manage upper range with
//        let n_sqrt = n.sqrt().to_usize().unwrap();
pub fn prime_factor(mut n: BigUint) -> Option<BigUint> {
    if Verification::is_prime(&n) {
        return Some(n);
    }

    let two = BigUint::one() + BigUint::one();
    let three = BigUint::one() + BigUint::one() + BigUint::one();

    while n.is_even() {
        n /= &two;
    }

    let n_sqrt = n.sqrt();
    let mut i = three;
    while i < n_sqrt {
        while n.divides(&i.clone()) {
            n /= i.clone();
        }
        i += two.clone();
    }

    if n > two {
        Some(n)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::prime_factor;
    use num_bigint::BigUint;

    #[test]
    #[ignore]
    fn capture_the_flag() {
        let n = BigUint::parse_bytes(b"510143758735509025530880200653196460532653147", 10).unwrap();

        println!("{}", prime_factor(n).unwrap());
    }
    #[test]
    fn test_17_11() {
        let p = BigUint::from(11u8);
        let q = BigUint::from(17u8);

        assert_eq!(p.clone(), prime_factor(p * q).unwrap());
    }
}
