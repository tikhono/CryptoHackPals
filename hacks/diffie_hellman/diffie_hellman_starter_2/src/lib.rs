use num::bigint::BigInt;

pub fn order(g: BigInt, p: u32) -> u32 {
    for i in 2..p {
        if g.modpow(&BigInt::from(i), &BigInt::from(p)) == g {
            return i;
        }
    }
    p
}

pub fn generator(p: u32) -> u32 {
    for g in 2..p {
        //Generator couldn't be 0 or 1
        if order(BigInt::from(g), p) == p {
            return g;
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use crate::{generator, order};
    use num::bigint::BigInt;

    #[test]
    fn capture_the_flag() {
        assert_eq!(generator(28151), 7);
    }

    #[test]
    fn test_order() {
        assert_eq!(order(BigInt::from(209), 991), 67);
    }
}
