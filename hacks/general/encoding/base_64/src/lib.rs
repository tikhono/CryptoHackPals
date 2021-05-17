#[cfg(test)]
mod tests {
    #[test]
    fn capture_the_flag() {
        let bytes = hex::decode("72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf").unwrap();
        println!("{}", base64::encode(bytes));
    }
}
