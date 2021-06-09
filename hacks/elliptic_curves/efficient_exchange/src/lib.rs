#[cfg(test)]
mod tests {
    use aes::Aes128;
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Cbc};
    use curves_and_logs::Getter;
    use point_addition::Point;
    use scalar_multiplication::multiplication;
    use sha1::{Digest, Sha1};

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    #[test]
    fn capture_the_flag() {
        let x = 4726;
        let y_square: i64 = ((x * x * x) + 497 * x + 1768) % 9739;
        let mut y = 2;
        while (y * y) % 9739 != y_square {
            y += 1;
        }

        let qa1 = Point::Coord { x, y };

        let nb = 6534;
        let shared_secret = multiplication(&qa1, nb).x().unwrap().to_string();

        let mut hasher = Sha1::new();
        hasher.update(shared_secret);

        let key = hasher.finalize().to_vec();
        let key = key.get(0..16).unwrap();

        let iv = hex::decode("cd9da9f1c60925922377ea952afc212c").unwrap();

        let flag = hex::decode("febcbe3a3414a730b125931dccf912d2239f3e969c4334d95ed0ec86f6449ad8")
            .unwrap();

        let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
        let mut buf = flag;

        if cipher.decrypt(&mut buf).is_ok() {};
        println!("{}", String::from_utf8_lossy(&buf));
    }
    #[test]
    fn capture_theag() {
        let mut n = 1;
        let G = Point::Coord {
            x: 179210853392303317793440285562762725654,
            y: 105268671499942631758568591033409611165,
        };

        // while multiplication() {}
    }
}
