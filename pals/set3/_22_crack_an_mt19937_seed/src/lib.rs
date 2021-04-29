use _21_implement_the_mt19937_mersenne_twister_rng::MT19937;
use rand::Rng;
use std::time::SystemTime;
use std::{thread, time};

const MIN_SLEEP_TIME: u64 = 40;
const MAX_SLEEP_TIME: u64 = 100;

pub fn sleep_mersenne() -> u64 {
    let mut sys_rng = rand::thread_rng();

    thread::sleep(time::Duration::from_secs(
        sys_rng.gen_range(MIN_SLEEP_TIME, MAX_SLEEP_TIME),
    ));

    //let seed = time::Instant::now().elapsed().as_secs();

    let seed = time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    thread::sleep(time::Duration::from_secs(
        sys_rng.gen_range(MIN_SLEEP_TIME, MAX_SLEEP_TIME),
    ));

    let mut x = MT19937::init(seed);

    x.random()
}

pub fn crack_seed(r: u64) -> u64 {
    let mut seed = time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    while MT19937::init(seed).random() != r && seed > 0 {
        seed -= 1;
    }
    seed
}

#[cfg(test)]
mod tests {
    use crate::{crack_seed, sleep_mersenne};

    #[test]
    fn test_seed_crack() {
        let r = sleep_mersenne();
        let seed = crack_seed(r);
        println!("{:?} ", seed);
    }
}
