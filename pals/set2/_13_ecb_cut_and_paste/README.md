[Challenge 13 Set 2 - The Cryptopals Crypto Challenges](https://cryptopals.com/sets/2/challenges/13)

> ### ECB cut-and-paste
>
> Write a k=v parsing routine, as if for a structured cookie. The routine should take:
>
> `foo=bar&baz=qux&zap=zazzle`
>
> ... and produce:
>
>     {
>         foo: 'bar',
>         baz: 'qux',
>         zap: 'zazzle'
>     }
>
> (you know, the object; I don't care if you convert it to JSON).
>
> Now write a function that encodes a user profile in that format, given an email address. You should have something like:
>
>     profile_for("foo@bar.com")
>
> ... and it should produce:
>
>     {
>         email: 'foo@bar.com',
>         uid: 10,
>         role: 'user'
>     }
>
> ... encoded as:
>
>     email=foo@bar.com&uid=10&role=user
>
> Your "profile_for" function should _not_ allow encoding metacharacters (& and =). Eat them, quote them, whatever you want to do, but don't let people set their email address to "foo@bar.com&role=admin".
>
> Now, two more easy functions. Generate a random AES KEY, then:
>
> 1.  Encrypt the encoded user profile under the KEY; "provide" that to the "attacker".
> 2.  Decrypt the encoded user profile and parse it.
>
> Using only the user input to profile\_for() (as an oracle to generate "valid" ciphertexts) and the ciphertexts themselves, make a role=admin profile.

> ### How to:
> Run all tests from this package:
>
>     cargo test --package _13_ecb_cut_and_paste --lib tests
>
> Test parse:
>
>     cargo test --package _13_ecb_cut_and_paste --lib tests::test_parse -- --exact
>
> Test profile_for:
>
>     cargo test --package _13_ecb_cut_and_paste --lib tests::test_profile_for -- --exact
>
> Test oracles:
>
>     cargo test --package _13_ecb_cut_and_paste --lib tests::test_oracles -- --exact
>
> Capture the flag:
>
>     cargo test --package _13_ecb_cut_and_paste --lib tests::capture_the_flag -- --exact
>
