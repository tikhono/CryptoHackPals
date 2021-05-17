#[cfg(test)]
mod tests {
    use ascii::IntoAsciiString;
    use hex::FromHex;

    #[test]
    fn capture_the_flag() {
        let vec = Vec::from_hex("63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d").unwrap() ;
        for b in vec {
            print!("{}", b as char);
        }
    }
    #[test]
    fn capture() {
        let vec = hex::decode("646566696e696e675f636f6e6669726d6174696f6e5f6c62").unwrap();
        print!("{}", vec.into_ascii_string().unwrap());
    }
}
