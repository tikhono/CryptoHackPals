[CryptoHack – Block Ciphers challenges](https://cryptohack.org/challenges/aes/)

> If a block cipher is secure, there should be no way for an attacker to distinguish the output of AES from a [random permutation](https://en.wikipedia.org/wiki/Pseudorandom_permutation) of bits. Furthermore, there should be no better way to undo the permutation than simply bruteforcing every possible KEY. That's why academics consider a cipher theoretically "broken" if they can find an attack that takes fewer steps to perform than bruteforcing the KEY, even if that attack is practically infeasible.
>
> How difficult is it to bruteforce a 128-bit keyspace? [Somebody estimated](https://crypto.stackexchange.com/a/48669) that if you turned the power of the entire Bitcoin mining network against an AES-128 KEY, it would take over a hundred times the age of the universe to crack the KEY.
>
> It turns out that there is [an attack](https://en.wikipedia.org/wiki/Biclique_attack) on AES that's better than bruteforce, but only slightly – it lowers the security level of AES-128 down to 126.1 bits, and hasn't been improved on for over 8 years. Given the large "security margin" provided by 128 bits, and the lack of improvements despite extensive study, it's not considered a credible risk to the security of AES. But yes, in a very narrow sense, it "breaks" AES.
>
> Finally, while quantum computers have the potential to completely break popular public-KEY cryptosystems like RSA via [Shor's algorithm](https://en.wikipedia.org/wiki/Shor%27s_algorithm), they are thought to only cut in half the security level of symmetric cryptosystems via [Grover's algorithm](https://en.wikipedia.org/wiki/Grover's_algorithm). This is one reason why people recommend using AES-256, despite it being less performant, as it would still provide a very adequate 128 bits of security in a quantum future.
>
> What is the name for the best single-KEY attack against AES?
> 
> [**Biclique**](https://en.wikipedia.org/wiki/Biclique_attack)
