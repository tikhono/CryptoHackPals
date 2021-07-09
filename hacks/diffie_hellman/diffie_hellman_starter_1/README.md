[CryptoHack – Diffie-Hellman challenges](https://cryptohack.org/challenges/diffie-hellman/)

> The set of integers modulo `n`, together with the operations of both addition and multiplication is a ring. This means that adding or multiplying any two elements in the set returns another element in the set.
>
> When the modulus is prime: `n = p`, we are guaranteed an inverse of every element in the set, and so the ring is promoted to a field. We refer to this field as a finite field `Fp`.
>
> The Diffie-Hellman protocol works with elements of some finite field `Fp`, where the prime modulus is typically a large prime.
>
> Given the prime `p = 991`, and the element `g = 209`, find the inverse element `d` such that `g * d mod 991 = 1`.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package diffie_hellman_starter_1 --lib tests
>
> Capture the flag:
>
>     cargo test --package diffie_hellman_starter_1 --lib tests::capture_the_flag -- --exact
>
> Test egcd:
>
>     cargo test --package diffie_hellman_starter_1 --lib tests::test_egcd -- --exact
