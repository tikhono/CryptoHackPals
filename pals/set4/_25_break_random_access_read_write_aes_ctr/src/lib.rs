#[macro_use]
extern crate lazy_static;

use openssl::symm::{decrypt, encrypt, Cipher};
use std::sync::RwLock;

const BLOCK_SIZE: usize = 16;

lazy_static! {
    static ref KEY: RwLock<[u8; BLOCK_SIZE]> = RwLock::new([0u8; BLOCK_SIZE]);
    static ref IV: RwLock<[u8; BLOCK_SIZE]> = RwLock::new([0u8; BLOCK_SIZE]);
    static ref CIPHER: Cipher = Cipher::aes_128_ctr();
}

pub fn edit(ciphertext: &[u8], offset: usize, character: u8) -> Vec<u8> {
    let mut plaintext = decrypt(
        *CIPHER,
        &*KEY.read().unwrap(),
        Some(&*IV.read().unwrap()),
        ciphertext,
    )
    .unwrap();

    plaintext[offset] = character;

    encrypt(
        *CIPHER,
        &*KEY.read().unwrap(),
        Some(&*IV.read().unwrap()),
        &plaintext,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{edit, CIPHER, IV, KEY};
    use openssl::symm::encrypt;

    #[test]
    fn capture_the_flag() {
        {
            let mut w_key = KEY.write().unwrap();
            *w_key = rand::random();
            let mut w_iv = IV.write().unwrap();
            *w_iv = rand::random();
        }
        let plaintext = "I'm back and I'm ringin' the bell 
A rockin' on the mike while the fly girls yell 
In ecstasy in the back of me 
Well that's my DJ Deshay cuttin' all them Z's 
Hittin' hard and the girlies goin' crazy 
Vanilla's on the mike, man I'm not lazy. 

I'm lettin' my drug kick in 
It controls my mouth and I begin 
To just let it flow, let my concepts go 
My posse's to the side yellin', Go Vanilla Go! 

Smooth 'cause that's the way I will be 
And if you don't give a damn, then 
Why you starin' at me 
So get off 'cause I control the stage 
There's no dissin' allowed 
I'm in my own phase 
The girlies sa y they love me and that is ok 
And I can dance better than any kid n' play 

Stage 2 -- Yea the one ya' wanna listen to 
It's off my head so let the beat play through 
So I can funk it up and make it sound good 
1-2-3 Yo -- Knock on some wood 
For good luck, I like my rhymes atrocious 
Supercalafragilisticexpialidocious 
I'm an effect and that you can bet 
I can take a fly girl and make her wet. 

I'm like Samson -- Samson to Delilah 
There's no denyin', You can try to hang 
But you'll keep tryin' to get my style 
Over and over, practice makes perfect 
But not if you're a loafer. 

You'll get nowhere, no place, no time, no girls 
Soon -- Oh my God, homebody, you probably eat 
Spaghetti with a spoon! Come on and say it! 

VIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino 
Intoxicating so you stagger like a wino 
So punks stop trying and girl stop cryin' 
Vanilla Ice is sellin' and you people are buyin' 
'Cause why the freaks are jockin' like Crazy Glue 
Movin' and groovin' trying to sing along 
All through the ghetto groovin' this here song 
Now you're amazed by the VIP posse. 

Steppin' so hard like a German Nazi 
Startled by the bases hittin' ground 
There's no trippin' on mine, I'm just gettin' down 
Sparkamatic, I'm hangin' tight like a fanatic 
You trapped me once and I thought that 
You might have it 
So step down and lend me your ear 
'89 in my time! You, '90 is my year. 

You're weakenin' fast, YO! and I can tell it 
Your body's gettin' hot, so, so I can smell it 
So don't be mad and don't be sad 
'Cause the lyrics belong to ICE, You can call me Dad 
You're pitchin' a fit, so step back and endure 
Let the witch doctor, Ice, do the dance to cure 
So come up close and don't be square 
You wanna battle me -- Anytime, anywhere 

You thought that I was weak, Boy, you're dead wrong 
So come on, everybody and sing this song 

Say -- Play that funky music Say, go white boy, go white boy go 
play that funky music Go white boy, go white boy, go 
Lay down and boogie and play that funky music till you die. 

Play that funky music Come on, Come on, let me hear 
Play that funky music white boy you say it, say it 
Play that funky music A little louder now 
Play that funky music, white boy Come on, Come on, Come on 
Play that funky music 
";

        let ciphertext = encrypt(
            *CIPHER,
            &*KEY.read().unwrap(),
            Some(&*IV.read().unwrap()),
            plaintext.as_bytes(),
        )
        .unwrap();

        let mut result = "".to_string();
        for i in 0..ciphertext.len() {
            for guess in 0u8..=255 {
                if edit(&ciphertext, i, guess) == ciphertext {
                    result.push(guess as char);
                }
            }
        }
        assert_eq!(result, plaintext);
    }
}
