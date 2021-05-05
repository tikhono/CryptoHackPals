use _21_implement_the_mt19937_mersenne_twister_rng::MT19937;

fn mt_crypt(plaintext: &[u8], seed: u64) -> Vec<u8> {
    let mut mt = MT19937::init(seed);
    let mut ciphertext = Vec::new();
    for m in plaintext.iter() {
        ciphertext.push((*m as u64 ^ mt.random() & 0xFF) as u8);
    }
    ciphertext
}

fn mt_crack(ciphertext: &[u8]) -> Option<u64> {
    let plaintext = b"A".repeat(ciphertext.len());

    for seed in 0..0xFFFF {
        if ciphertext.get(ciphertext.len() - 14..)
            == mt_crypt(&plaintext, seed).get(plaintext.len() - 14..)
        {
            return Some(seed);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{mt_crack, mt_crypt};
    use rand::{thread_rng, Rng};

    #[test]
    fn it_works() {
        let mut rng = thread_rng();

        let seed: u16 = rand::random();

        let mut plaintext: Vec<u8> = vec![0; rng.gen_range(0, 100)]
            .into_iter()
            .map(|_| rand::random())
            .collect();
        plaintext.append(&mut b"A".repeat(14));

        let ciphertext = mt_crypt(&plaintext, seed as u64);

        let found_seed = mt_crack(&ciphertext).expect("Failed to retrieve seed");

        assert_eq!(found_seed, seed as u64);
    }
}
