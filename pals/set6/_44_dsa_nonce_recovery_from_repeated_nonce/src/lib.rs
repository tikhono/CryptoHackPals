#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;
    use num::Num;
    use sha1::{Digest, Sha1};

    #[test]
    fn capture_the_flag() {
        let q = BigInt::from_str_radix("f4f47f05794b256174bba6e9b396a7707e563c5b", 16).unwrap();

        let r = BigInt::from_str_radix("1105520928110492191417703162650245113664610474875", 10)
            .unwrap();
        let s1 = BigInt::from_str_radix("1267396447369736888040262262183731677867615804316", 10)
            .unwrap();
        let m1 = BigInt::from_str_radix("a4db3de27e2db3e5ef085ced2bced91b82e0df19", 16).unwrap();

        let s2 = BigInt::from_str_radix("1021643638653719618255840562522049391608552714967", 10)
            .unwrap();
        let m2 = BigInt::from_str_radix("d22804c4899b522b23eda34d2137cd8cc22b9ce8", 16).unwrap();

        let k = (((&m2 - &m1) % &q) * mod_inverse((&s2 - &s1) % &q, q.clone()).unwrap()) % &q;

        let res = ((((&s1 * &k) - &m1) % &q) * mod_inverse(r, q.clone()).unwrap()) % &q;

        assert_eq!(
            hex::decode("ca8f6f7c66fa362d40760d135b763eb8527d3d52").unwrap(),
            Sha1::new()
                .chain(res.to_str_radix(16))
                .finalize()
                .as_slice(),
        );
    }
}
