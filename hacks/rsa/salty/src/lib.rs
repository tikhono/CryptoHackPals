#[cfg(test)]
mod tests {
    use num::{BigInt, Num};

    #[test]
    fn capture_the_flag() {
        let ct = BigInt::from_str_radix(
            "44981230718212183604274785925793145442655465025264554046028251311164494127485",
            10,
        )
        .unwrap();
        println!("{}", String::from_utf8_lossy(&ct.to_signed_bytes_be()));
    }
}
