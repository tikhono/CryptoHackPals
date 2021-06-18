fn make_padding(message: &str, key_length: usize) -> Vec<u8> {
    let len = message.len() + key_length;
    let mut pad: Vec<u8> = vec![b'\x80'];
    pad.append(&mut [0].repeat((56 - (len + 1) % 64) % 64));
    pad.append(&mut (message.len() as u64 * 8).to_be_bytes().to_vec());
    pad
}

#[cfg(test)]
mod tests {
    use crate::make_padding;
    use sha1::{Digest, Sha1};

    #[test]
    fn capture_the_flag() {
        let message =
            "comment1=cooking%20MCs;userdata=foo;comment2=%20like%20a%20pound%20of%20bacon";

        let hash = Sha1::new().chain(b"YELLOW SUBMARINE").chain(message);

        let hash = hash.chain(make_padding(message, 16)).chain(";admin=true"); //Forged message.

        println!("Result: {:x}", hash.finalize());

        let hash = Sha1::new();

        let hash = hash
            .chain(b"YELLOW SUBMARINE") //Rand key with the same length as original key.
            .chain(message) //Original message and paddinding next to it.
            .chain(make_padding(message, 16))
            .chain(";admin=true"); //Forged message.

        println!("Result: {:x}", hash.finalize());
    }
}
