[CryptoHack – Block Ciphers challenges](https://cryptohack.org/challenges/aes/)

> Here you can encrypt in CBC but only decrypt in ECB. That shouldn't be a weakness because they're different modes... right?
>
> Play at [http://aes.cryptohack.org/ecbcbcwtf](http://aes.cryptohack.org/ecbcbcwtf)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package ecb_cbc_wtf --lib tests
>
> Capture the flag:
>
>     cargo test --package ecb_cbc_wtf --lib tests::capture_the_flag -- --exact
