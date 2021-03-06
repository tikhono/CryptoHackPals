[Challenge 20 Set 3 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/3/challenges/20)

> ### Break fixed-nonce CTR statistically
>
> [In this file](https://cryptopals.com/static/challenge-data/20.txt) find a similar set of Base64'd plaintext. Do with them exactly what you did with the first, but solve the problem differently.
>
> Instead of making spot guesses at to known plaintext, treat the collection of ciphertexts the same way you would repeating-KEY XOR.
>
> Obviously, CTR encryption appears different from repeated-KEY XOR, _but with a fixed nonce they are effectively the same thing._
>
> To exploit this: take your collection of ciphertexts and truncate them to a common length (the length of the smallest CIPHERTEXT will work).
>
> Solve the resulting concatenation of ciphertexts as if for repeating- KEY XOR, with a KEY size of the length of the CIPHERTEXT you XOR'd.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _20_break_fixed_nonce_ctr_statistically --lib tests
>
> Test clone_mt:
>
>     cargo test --package _20_break_fixed_nonce_ctr_statistically --lib tests::test_clone_mt -- --exact
>
