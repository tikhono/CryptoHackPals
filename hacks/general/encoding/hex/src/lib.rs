#[cfg(test)]
mod tests {
    use hex::FromHex;

    #[test]
    fn capture_the_flag() {
        let vec = Vec::from_hex("63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d").unwrap() ;
        for b in vec {
            print!("{}", b as char);
        }
    }
}
