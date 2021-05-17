[CryptoHack – General challenges](https://cryptohack.org/challenges/general/)

> ASCII is a 7-bit encoding standard which allows the representation of text using the integers 0-127.
>
> Using the below integer array, convert the numbers to their corresponding ASCII characters to obtain a flag.
>
>     [99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125]

> ### How to:
> Run all tests from this package:
>
>     cargo test --package ascii --lib tests
> 
> Capture the flag:
> 
>     cargo test --package ascii --lib tests::capture_the_flag -- --exact
> 
> Test ascii crate:
> 
>     cargo test --package ascii --lib tests::test_ascii_crate -- --exact
