[Challenge 5 Set 1 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/1/challenges/5)

> ### Implement repeating-KEY XOR
>
> Here is the opening stanza of an important work of the English language:
>
>     Burning 'em, if you ain't quick and nimble
>     I go crazy when I hear a cymbal
>
> Encrypt it, under the KEY "ICE", using repeating-KEY XOR.
>
> In repeating-KEY XOR, you'll sequentially apply each byte of the KEY; the first byte of plaintext will be XOR'd against I, the next C, the next E, then I again for the 4th byte, and so on.
>
> It should come out to:
>
>     0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
>     a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
>
> Encrypt a bunch of stuff using your repeating-KEY XOR function. Encrypt your mail. Encrypt your password file. Your .sig file. Get a feel for it. I promise, we aren't wasting your time with this.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _05_implement_repeating_key_xor --lib tests
>
> Test repeating_key_xor:
>
>     cargo test --package _05_implement_repeating_key_xor --lib tests::test_repeating_key_xor -- --exact
