use hex;
use num::bigint::BigUint;
use num::Num;
use rand::{self, Rng};
use sha2::{Digest, Sha256};

fn forge_sig(msg: &[u8], mod_size: u32) -> BigUint {
    let two = BigUint::from(2u32);
    let pos = mod_size - 8 * 50;

    let mut hasher = Sha256::new();
    hasher.update(msg);
    let result = hasher.finalize();
    let hash = hex::encode(result);
    let hash_len_str = format!("{:02x}", hash.len() / 2);

    let d_str: String = ["00", &*hash_len_str, &*hash].concat();
    let d = BigUint::from_str_radix(&d_str, 16).unwrap();

    let cube =
        &two.pow(mod_size - 7) - &two.pow(pos) + &d * &two.pow(pos - 8 * (hash.len() as u32 - 30));
    cube.nth_root(3)
}

fn verify(message: &[u8], sig: &BigUint) {
    let cube = sig.pow(3);
    let cube_str = cube.to_str_radix(16);

    assert!(cube_str.starts_with("1ff"));

    let hash_idx = 4 + cube_str.find("ff00").unwrap();
    let hash_len = 2 * usize::from_str_radix(&cube_str[hash_idx..(hash_idx + 2)], 16).unwrap();
    let hash1 = &cube_str[(hash_idx + 2)..(hash_idx + 2 + hash_len)];

    let mut hasher = Sha256::new();
    hasher.update(message);
    let result = hasher.finalize();
    let hash2 = hex::encode(result);
    assert_eq!(hash1[..32], hash2[..32]);
}
#[test]
fn capture_the_flag() {
    let mut rng = rand::thread_rng();

    let modulus_size = 8 * rng.gen_range(256, 1250);

    let msg = b"hi mom";
    let sig = forge_sig(msg, modulus_size);

    verify(msg, &sig);
}
