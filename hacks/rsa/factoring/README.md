[CryptoHack – RSA challenges](https://cryptohack.org/challenges/rsa/)

> RSA relies on the difficulty of the factorisation of the modulus `N`. If the primes can be found then we can calculate the [Euler totient](https://leimao.github.io/article/RSA-Algorithm/) of `N` and thus decrypt the ciphertext.
>
> Given `N = p*q` and two primes:
>
> `p = 857504083339712752489993810777`
>
> `q = 1029224947942998075080348647219`
>
> What is the totient of `N`?

> ### How to:
> Run all tests from this package:
>
>     cargo test --package factoring --lib tests
>
> Capture the flag:
>
>     cargo test --package factoring --lib tests::capture_the_flag -- --exact
