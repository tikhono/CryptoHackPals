static mut KEY: [u8; 16] = [0u8; 16];
use _09_implement_pkcs7_padding::pad;
use openssl::symm::{decrypt, encrypt, Cipher};
use std::collections::HashMap;

fn parse(str: &str) -> HashMap<String, String> {
    let mut cookie = HashMap::new();
    for pair in str.split('&') {
        let v: Vec<&str> = pair.split('=').collect();
        cookie.insert(v[0].to_string(), v[1].to_string());
    }
    println!("{:?}", cookie);
    cookie
}

fn profile_for(email: &str) -> String {
    let email = email.replace("&", "").replace("=", "");
    format!("email={}&uid=10&role=user", email)
}

fn oracle(email: &str) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();

    let profile = profile_for(email);
    encrypt(cipher, &unsafe { KEY }, None, &pad(profile.as_ref(), 16)).unwrap()
}

fn decrypt_cookie(profile: &[u8]) -> String {
    let cipher = Cipher::aes_128_ecb();

    let bytes = decrypt(cipher, &unsafe { KEY }, None, profile).unwrap();
    String::from_utf8_lossy(&bytes).to_string()
}

fn rewrite_cookie(email: &mut String) -> Result<Vec<u8>, &'static str> {
    let block_size = 16;

    if email.len() > 13 {
        return Err("Please, past shorter mail");
    } else {
        email.push_str(&"_".repeat(13 - email.len()));
    };

    let admin_block = pad(b"admin", block_size as u8);
    let email = [
        email.get(0..email.len() - 3).unwrap().as_bytes(),
        admin_block.as_slice(),
        email.get(email.len() - 3..email.len()).unwrap().as_bytes(),
    ]
    .concat();

    let ciphertext = &oracle(std::str::from_utf8(&email).unwrap());

    let c1 = ciphertext.get(0..16).unwrap();
    let c2 = ciphertext.get(16..32).unwrap();
    let c3 = ciphertext.get(32..48).unwrap();
    let c4 = ciphertext.get(48..).unwrap();

    Ok([c1, c3, c2, c4].concat())
}

#[cfg(test)]
mod tests {
    use crate::{decrypt_cookie, oracle, parse, profile_for, rewrite_cookie, KEY};

    #[test]
    fn test_parse() {
        parse("email=AAAAAAAAA.com&uid=10&role=admin:::::::::::user");
    }

    #[test]
    fn test_profile_for() {
        println!("{}", profile_for("fake@email.com"));
    }

    #[test]
    fn test_oracles() {
        assert_eq!(
            "email=fake@email.com&uid=10&role=user\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}\u{b}",
            decrypt_cookie(&oracle("fake@email.com"))
        );
    }

    #[test]
    fn capture_the_flag() {
        unsafe {
            KEY = rand::random();
        }

        let admin_cookie = rewrite_cookie(&mut "fake@mail.com".to_string());

        match admin_cookie {
            Ok(cookie) => println!("{}", decrypt_cookie(&cookie)),
            Err(e) => println!("{}", e),
        }

        let admin_cookie = rewrite_cookie(&mut "very_long_mail.com".to_string());

        match admin_cookie {
            Ok(cookie) => println!("{}", decrypt_cookie(&cookie)),
            Err(e) => println!("{}", e),
        }

        let admin_cookie = rewrite_cookie(&mut "short@mail".to_string());

        match admin_cookie {
            Ok(cookie) => println!("{}", decrypt_cookie(&cookie)),
            Err(e) => println!("{}", e),
        }
    }
}
