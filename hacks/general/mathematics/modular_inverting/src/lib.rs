#[cfg(test)]
mod tests {

    #[test]
    fn capture_the_flag() {
        let a = 3u32;
        let p = 13u32;

        assert_eq!(9, a.pow(p - 2) % p);
        println!("{:?}", a.pow(p - 2) % p);
    }
}
