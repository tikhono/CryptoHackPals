pub fn fixed_xor(l: &str, r: &str) -> String {
    let mut l = hex::decode(l).unwrap();
    let r = hex::decode(r).unwrap();
    let zipped = l.iter_mut().zip(r.iter());
    for (l_byte, r_byte) in zipped {
        *l_byte ^= *r_byte;
    }
    hex::encode(l)
}

#[cfg(test)]
mod tests {
    use crate::fixed_xor;

    #[test]
    fn capture_the_flag() {
        println!(
            "{}",
            fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            )
        );
    }
    #[test]
    fn test_0() {
        assert_eq!("00", fixed_xor("00", "00"));
    }
    #[test]
    fn test_1() {
        assert_eq!("ff", fixed_xor("00", "ff"));
    }
    #[test]
    fn test_2() {
        assert_eq!("ff", fixed_xor("ff", "00"));
    }
    #[test]
    fn test_3() {
        assert_eq!("00", fixed_xor("ff", "ff"));
    }
}
