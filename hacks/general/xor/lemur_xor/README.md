[CryptoHack – General challenges](https://cryptohack.org/challenges/general/)

> I've hidden two cool images by XOR with the same secret KEY so you can't see them!
>
> [lemur.png](https://cryptohack.org/static/challenges/lemur_ed66878c338e662d3473f0d98eedbd0d.png)
>
> [flag.png](https://cryptohack.org/static/challenges/flag_7ae18c704272532658c10b5faad06d74.png)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package lemur_xor --lib tests
>
> Capture the flag:
>
>     cargo test --package lemur_xor --lib tests::capture_the_flag -- --exact
