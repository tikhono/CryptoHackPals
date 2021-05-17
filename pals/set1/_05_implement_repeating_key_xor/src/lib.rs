pub fn repeating_key_xor(plaintext: String, key: String) -> String {
    use _02_fixed_xor::fixed_xor;

    let repeated_key = &(key.repeat(plaintext.len() / key.len())
        + key.get(0..plaintext.len() % key.len()).unwrap());
    fixed_xor(&hex::encode(plaintext), &hex::encode(repeated_key))
}

#[cfg(test)]
mod tests {
    use crate::repeating_key_xor;

    #[test]
    fn test_repeating_key_xor() {
        assert_eq!(
            repeating_key_xor(
                "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"
                    .to_string(),
                "ICE".to_string()
            ),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
                .to_string()
        );
    }
}
