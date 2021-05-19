[CryptoHack – Diffie-Hellman challenges](https://cryptohack.org/challenges/diffie-hellman/)

> Every element of a finite field `Fp` can be used to make a subgroup `H` under repeated action of multiplication. In other words, for an element `g`: `H = {g, g^2, g^3, ...}`
>
> A primitive element of `Fp` is an element whose subgroup `H = Fp`, _i.e._, every element of `Fp`, can be written as `g^n mod p` for some integer `n`. Because of this, primitive elements are sometimes called generators of the finite field.
>
> For the finite field with `p = 28151` find the smallest element `g` which is a primitive element of `Fp`.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package diffie_hellman_starter_2 --lib tests
>
> Capture the flag:
>
>     cargo test --package diffie_hellman_starter_2 --lib tests::capture_the_flag -- --exact
