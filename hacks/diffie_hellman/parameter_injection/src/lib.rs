#[cfg(test)]
mod tests {

    use aes::Aes128;
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Cbc};
    use num::bigint::BigInt;
    use serde::{Deserialize, Serialize};
    use sha1::{Digest, Sha1};
    use tokio::io::AsyncBufReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::io::BufReader;
    use tokio::net::TcpStream;

    // create an alias for convenience
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    const BLOCK_SIZE: usize = 16;

    #[derive(Serialize, Deserialize, Debug)]
    struct FromAlice {
        p: String,
        g: String,
        A: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct FromBob {
        B: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Flag {
        iv: String,
        encrypted_flag: String,
    }

    #[tokio::test]
    async fn capture_the_flag() {
        let stream = TcpStream::connect("134.122.111.232:13371").await.unwrap();
        let mut stream = BufReader::new(stream);

        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();

        let alice_data: FromAlice =
            serde_json::from_str(line.strip_prefix("Intercepted from Alice: ").unwrap()).unwrap();

        let p = BigInt::parse_bytes(&alice_data.p.as_bytes()[2..], 16).unwrap();
        let alice_pub = BigInt::parse_bytes(&alice_data.A.as_bytes()[2..], 16).unwrap();
        let shared_secret = alice_pub % p;

        let data_for_bob = format!(
            "{{\"p\": \"{}\", \"g\": \"{}\", \"A\": \"{}\"}}",
            alice_data.p, alice_data.g, alice_data.g,
        );
        stream.write_all(data_for_bob.as_bytes()).await.unwrap();

        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();

        let data_for_alice = format!("{{\"B\": \"{}\"}}", alice_data.g);
        stream.write_all(data_for_alice.as_bytes()).await.unwrap();

        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();

        let flag: Flag = serde_json::from_str(
            line.strip_prefix("Send to Alice: Intercepted from Alice: ")
                .unwrap(),
        )
        .unwrap();

        let iv: Vec<u8> = hex::decode(flag.iv).unwrap();
        let flag = hex::decode(flag.encrypted_flag).unwrap();

        let mut hasher = Sha1::new();
        hasher.update(&shared_secret.to_str_radix(10));
        let key = hasher.finalize().to_vec();
        let key = key.get(0..BLOCK_SIZE).unwrap();

        let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
        let mut buf = flag;

        if cipher.decrypt(&mut buf).is_ok() {};
        println!("{}", String::from_utf8_lossy(&buf));
    }
}
