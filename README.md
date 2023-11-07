# cha-rs

[![Crates.io](https://img.shields.io/crates/v/cha-rs.svg)](https://crates.io/crates/cha-rs)
[![Docs.rs](https://docs.rs/cha-rs/badge.svg)](https://docs.rs/cha-rs)
[![CI](https://github.com/jj-style/cha-rs/workflows/CI/badge.svg)](https://github.com/jj-style/cha-rs/actions)

A simple library and app to extract specific characters from an input string. Useful for when you need to verify arbitrary characters of your long random password for your bank.  
For example:  
```bash
$ bw get password <mybank> | cha-rs -c 3 -c 11 -c 16
# 3 => C
# 11 => !
# 16 => p
```
(Please note, those characters are made up and do not belong to any of my passwords! :D)

Originally I created a pull request to Bitwarden ([https://github.com/bitwarden/clients/pull/5042](https://github.com/bitwarden/clients/pull/5042)) to add this functionality to the command line app. But that feels a bit over-engineered and it is more inline with the [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) to create a small program that does one thing well, accepting text as its input, and writing text as its output. So here it is!  

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install cha-rs`

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).
