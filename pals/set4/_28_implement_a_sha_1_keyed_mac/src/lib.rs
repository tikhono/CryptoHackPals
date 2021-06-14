#[cfg(test)]
mod tests {

    #[test]
    fn capture_the_flag() {
        use sha1::{Digest, Sha1};

        let hash = Sha1::new().chain("key").chain("message").finalize();

        println!("Result: {:x}", hash);

        assert_ne!(
            Sha1::new().chain("key").chain("message").finalize(),
            Sha1::new().chain("ke").chain("message").finalize()
        );
        assert_ne!(
            Sha1::new().chain("key").chain("message").finalize(),
            Sha1::new().chain("keyy").chain("message").finalize()
        );
        assert_ne!(
            Sha1::new().chain("key").chain("message").finalize(),
            Sha1::new().chain("key").chain("messsage").finalize()
        );
        assert_ne!(
            Sha1::new().chain("key").chain("message").finalize(),
            Sha1::new().chain("key").chain("mesage").finalize()
        );
    }
}
