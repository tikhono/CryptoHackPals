#[cfg(test)]
mod tests {
    use aes::Aes128;
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Cbc};
    use sha1::{Digest, Sha1};

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    #[test]
    fn capture_the_flag() {
        /*
        sage: p = 310717010502520989590157367261876774703
        sage: a = 2
        sage: b = 3
        sage: E = EllipticCurve(GF(p), [a, b])
        sage: P = E(179210853392303317793440285562762725654,105268671499942631758568591033409611165 )
        sage: Q = E(280810182131414898730378982766101210916, 291506490768054478159835604632710368904)
        sage: discrete_log(Q,P,operation='+')
        47836431801801373761601790722388100620
        sage: Pub = E(272640099140026426377756188075937988094, 51062462309521034358726608268084433317)
        sage: S = Pub * 47836431801801373761601790722388100620
        sage: S
        (171172176587165701252669133307091694084 : 188106434727500221954651940996276684440 : 1)
        */

        let shared_secret = "171172176587165701252669133307091694084";

        let mut hasher = Sha1::new();
        hasher.update(shared_secret);

        let key = hasher.finalize().to_vec();
        let key = key.get(0..16).unwrap();

        let iv = hex::decode("07e2628b590095a5e332d397b8a59aa7").unwrap();

        let mut flag = hex::decode("8220b7c47b36777a737f5ef9caa2814cf20c1c1ef496ec21a9b4833da24a008d0870d3ac3a6ad80065c138a2ed6136af")
            .unwrap();

        let cipher = Aes128Cbc::new_from_slices(key, &iv).unwrap();
        // let mut buf = flag;

        cipher.decrypt(&mut flag).expect("Bad padding or decrypt");
        println!("{}", String::from_utf8_lossy(&flag));
    }
}
