[CryptoHack – Block Ciphers challenges](https://cryptohack.org/challenges/aes/)

> We're going to skip over the finer details of the **KeyExpansion** phase for now. The main point is that it takes in our 16 byte KEY and produces 11 4x4 matrices called "round keys" derived from our initial KEY. These round keys allow AES to get extra mileage out of the single KEY that we provided.
>
> The **initial KEY addition** phase, which is next, has a single _AddRoundKey_ step. The _AddRoundKey_ step is straightforward: it XORs the current state with the current round KEY.
>
> ![diagram showing AddRoundKey](https://cryptohack.org/static/img/aes/AddRoundKey.png)
>
> _AddRoundKey_ also occurs as the final step of each round. _AddRoundKey_ is what makes AES a "keyed permutation" rather than just a permutation. It's the only part of AES where the KEY is mixed into the state, but is crucial for determining the permutation that occurs.
>
> As you've seen in previous challenges, XOR is an easily invertible operation if you know the KEY, but tough to undo if you don't. Now imagine trying to recover plaintext which has been XOR'd with 11 different keys, and heavily jumbled between each XOR operation with a series of substitution and transposition ciphers. That's kinda what AES does! And we'll see just how effective the jumbling is in the next few challenges.
>
> Complete the `add_round_key` function, then use the `matrix2bytes` function to get your next flag.
>
> [add\_round\_key.py](https://cryptohack.org/static/challenges/add_round_key_b67b9a529ae739156107a74b14adde98.py)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package round_keys --lib tests
>
> Capture the flag:
>
>     cargo test --package round_keys --lib tests::capture_the_flag -- --exact
