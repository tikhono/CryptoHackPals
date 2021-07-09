[CryptoHack – Elliptic Curves challenges](https://cryptohack.org/challenges/ecc/)

> The Elliptic Curve Discrete Logarithm Problem (ECDLP) is the problem of finding an integer `n` such that `Q = nP`.
>
> Like we encountered with the discrete logarithm problem, scalar multiplication of a point in `E(Fp)` seems to be be a hard problem to undo, with the most efficient algorithm running at `p1/2` time.
>
> This makes it a great candidate for a trapdoor function.
>
> Alice and Bob are talking and they want to create a shared secret so they can start encrypting their messages with some symmetric cryptographic protocol.
>
> Alice and Bob don't trust their connection, so they need a way create a secret others can't replicate.
>
> Alice and Bob agree on a curve `E`, a prime `p` and a generator point `G`
>
> In elliptic curve cryptography, it is important that the order of `G` is prime. Constructing secure curves is complicated and it is recommended to use a preconstructed curve where a client is given the curve, the prime and the generator to use.
>
> Alice generates a secret random integer `nA` and calculates `QA = nAG`
>
> Bob generates a secret random integer `nB` and calculates `QB = nBG`
>
> Alice sends Bob `QA`, and Bob sends Alice `QB`. Due to the hardness of ECDLP, an onlooker Eve is unable to calculate `nA/B` in reasonable time.
>
> Alice then calculates `nAQB`, and Bob calculates `nBQA`.
>
> Due to the associativity of scalar multiplication, `S = nAQB = nBQA`.
>
> Alice and Bob can use `S` as their shared secret.
>
> Using the curve, prime and generator:
>
> E: Y2 = X3 + 497 X + 1768, p: 9739, G: (1804,5368)
>
> Calculate the shared secret after Alice sends you `QA = (815, 3190)`, with your secret integer `nB = 1829`.
>
> Generate a key by calculating the SHA1 hash of the `x` coordinate (take the decimal representation of the coordinate and cast it to a string). The flag is the hexdigest you find.
>
> This curve is not cryptographically secure!! We've picked a small prime for these starter challenges to keep everything fast while you learn. Cryptographically secure curves have primes of bit size ≈ 256

> ### How to:
> Run all tests from this package:
>
>     cargo test --package curves_and_logs --lib tests
>
> Capture the flag:
>
>     cargo test --package curves_and_logs --lib tests::capture_the_flag -- --exact
