#[cfg(test)]
mod tests {
    #[test]
    fn capture_the_flag() {
        let ascii: [u8; 23] = [
            99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52,
            98, 108, 51, 125,
        ];
        for iter in ascii.iter() {
            print!("{}", *iter as char);
        }
    }
    #[test]
    fn test_ascii_crate() {
        use ascii::IntoAsciiString;
        let ascii_hello: Vec<u8> = [104, 101, 108, 108, 111].to_vec();
        let string_hello = ascii_hello.into_ascii_string().unwrap();
        assert_eq!(string_hello, "hello");
    }
}
