[CryptoHack – RSA challenges](https://cryptohack.org/challenges/rsa/)

> I've found a super fast way to generate primes from my secret list.
>
> [marin.py](https://cryptohack.org/static/challenges/marin_c5ddcf5ba9b4873a7d215b3101a1383f.py)
>
> [output.txt](https://cryptohack.org/static/challenges/output_f194012343666ced1a6699d196c8adc5.txt)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package marins_secrets --lib tests
>
> Capture the flag:
>
>     cargo test --package marins_secrets --lib tests::capture_the_flag -- --exact
