[Challenge 24 Set 3 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/3/challenges/24)

> ### Create the MT19937 stream cipher and break it
>
> You can create a trivial stream cipher out of any PRNG; use it to generate a sequence of 8 bit outputs and call those outputs a keystream. XOR each byte of plaintext with each successive byte of keystream.
>
> Write the function that does this for MT19937 using a 16-bit seed. Verify that you can encrypt and decrypt properly. This code should look similar to your CTR code.
>
> Use your function to encrypt a known plaintext (say, 14 consecutive 'a' characters) prefixed by a random number of random characters.
>
> From the CIPHERTEXT, recover the "KEY" (the 16 bit seed).
>
> Use the same idea to generate a random "password reset token" using MT19937 seeded from the current time.
>
> Write a function to check if any given password token is actually the product of an MT19937 PRNG seeded with the current time.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _24_create_the_mt_19937_stream_cipher_ans_break_it --lib tests
>
> Test clone_mt:
>
>     cargo test --package _24_create_the_mt_19937_stream_cipher_ans_break_it --lib tests::test_clone_mt -- --exact
>
