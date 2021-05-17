pub struct MT19937 {
    pub index: u64,
    pub state: Vec<u64>,
}

impl MT19937 {
    pub fn init(seed: u64) -> MT19937 {
        let mut mt = MT19937 {
            index: 0,
            state: vec![0],
        };
        mt.index = 624;
        mt.state = Vec::from([0; 624]);
        mt.state[0] = seed & 4294967295;
        for i in 1..624 {
            mt.state[i] = ((1812433253 * (mt.state[(i - 1)] ^ (mt.state[(i - 1)] >> 30)))
                + i as u64)
                & 4294967295;
        }
        mt
    }

    pub fn temper(mut y: u64) -> u64 {
        y ^= y >> 11;
        y ^= (y << 7) & 2636928640;
        y ^= (y << 15) & 4022730752;
        y ^= y >> 18;
        y
    }

    pub fn random(&mut self) -> u64 {
        if self.index >= 624 {
            self._twist();
        }
        let y = MT19937::temper(self.state[self.index as usize]);
        self.index += 1;
        y
    }

    fn _twist(&mut self) {
        for i in 0..624 {
            let y = (self.state[i] & 2147483648) + (self.state[((i + 1) % 624)] & 2147483647);
            self.state[i] = self.state[((i + 397) % 624)] ^ (y >> 1);
            if (y % 2) != 0 {
                self.state[i] ^= 2567483615;
            }
        }
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::MT19937;

    #[test]
    fn test_seed_5489() {
        let mut x = MT19937::init(5489);
        let mut y = MT19937::init(5489);

        for _ in 0..100 {
            assert_eq!(x.random(), y.random());
            println!("{:?} : {:?}", x.random(), y.random());
        }
    }
}
