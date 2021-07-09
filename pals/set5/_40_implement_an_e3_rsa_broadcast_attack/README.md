[Challenge 40 Set 5 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/5/challenges/40)

> ### Implement an E=3 RSA Broadcast attack
>
> Assume you're a Javascript programmer. That is, you're using a naive handrolled RSA to encrypt without padding.
>
> Assume you can be coerced into encrypting the same plaintext three times, under three different public keys. You can; it's happened.
>
> Then an attacker can trivially decrypt your message, by:
>
> 1.  Capturing any 3 of the ciphertexts and their corresponding pubkeys
> 2.  Using the CRT to solve for the number represented by the three ciphertexts (which are residues mod their respective pubkeys)
> 3.  Taking the cube root of the resulting number
>
> The CRT says you can take any number and represent it as the combination of a series of residues mod a series of moduli. In the three-residue case, you have:
>
> result =
>   (c\_0 \* m\_s\_0 \* invmod(m\_s\_0, n\_0)) +
>   (c\_1 \* m\_s\_1 \* invmod(m\_s\_1, n\_1)) +
>   (c\_2 \* m\_s\_2 \* invmod(m\_s\_2, n\_2)) mod N\_012
>
> where:
>
>  c\_0, c\_1, c\_2 are the three respective residues mod
>  n\_0, n\_1, n\_2
>
>  m\_s\_n (for n in 0, 1, 2) are the product of the moduli
>  EXCEPT n\_n --- ie, m\_s\_1 is n\_0 \* n\_2
>
>  N\_012 is the product of all three moduli
>
> To decrypt RSA using a simple cube root, leave off the final modulus operation; just take the raw accumulated result and cube-root it.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _40_implement_an_e3_rsa_broadcast_attack --lib tests
>
> Capture the flag:
>
>     cargo test --package _40_implement_an_e3_rsa_broadcast_attack --lib tests::capture_the_flag -- --exact
