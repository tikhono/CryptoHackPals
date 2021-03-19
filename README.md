> # Crypto Hack Pals
> ## This repo contains solutions of challenges from [Crypto Hack](https://cryptohack.org/) and [Crypto Pals](https://cryptopals.com/).

> # First step: [Install Rust](https://www.rust-lang.org/tools/install)
>
> ## Using rustup (Recommended)
>
> If you’re running macOS, Linux, or another Unix-like OS. To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions. See ["Other Installation Methods"](https://forge.rust-lang.org/infra/other-installation-methods.html) if you are on Windows.
>
>     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

> # Second step: [Build project](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
>
> ## Execute following command to build all libraries:
>
>     cargo build --workspace
> 
> ## Or next from package dir to build only chosen one:
>
>     cargo build
> 
>  Each of libraries contain functions and tests for completing one challenge.

> # Third step: [Run all or only specified tests](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html#adding-a-test-to-a-workspace)
>
> ## To run all tests just execute:
> 
>     cargo test
>
> ## Use flag "nocapture" to see flags from challenges:
> 
>     cargo test -- --nocapture
> 
> ## To run specified test use "package" and "exact" flags:
> 
>     cargto test --package ascii --lib tests::capture_the_flag -- --exact --nocapture
