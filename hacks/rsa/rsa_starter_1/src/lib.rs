#[cfg(test)]
mod tests {
    use num::bigint::BigInt;

    #[test]
    fn capture_the_flag() {
        let a = BigInt::from(101);
        let b = BigInt::from(17);
        let p = BigInt::from(22663);
        println!("{}", a.modpow(&b, &p));
    }
}
