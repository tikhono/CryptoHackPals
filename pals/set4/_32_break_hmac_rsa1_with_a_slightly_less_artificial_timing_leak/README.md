[Challenge 32 Set 4 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/4/challenges/32)

> ### Break HMAC-SHA1 with a slightly less artificial timing leak
>
> Reduce the sleep in your "insecure\_compare" until your previous solution breaks. (Try 5ms to start.)
>
> Now break it again.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _32_break_hmac_rsa1_with_a_slightly_less_artificial_timing_leak --lib tests
>
> Capture the flag:
>
>     cargo test --package _32_break_hmac_rsa1_with_a_slightly_less_artificial_timing_leak --lib tests::capture_the_flag -- --exact
