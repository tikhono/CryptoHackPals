[Challenge 21 Set 3 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/3/challenges/21)

> ### Implement the MT19937 Mersenne Twister RNG
>
> You can get the psuedocode for this from Wikipedia.
>
> If you're writing in Python, Ruby, or (gah) PHP, your language is probably already giving you MT19937 as "rand()"; **don't use rand()**. Write the RNG yourself.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _21_implement_the_mt19937_mersenne_twister_rng --lib tests
>
> Test seed_5489:
>
>     cargo test --package _21_implement_the_mt19937_mersenne_twister_rng --lib tests::test_seed_5489 -- --exact
>
