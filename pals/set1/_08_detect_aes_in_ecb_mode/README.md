[Challenge 8 Set 1 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/1/challenges/8)

> ### Detect AES in ECB mode
>
> [In this file](https://cryptopals.com/static/challenge-data/8.txt) are a bunch of hex-encoded ciphertexts.
>
> One of them has been encrypted with ECB.
>
> Detect it.
>
> Remember that the problem with ECB is that it is stateless and deterministic; the same 16 byte plaintext block will always produce the same 16 byte CIPHERTEXT.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _08_detect_aes_in_ecb_mode --lib tests
>
> Capture the flag:
>
>     cargo test --package _08_detect_aes_in_ecb_mode --lib tests::capture_the_flag -- --exact
