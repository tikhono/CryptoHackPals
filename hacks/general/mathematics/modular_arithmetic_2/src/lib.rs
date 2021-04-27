#[cfg(test)]
mod tests {
    use num::Integer;

    #[test]
    fn capture_the_flag() {
        //coprime
        assert_eq!(1, 273246787654u64.gcd(&65537));
        println!("{:?}", 273246787654u64.gcd(&65537));
    }
}
