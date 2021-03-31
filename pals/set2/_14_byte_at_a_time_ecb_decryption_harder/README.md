[Challenge 14 Set 2 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/2/challenges/14)

> ### Byte-at-a-time ECB decryption (Harder)
>
> Take your oracle function [from #12.](https://cryptopals.com/sets/2/challenges/12) Now generate a random count of random bytes and prepend this string to every plaintext. You are now doing:
>
>     AES-128-ECB(random-prefix || attacker-controlled || target-bytes, random-KEY)
>
> Same goal: decrypt the target-bytes.
>
> > ### Stop and think for a second.
> > What's harder than challenge #12 about doing this? How would you overcome that obstacle? The hint is: you're using all the tools you already have; no crazy math is required.
> > Think "STIMULUS" and "RESPONSE".