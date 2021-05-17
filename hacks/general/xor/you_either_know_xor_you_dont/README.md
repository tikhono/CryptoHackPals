[CryptoHack – General challenges](https://cryptohack.org/challenges/general/)

> I've encrypted the flag with my secret KEY, you'll never be able to guess it.
>
> Remember the flag format and how it might help you in this challenge!
>
>     0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104

> ### How to:
> Run all tests from this package:
>
>     cargo test --package you_either_know_xor_you_dont --lib tests
>
> Capture the flag:
>
>     cargo test --package you_either_know_xor_you_dont --lib tests::capture_the_flag -- --exact
