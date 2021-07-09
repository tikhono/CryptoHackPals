use std::thread::sleep;
use std::time::Duration;

pub fn insecure_compare(hash1: &[u8], hash2: &[u8]) -> bool {
    let duration = Duration::from_nanos(500000);
    for (b1, b2) in hash1.iter().zip(hash2.iter()) {
        if b1 != b2 {
            return false;
        }
        sleep(duration);
    }
    if hash1.len() != hash2.len() {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::insecure_compare;
    use hmac::{Hmac, Mac, NewMac};
    use sha2::Sha256;
    use std::time::{Duration, Instant};

    type HmacSha256 = Hmac<Sha256>;

    #[test]
    fn capture_the_flag() {
        let mut signature = HmacSha256::new_from_slice(b"my secret and secure key")
            .expect("HMAC can take key of any size");
        signature.update(b"input message");

        let signature = signature.finalize();

        // To get underlying array use `into_bytes` method, but be careful, since
        // incorrect use of the code value may permit timing attacks which defeat
        // the security provided by the `Output`, and its exactly what we need.
        let signature = signature.into_bytes();
        let signature = signature.as_slice();

        let mut guess = Vec::<u8>::new();
        'outer: for _keylen in 0..32 {
            let mut time_of_each_byte_guess = Vec::new();
            for byte in 0u8..=255 {
                let mut median_time = Duration::new(0, 0);
                let iterations = 5;
                for _ in 0..iterations {
                    let mut current_guess = guess.clone();
                    current_guess.push(byte);
                    let start = Instant::now();
                    if insecure_compare(&current_guess, signature) {
                        guess.push(byte);
                        break 'outer;
                    }
                    let finish = Instant::now();
                    median_time += finish - start;
                }
                time_of_each_byte_guess.push((median_time, byte));
            }
            let max_time = time_of_each_byte_guess.iter().max().unwrap();
            guess.push(max_time.1);
        }
        assert_eq!(&guess, &signature)
    }
}
