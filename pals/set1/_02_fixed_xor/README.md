[Challenge 2 Set 1 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/1/challenges/2)

> ### Fixed XOR
>
> Write a function that takes two equal-length buffers and produces their XOR combination.
>
> If your function works properly, then when you feed it the string:
>
>     1c0111001f010100061a024b53535009181c
>
> ... after hex decoding, and when XOR'd against:
>
>     686974207468652062756c6c277320657965
>
> ... should produce:
>
>     746865206b696420646f6e277420706c6179

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _02_fixed_xor --lib tests
>
> Capture the flag:
>
>     cargo test --package _02_fixed_xor --lib tests::capture_the_flag -- --exact
>
> Test 00 xor 00:
>
>     cargo test --package _02_fixed_xor --lib tests::test_00_xor_00 -- --exact
>
> Test 00 xor ff:
>
>     cargo test --package _02_fixed_xor --lib tests::test_00_xor_ff -- --exact
>
> Test ff xor 00:
>
>     cargo test --package _02_fixed_xor --lib tests::test_ff_xor_00 -- --exact
>
> Test ff xor ff:
>
>     cargo test --package _02_fixed_xor --lib tests::test_ff_xor_ff -- --exact
