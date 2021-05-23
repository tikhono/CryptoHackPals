#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;
    use num::Num;

    #[test]
    fn capture_the_flag() {
        let n = BigInt::from_str_radix(
            "742449129124467073921545687640895127535705902454369756401331",
            10,
        )
        .unwrap();
        let p = BigInt::from_str_radix("752708788837165590355094155871", 10).unwrap();
        let q = BigInt::from_str_radix("986369682585281993933185289261", 10).unwrap();
        let phi: BigInt = (&p - 1) * (&q - 1);
        let e = BigInt::from(3);
        let ct = BigInt::from_str_radix(
            "39207274348578481322317340648475596807303160111338236677373",
            10,
        )
        .unwrap();

        let d = mod_inverse(e, phi).unwrap();
        let pt = ct.modpow(&d, &n);
        println!("{}", String::from_utf8_lossy(&pt.to_signed_bytes_be()));
    }
}
