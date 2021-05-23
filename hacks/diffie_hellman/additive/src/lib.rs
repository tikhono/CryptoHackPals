#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;
    use openssl::symm::{decrypt, Cipher};
    use serde::Deserialize;
    use sha1::{Digest, Sha1};
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    use tokio::net::TcpStream;

    const BLOCK_SIZE: usize = 16;

    #[derive(Deserialize, Debug)]
    struct FromAlice {
        p: String,
        g: String,
        A: String,
    }

    #[derive(Deserialize, Debug)]
    struct FromBob {
        B: String,
    }

    #[derive(Deserialize, Debug)]
    struct Flag {
        iv: String,
        encrypted: String,
    }

    #[tokio::test]
    async fn capture_the_flag() {
        let stream = TcpStream::connect("134.122.111.232:13380").await.unwrap();
        let mut stream = BufReader::new(stream);

        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();
        let alice_data: FromAlice =
            serde_json::from_str(&line.strip_prefix("Intercepted from Alice: ").unwrap()).unwrap();

        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();
        let bob_data: FromBob =
            serde_json::from_str(&line.strip_prefix("Intercepted from Bob: ").unwrap()).unwrap();

        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();
        let flag: Flag =
            serde_json::from_str(&line.strip_prefix("Intercepted from Alice: ").unwrap()).unwrap();

        let iv = hex::decode(flag.iv).unwrap();
        let flag = hex::decode(flag.encrypted).unwrap();

        //First 2 bytes are "0x", so cut off them to successfully parse bigint.
        let alice_pub = BigInt::parse_bytes(&alice_data.A.as_bytes()[2..], 16).unwrap();
        let bob_pub = BigInt::parse_bytes(&bob_data.B.as_bytes()[2..], 16).unwrap();
        let p = BigInt::parse_bytes(&alice_data.p.as_bytes()[2..], 16).unwrap();
        let g = BigInt::parse_bytes(&alice_data.g.as_bytes()[2..], 16).unwrap();

        let mut alice_secret = BigInt::parse_bytes(b"0", 10).unwrap();

        let inv = mod_inverse(g, p.clone()).unwrap();
        alice_secret = alice_pub * inv % &p;

        let shared_secret = bob_pub * &alice_secret % &p;

        let mut hasher = Sha1::new();
        hasher.update(&shared_secret.to_str_radix(10));
        let key = hasher.finalize().to_vec();
        let key = key.get(0..BLOCK_SIZE).unwrap();

        let cipher = Cipher::aes_128_cbc();
        match decrypt(cipher, key, Some(&iv), &flag) {
            Ok(text) => {
                println!("{}", String::from_utf8_lossy(&text))
            }
            Err(_) => {
                println!("Unable to decrypt");
            }
        };
    }
}
