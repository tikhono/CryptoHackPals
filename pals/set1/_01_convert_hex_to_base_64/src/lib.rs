#[cfg(test)]
mod tests {
    #[test]
    fn capture_the_flag() {
        let bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let base64 = base64::encode(bytes);
        println!("{}", base64);
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            base64
        );
    }
}
