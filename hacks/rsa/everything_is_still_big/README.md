[CryptoHack – RSA challenges](https://cryptohack.org/challenges/rsa/)

> Here is my super-strong RSA implementation, because it's 1600 bits strong it should be unbreakable... at least I think so!
>
> [inferius.py](https://cryptohack.org/static/challenges/inferius_e85eea9b19cd68aa71ce850918302bad.py)
>
> [output.txt](https://cryptohack.org/static/challenges/output_4b843d94b6196df152219c3165b9347f.txt)

> ### How to:
> Run all tests from this package:
>
>     cargo test --package everything_is_still_big --lib tests
>
> Capture the flag with factordb data:
>
>     cargo test --package everything_is_still_big --lib tests::capture_the_flag_db -- --exact
> 
> Capture the flag with reduction:
>
>     cargo test --package everything_is_still_big --lib tests::capture_the_flag_reduction -- --exact

> ### How to build:
> ## [Building on GNU/Linux](https://docs.rs/gmp-mpfr-sys/1.4.5/gmp_mpfr_sys/#building-on-gnulinux)
>
> To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`, `m4` and `make` installed on your system. For example on Fedora:
>
>     sudo dnf install diffutils gcc m4 make
>
>
> ## [Building on macOS](https://docs.rs/gmp-mpfr-sys/1.4.5/gmp_mpfr_sys/#building-on-macos)
>
> To build on macOS, you need the command-line developer tools. To install them, run the following command in a terminal:
>
>     xcode-select --install
>
>
> ## [Building on Windows](https://docs.rs/gmp-mpfr-sys/1.4.5/gmp_mpfr_sys/#building-on-windows)
>
> You can build on Windows with the Rust GNU toolchain and an up-to-date MSYS2 installation. Some steps for a 64-bit environment are listed below. (32-bit: Changes for a 32-bit environment are written in brackets like this comment.)
>
> To install MSYS2:
>
> 1.  Install MSYS2 using the [installer](https://www.msys2.org/).
>
> 2.  Launch the MSYS2 MinGW 64-bit terminal from the start menu. (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)
>
> 3.  Install the required tools.
      >
      >         pacman -S pacman-mirrors
      >         pacman -S diffutils m4 make mingw-w64-x86_64-gcc
>
>
>     (32-bit: Install `mingw-w64-i686-gcc` instead of `mingw-w64-x86_64-gcc`.)
>
>
> Then, to build a crate with a dependency on this crate:
>
> 1.  Launch the MSYS2 MinGW 64-bit terminal from the start menu. (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)
>
> 2.  Change to the crate directory.
>
> 3.  Build the crate using `cargo`.
