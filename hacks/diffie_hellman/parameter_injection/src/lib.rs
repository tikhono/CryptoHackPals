#[cfg(test)]
mod tests {

    use aes::Aes128;
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Cbc};
    use num::bigint::BigInt;
    use serde::{Deserialize, Serialize};
    use sha1::{Digest, Sha1};
    use telnet::{Telnet, TelnetEvent};

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

    #[test]
    fn capture_the_flag() {
        let mut connection = Telnet::connect(("134.122.111.232", 13371), 8192)
            .expect("Couldn't connect to the server...");

        let event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(_) => {}
            _ => println!("Error"),
        }
        let mut alice_data = FromAlice {
            p: "".to_string(),
            g: "".to_string(),
            A: "".to_string(),
        };
        let event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(buffer) => {
                alice_data =
                    serde_json::from_slice(&buffer.strip_suffix(b"\nSend to Bob: ").unwrap())
                        .unwrap();
            }
            _ => println!("Error"),
        }

        let p = BigInt::parse_bytes(&alice_data.p.as_bytes()[2..], 16).unwrap();
        let alice_pub = BigInt::parse_bytes(&alice_data.A.as_bytes()[2..], 16).unwrap();
        let shared_secret = alice_pub % p;

        let data_for_bob = format!(
            "{{\"p\": \"{}\", \"g\": \"{}\", \"A\": \"{}\"}}",
            alice_data.p, alice_data.g, alice_data.g,
        );
        connection
            .write(data_for_bob.as_bytes())
            .expect("Write Error");

        let event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(_) => {} //Need 2 reads, in this buffer just "Intercepted from Bob"
            _ => println!("Error"),
        }
        let mut _bob_data = FromBob { B: "".to_string() };
        let event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(buffer) => {
                _bob_data =
                    serde_json::from_slice(&buffer.strip_suffix(b"\nSend to Alice: ").unwrap())
                        .unwrap();
            }
            _ => println!("Error"),
        }

        let data_for_alice = format!("{{\"B\": \"{}\"}}", alice_data.g);
        connection
            .write(data_for_alice.as_bytes())
            .expect("Write Error");

        let event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(_) => {} //Need 2 reads, in this buffer just "Intercepted from Bob"
            _ => println!("Error"),
        }

        let mut flag = Flag {
            iv: "".to_string(),
            encrypted_flag: "".to_string(),
        };
        let event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(buffer) => {
                flag = serde_json::from_slice(&buffer).unwrap();
            }
            _ => println!("Error"),
        }

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
