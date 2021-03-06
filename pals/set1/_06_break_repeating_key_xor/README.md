[Challenge 6 Set 1 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/1/challenges/6)

> ### Break repeating-KEY XOR
>
> > ### It is officially on, now.
> > This challenge isn't conceptually hard, but it involves actual error-prone coding. The other challenges in this set are there to bring you up to speed. This one is there to **qualify** you. If you can do this one, you're probably just fine up to Set 6.
>
> [There's a file here.](https://cryptopals.com/static/challenge-data/6.txt) It's been base64'd after being encrypted with repeating-KEY XOR.
>
> Decrypt it.
>
> Here's how:
>
> 1.  Let KEYSIZE be the guessed length of the KEY; try values from 2 to (say) 40.
> 2.  Write a function to compute the edit distance/Hamming distance between two strings. _The Hamming distance is just the number of differing bits._ The distance between:\
>     `this is a test`\
>     and\
>     `wokka wokka!!!`\
>      is **37.** _Make sure your code agrees before you proceed._
> 3.  For each KEYSIZE, take the _first_ KEYSIZE worth of bytes, and the _second_ KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
> 4.  The KEYSIZE with the smallest normalized edit distance is probably the KEY. You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.
> 5.  Now that you probably know the KEYSIZE: break the CIPHERTEXT into blocks of KEYSIZE length.
> 6.  Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.
> 7.  Solve each block as if it was single-character XOR. You already have code to do this.
> 8.  For each block, the single-byte XOR KEY that produces the best looking histogram is the repeating-KEY XOR KEY byte for that block. Put them together and you have the KEY.
>
> This code is going to turn out to be surprisingly useful later on. Breaking repeating-KEY XOR ("Vigenere") statistically is obviously an academic exercise, a "Crypto 101" thing. But more people "know how" to break it than can _actually break it_, and a similar technique breaks something much more important.
>
> > ### No, that's not a mistake.
> > We get more tech support questions for this challenge than any of the other ones. We promise, there aren't any blatant errors in this text. In particular: the "wokka wokka!!!" edit distance really is 37.


> ### How to:
> Run all tests from this package:
>
>     cargo test --package _06_break_repeating_key_xor --lib tests
>
> Capture the flag:
>
>     cargo test --package _06_break_repeating_key_xor --lib tests::capture_the_flag -- --exact
> 
> Test distance:
>
>     cargo test --package _06_break_repeating_key_xor --lib tests::test_distance -- --exact
> 
> Change rust toolchain:
> 
>     rustup override set nightly
