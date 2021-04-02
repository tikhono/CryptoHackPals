[CryptoHack – Block Ciphers challenges](https://cryptohack.org/challenges/aes/)

> Some block cipher modes, such as OFB, CTR, or CFB, turn a block cipher into a stream cipher. The idea behind stream ciphers is to produce a pseudorandom keystream which is then XORed with the plaintext. One advantage of stream ciphers is that they can work of plaintext of arbitrary length, with no padding required.
>
> OFB is an obscure cipher mode, with no real benefits these days over using CTR. This challenge introduces an unusual property of OFB.
>
> Play at [http://aes.cryptohack.org/symmetry](http://aes.cryptohack.org/symmetry)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package symmetry --lib tests
>
> Capture the flag:
>
>     cargo test --package symmetry --lib tests::capture_the_flag -- --exact
