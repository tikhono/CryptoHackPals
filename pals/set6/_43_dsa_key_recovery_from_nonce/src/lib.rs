#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::{BigInt, Sign};
    use num::{Num, Signed};
    use rayon::iter::ParallelIterator;
    use rayon::prelude::IntoParallelIterator;
    use sha1::{Digest, Sha1};

    #[test]
    fn capture_the_flag() {
        let p = BigInt::from_str_radix(
            "800000000000000089e1855218a0e7dac38136ffafa72eda7859f2171e25e65eac698c1702578b07dc2a1076da241c76c62d374d8389ea5aeffd3226a0530cc565f3bf6b50929139ebeac04f48c3c84afb796d61e5a4f9a8fda812ab59494232c7d2b4deb50aa18ee9e132bfa85ac4374d7f9091abc3d015efc871a584471bb1",
            16,
        ).unwrap();

        let q = BigInt::from_str_radix("f4f47f05794b256174bba6e9b396a7707e563c5b", 16).unwrap();

        let g = BigInt::from_str_radix(
            "5958c9d3898b224b12672c0b98e06c60df923cb8bc999d119458fef538b8fa4046c8db53039db620c094c9fa077ef389b5322a559946a71903f990f1f7e0e025e2d7f7cf494aff1a0470f5b64c36b625a097f1651fe775323556fe00b3608c887892878480e99041be601a62166ca6894bdd41a7054ec89f756ba9fc95302291", 
            16,
        ).unwrap();

        let pubkey = BigInt::from_str_radix(
            "84ad4719d044495496a3201c8ff484feb45b962e7302e56a392aee4abab3e4bdebf2955b4736012f21a08084056b19bcd7fee56048e004e44984e2f411788efdc837a0d2e5abb7b555039fd243ac01f0fb2ed1dec568280ce678e931868d23eb095fde9d3779191b8c0299d6e07bbb283e6633451e535c45513b2d33c99ea17",
            16,
        ).unwrap();

        let r =
            BigInt::from_str_radix("548099063082341131477253921760299949438196259240", 10).unwrap();

        let s =
            BigInt::from_str_radix("857042759984254168557880549501802188789837994940", 10).unwrap();

        let hash =BigInt::from_bytes_be(Sign::Plus,&Sha1::new().chain("For those that envy a MC it can be hazardous to your health\nSo be friendly, a matter of life and death, just like a etch-a-sketch\n").finalize()[..]);

        let range = 1..=0xFFFF;
        let k = range.into_par_iter().find_any(|&k| {
            let mut private = ((&s * BigInt::from(k)) - &hash).abs();
            private = (private * mod_inverse(r.clone(), q.clone()).unwrap()) % &q;
            pubkey == g.modpow(&private, &p)
        });
        println!("{:?}", k);
    }
}
