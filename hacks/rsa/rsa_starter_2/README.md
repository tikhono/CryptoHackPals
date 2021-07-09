[CryptoHack – RSA challenges](https://cryptohack.org/challenges/rsa/)

> RSA encryption is modular exponentiation of a message with an exponent `e` and a modulus `N` which is normally a product of two primes: `N = p * q`.
>
> Together the exponent and modulus form an RSA "public key" `(N, e)`. The most common value for `e` is `0x10001` or `65537`.
>
> "Encrypt" the number `12` using the exponent `e = 65537` and the primes `p = 17` and `q = 23`. What number do you get as the ciphertext?

> ### How to:
> Run all tests from this package:
>
>     cargo test --package rsa_starter_2 --lib tests
>
> Capture the flag:
>
>     cargo test --package rsa_starter_2 --lib tests::capture_the_flag -- --exact
