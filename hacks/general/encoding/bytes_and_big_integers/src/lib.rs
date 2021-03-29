pub fn decode(str: &str) -> String {
    use ascii::IntoAsciiString;
    use uint::construct_uint;

    construct_uint! {pub struct U1024(16);}

    let num = U1024::from_dec_str(str).unwrap();
    let bytes: &mut [u8] = &mut [0; 16 * 8];
    num.to_big_endian(bytes);
    bytes
        .into_ascii_string()
        .unwrap()
        .to_string()
        .trim_start_matches("\u{0}")
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::decode;
    use ascii::IntoAsciiString;
    use uint::construct_uint;

    #[test]
    fn capture_the_flag() {
        let string =
            "11515195063862318899931685488813747395775516287289682636499965282714637259206269";
        println!("{:?}", decode(string));
    }
}
