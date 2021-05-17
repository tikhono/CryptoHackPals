[CryptoHack – General challenges](https://cryptohack.org/challenges/general/)

> Now you've got the hang of the various encodings you'll be encountering, let's have a look at automating it.
>
> Can you pass all 100 levels to get the flag?
>
> The _13377.py_ file attached below is the source code for what's running on the server. The _pwntools\_example.py_ file provides the start of a solution using the incredibly convenient pwntools library. which you can use if you like. pwntools however is incompatible with Windows, so _telnetlib\_example.py_ is also provided.
>
> For more information about connecting to interactive challenges, see the [FAQ](https://cryptohack.org/faq#netcat). Feel free to skip ahead to the cryptography if you aren't in the mood for a coding challenge!
>
> Connect at `nc socket.cryptohack.org 13377`
>
> [13377.py](https://cryptohack.org/static/challenges/13377_43de0a0efed6ed7bd890d1c79db22fb1.py)
>
> [pwntools\_example.py](https://cryptohack.org/static/challenges/pwntools_example_f93ca6ccef2def755aa8f6d9aa6e9c5b.py)
>
> [telnetlib\_example.py](https://cryptohack.org/static/challenges/telnetlib_example_5b11a835055df17c7c8f8f2a08782c44.py)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package encoding_challenge --lib tests
>
> Capture the flag:
>
>     cargo test --package encoding_challenge --lib tests::capture_the_flag -- --exact
>
> Test json builder:
>
>     cargo test --package encoding_challenge --lib tests::json_converter -- --exact
