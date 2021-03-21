#[cfg(test)]
mod tests {
    use _02_fixed_xor::fixed_xor;
    use ascii::IntoAsciiString;

    #[test]
    fn capture_the_flag() {
        let key1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
        let key2_xor_key3 = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";
        let flag_xor_key1_xor_key2_xor_key3 =
            "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf";
        let flag = hex::decode(fixed_xor(
            &*fixed_xor(flag_xor_key1_xor_key2_xor_key3, key2_xor_key3),
            key1,
        ))
        .unwrap()
        .into_ascii_string()
        .unwrap();
        println!("{:?}", flag);
    }
}
