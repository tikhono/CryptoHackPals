[CryptoHack – Block Ciphers challenges](https://cryptohack.org/challenges/aes/)

> ECB is the most simple mode, with each plaintext block encrypted entirely independently. In this case, your input is prepended to the secret flag and encrypted and that's it. We don't even provide a decrypt function. Perhaps you don't need a padding oracle when you have an "ECB oracle"?
>
> Play at [http://aes.cryptohack.org/ecb\_oracle](http://aes.cryptohack.org/ecb_oracle)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package ecb_oracle --lib tests
>
> Capture the flag:
>
>     cargo test --package ecb_oracle --lib tests::capture_the_flag -- --exact
