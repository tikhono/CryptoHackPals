[CryptoHack – RSA challenges](https://cryptohack.org/challenges/rsa/)

> My boss has so many emails he's set up a server to just sign everything automatically. He also stores encrypted messages to himself for easy access. I wonder what he's been saying.
>
> Connect at `nc socket.cryptohack.org 13374`
>
> [13374.py](https://cryptohack.org/static/challenges/13374_1455e06ebe824637f7c31c94a9eb58fa.py)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package signing_server --lib tests
>
> Capture the flag:
>
>     cargo test --package signing_server --lib tests::capture_the_flag -- --exact
