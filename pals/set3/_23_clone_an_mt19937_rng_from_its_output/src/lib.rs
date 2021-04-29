use _21_implement_the_mt19937_mersenne_twister_rng::MT19937;

pub fn untemper(mut y: u64) -> u64 {
    y ^= y >> 18;
    y ^= (y << 15) & 0xefc60000;
    for _ in 0..7 {
        y ^= (y << 7) & 0x9d2c5680;
    }
    for _ in 0..3 {
        y ^= y >> 11;
    }
    y
}
pub fn clone_mt(mt: &mut MT19937) -> MT19937 {
    let mut mt_clone = MT19937::init(0);
    for i in 0..624 {
        mt_clone.state[i] = untemper(mt.random());
    }
    mt_clone
}

#[cfg(test)]
mod tests {
    use crate::clone_mt;
    use _21_implement_the_mt19937_mersenne_twister_rng::MT19937;

    #[test]
    fn test_clone_mt() {
        let mut mt = MT19937::init(5489);

        let mut mt_clone = clone_mt(&mut mt);
        for _ in 0..2000 {
            assert_eq!(mt_clone.random(), mt.random());
        }
    }
}
