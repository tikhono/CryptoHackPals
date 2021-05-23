[CryptoHack – Diffie-Hellman challenges](https://cryptohack.org/challenges/diffie-hellman/)

> Alice and Bob decided to do their DHKE in an additive group rather than a multiplicative group. What could go wrong?
>
> Use the script from "Diffie-Hellman Starter 5" to decrypt the flag once you've recovered the shared secret.
>
> Connect at `nc socket.cryptohack.org 13380`

> ### How to:
> Run all tests from this package:
>
>     cargo test --package additive --lib tests
>
> Capture the flag:
>
>     cargo test --package additive --lib tests::capture_the_flag -- --exact
