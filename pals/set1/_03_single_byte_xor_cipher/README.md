[Challenge 3 Set 1 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/1/challenges/3)

> ### Single-byte XOR cipher
>
> The hex encoded string:
>
>     1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
>
> ... has been XOR'd against a single character. Find the KEY, decrypt the message.
>
> You can do this by hand. But don't: write code to do it for you.
>
> How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.
>
> > ### Achievement Unlocked
> > You now have our permission to make "ETAOIN SHRDLU" jokes on Twitter.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _03_single_byte_xor_cipher --lib tests
>
> Capture the flag:
>
>     cargo test --package _03_single_byte_xor_cipher --lib tests::capture_the_flag -- --exact
