# cp-tools(cpt)

Commandline tools for competitive programming.

## Goals

### Test helper

- Test samples
    - Batch
    - Special judge
    - Reactive
    - Run twice
- Generate testcases
    - Hack-case generation (WA/TLE/RE)

### Expand library code (a.k.a `Bundle`)

- Expansion command binaries
    - [] C++
        - Based on `clang++ -E` command.

## Ambitious Goals

- Network commands
    - Download samples
    - Submission

## Not Goals

- Language specific command wrappers (such as building/executing).
    - Because these commands can be easily wrapped. For example, shell-aliases, Makefile,...
    - `Expand library code` support is the exception of philosophy.
        - This is neccesary for me, and can be complicated to wrap. 

### Limitation

- Only tested for linux

## Installation

1. `cd cargo-core`
  
2. `cargo install --path .`

## Usage


## Credits

See [THIRD-PARTY-LICENSES.toml](THIRD-PARTY-LICENSES.toml). 

This tool is heavily inspired by [oj](https://github.com/online-judge-tools/oj) (Just a reinventing of the wheel...).

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
