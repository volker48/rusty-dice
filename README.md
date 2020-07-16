# rusty-dice

Silly little app that uses the EFF large word list to generate [diceware](https://en.wikipedia.org/wiki/Diceware) passwords. This is mostly just a small project to help me learn Rust.

# Usage

By default with no arguments dicey will generate a 6 word passphrase.
    cargo run


If you want a longer passphrase provide a single integer argument of the number of words you want.

    cargo run 10

