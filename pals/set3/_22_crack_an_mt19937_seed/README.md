[Challenge 22 Set 3 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/3/challenges/22)

> ### Crack an MT19937 seed
>
> Make sure your MT19937 accepts an integer seed value. Test it (verify that you're getting the same sequence of outputs given a seed).
>
> Write a routine that performs the following operation:
>
> -   Wait a random number of seconds between, I don't know, 40 and 1000.
> -   Seeds the RNG with the current Unix timestamp
> -   Waits a random number of seconds again.
> -   Returns the first 32 bit output of the RNG.
>
> You get the idea. Go get coffee while it runs. Or just simulate the passage of time, although you're missing some of the fun of this exercise if you do that.
>
> From the 32 bit RNG output, discover the seed.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _22_crack_an_mt19937_seed --lib tests
>
> Test seed_crack:
>
>     cargo test --package _22_crack_an_mt19937_seed --lib tests::test_seed_crack -- --exact
>
