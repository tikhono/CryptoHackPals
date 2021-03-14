#[cfg(test)]
mod tests {
    #[test]
    fn capture_the_flag() {
        let ascii: [u8; 23] = [
            99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52,
            98, 108, 51, 125,
        ];
        for i in 0..ascii.len() {
            print!("{}", ascii[i] as char);
        }
    }
}

