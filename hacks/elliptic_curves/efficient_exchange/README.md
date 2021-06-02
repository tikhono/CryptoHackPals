[CryptoHack – Elliptic Curves challenges](https://cryptohack.org/challenges/ecc/)

> Alice and Bob are looking at the Elliptic Curve Discrete Logarithm Problem and thinking about the data they send.
>
> They want to try and keep their data transfer as efficient as possible and realise that sending both the `x` and `y` coordinate of their public key isn't necessary.
>
> As long as Alice and Bob agree on the curve parameters, there are only ever two possible values of `y` for a given `x`.
>
> In fact, given _either_ of the values of `y` permissible from the value `x` they receive, the `x` coordinate of their shared secret will be the same.
>
> For these challenges, we have used a prime `p ≡ 3 mod 4`, which will help you find `y` from `y2`.
>
> Using the curve, prime and generator:
>
> E: Y2 = X3 + 497 X + 1768, p: 9739, G: (1804,5368)
>
> Calculate the shared secret after Alice sends you `q_x = 4726`, with your secret integer `nB = 6534`.
>
> Use the `decrypt.py` file to decode the flag
>
> `{'iv': 'cd9da9f1c60925922377ea952afc212c', 'encrypted_flag': 'febcbe3a3414a730b125931dccf912d2239f3e969c4334d95ed0ec86f6449ad8'}`
>
> You can specifiy which of the two possible values your public `y` coordinate has taken by sending only one bit. Try and think about how you could do this. How are the two y values related to each other?

> ### How to:
> Run all tests from this package:
>
>     cargo test --package efficient_exchange --lib tests
>
> Capture the flag:
>
>     cargo test --package efficient_exchange --lib tests::capture_the_flag -- --exact
