[Challenge 27 Set 4 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/4/challenges/27)

> Take your code from [the CBC exercise](https://cryptopals.com/sets/2/challenges/16) and modify it so that it repurposes the key for CBC encryption as the IV.
>
> Applications sometimes use the key as an IV on the auspices that both the sender and the receiver have to know the key already, and can save some space by using it as both a key and an IV.
>
> Using the key as an IV is insecure; an attacker that can modify ciphertext in flight can get the receiver to decrypt a value that will reveal the key.
>
> The CBC code from exercise 16 encrypts a URL string. Verify each byte of the plaintext for ASCII compliance (ie, look for high-ASCII values). Noncompliant messages should raise an exception or return an error that includes the decrypted plaintext (this happens all the time in real systems, for what it's worth).
>
> Use your code to encrypt a message that is at least 3 blocks long:
>
> AES-CBC(P\_1, P\_2, P\_3) -> C\_1, C\_2, C\_3
>
> Modify the message (you are now the attacker):
>
> C\_1, C\_2, C\_3 -> C\_1, 0, C\_1
>
> Decrypt the message (you are now the receiver) and raise the appropriate error if high-ASCII is found.
>
> As the attacker, recovering the plaintext from the error, extract the key:
>
> P'\_1 XOR P'\_3

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _27_recover_the_key_from_cbc_with_iv_is_key --lib tests
>
> Capture the flag:
>
>     cargo test --package _27_recover_the_key_from_cbc_with_iv_is_key --lib tests::capture_the_flag -- --exact
