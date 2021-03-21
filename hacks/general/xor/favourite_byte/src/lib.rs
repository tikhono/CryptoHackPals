#[cfg(test)]
mod tests {
    use _02_fixed_xor::fixed_xor;
    use ascii::IntoAsciiString;

    #[test]
    fn capture_the_flag() {
        let flag = "73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d";
        let flag_first_char = &flag[..2];
        let favourite_byte = fixed_xor(flag_first_char, &*hex::encode("c"));
        let flag = fixed_xor(
            flag,
            &*favourite_byte.repeat(flag.len() / favourite_byte.len()),
        );
        let flag = hex::decode(flag).unwrap().into_ascii_string().unwrap();
        println!("{:?}", flag);
    }
}
