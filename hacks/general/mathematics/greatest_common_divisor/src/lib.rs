#[cfg(test)]
mod tests {
    use num::Integer;

    #[test]
    fn capture_the_flag() {
        assert_eq!(1512, 66528.gcd(&52920));
        println!("{:?}", 66528.gcd(&52920));
    }
}
