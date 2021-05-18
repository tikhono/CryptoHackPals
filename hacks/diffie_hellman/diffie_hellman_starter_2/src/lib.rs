#![feature(step_trait)]

use num::Integer;
use std::fmt::Debug;

pub fn order<T: num::PrimInt + Integer + std::ops::AddAssign + Copy>(g: T, p: T) -> T
where
    u32: From<T>,
    T: std::iter::Step,
{
    for i in T::zero()..p {
        if g.pow(i.into()) % p.clone() == g {
            return i;
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use crate::order;

    #[test]
    fn capture_the_flag() {
        assert_eq!(order::<u16>(209, 991), 1);
    }
}
