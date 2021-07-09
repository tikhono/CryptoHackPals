[CryptoHack – RSA challenges](https://cryptohack.org/challenges/rsa/)

> Using one prime factor was definitely a bad idea so I'll try using over 30 instead.
>
> If it's taking forever to factorise, read up on factorisation algorithms and make sure you're using one that's optimised for this scenario.
>
> [output.txt](https://cryptohack.org/static/challenges/output_5a478a5d4764257d0bbdfaed340fcbdd.txt)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package manyprime --lib tests
>
> Capture the flag:
>
>     cargo test --package manyprime --lib tests::capture_the_flag -- --exact
