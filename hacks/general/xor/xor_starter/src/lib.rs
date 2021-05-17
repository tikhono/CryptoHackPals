#[cfg(test)]
mod tests {
    #[test]
    fn capture_the_flag() {
        let string = "label";
        for byte in string.as_bytes().to_vec() {
            print!("{}", (byte ^ 13) as char);
        }
    }
}
