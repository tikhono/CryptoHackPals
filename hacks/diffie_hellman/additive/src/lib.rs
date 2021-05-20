#[cfg(test)]
mod tests {
    use num::bigint::BigInt;
    use openssl::symm::{decrypt, encrypt, Cipher};
    use serde::Deserialize;
    use sha1::{Digest, Sha1};
    use telnet::{Telnet, TelnetEvent};

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

    #[test]
    fn capture_the_flag() {
        let mut connection = Telnet::connect(("134.122.111.232", 13380), 4096)
            .expect("Couldn't connect to the server...");
        println!("Connected to server");

        let event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(_) => {} //Need 2 reads, in this buffer just "Intercepted from Alice"
            _ => println!("Error"),
        }
        let event = connection.read().expect("Read Error");

        match event {
            TelnetEvent::Data(buffer) => {
                let json_strings: Vec<&[u8]> = buffer.split(|n| *n == b'\n').collect();

                let alice_data: FromAlice = serde_json::from_slice(json_strings[0]).unwrap();
                let bob_data: FromBob = serde_json::from_slice(&json_strings[1][22..]).unwrap(); //Cut "Intercepted from Bob"
                let flag: Flag = serde_json::from_slice(&json_strings[2][24..]).unwrap(); //Cut "Intercepted from Alice"

                println!("{:#?}", alice_data);
                println!("{:#?}", bob_data);
                println!("{:#?}", flag);

                let iv: Vec<u8> = hex::decode(flag.iv).unwrap();

                let flag = hex::decode(flag.encrypted).unwrap();

                //First 2 bytes are "0x", so cut off them to successfully parse bigint.
                let alice_pub = BigInt::parse_bytes(&alice_data.A.as_bytes()[2..], 16).unwrap();
                let bob_pub = BigInt::parse_bytes(&bob_data.B.as_bytes()[2..], 16).unwrap();
                let p = BigInt::parse_bytes(&alice_data.p.as_bytes()[2..], 16).unwrap();
                let g = BigInt::parse_bytes(&alice_data.g.as_bytes()[2..], 16).unwrap();

                let mut alice_secret = BigInt::parse_bytes(b"0", 10).unwrap();

                for n in 0..u64::MAX {
                    println!("{}", n);
                    if (&bob_pub * ((&alice_pub + (&p * n)) / &g)) % &p
                        == (&alice_pub * ((&bob_pub + (&p * n)) / &g)) % &p
                    {
                        alice_secret = (&alice_pub + (&p * n)) / &g;
                        break;
                    }
                }

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
            _ => println!("Error"),
        }
    }
}
