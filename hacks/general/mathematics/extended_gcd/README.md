[CryptoHack – General challenges](https://cryptohack.org/challenges/general/)

> Let `a` and `b` be positive integers.
>
> The extended Euclidean algorithm is an efficient way to find integers `U,v` such that
>
>     a * U + b * v = gcd(a,b)
>
> > Later, when we learn to decrypt RSA, we will need this algorithm to calculate the modular inverse of the public exponent.
>
> Using the two primes `p = 26513, q = 32321`, find the integers `U,v` such that
>
>     p * U + q * v = gcd(p,q)
>
> > Knowing that `p,q` are prime, what would you expect `gcd(p,q)` to be?

> ### How to:
> Run all tests from this package:
>
>     cargo test --package greatest_common_divisor --lib tests
>
> Capture the flag:
>
>     cargo test --package greatest_common_divisor --lib tests::capture_the_flag -- --exact
