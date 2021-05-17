#[cfg(test)]
mod tests {
    use num::Integer;

    #[test]
    fn capture_the_flag() {
        assert_eq!(1, 26513.gcd(&32321));
        println!("{:?}", 26513.gcd(&32321));
    }
}
