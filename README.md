# cp-tools(cpt)

Command line tools for competitive programming.

## Goals

### Test helper

- Test samples
    - [x] Batch
    - [x] Special judge
    - [x] Reactive
    - [ ] Run twice
- Hackcase generation(WA/RE/TLE)
    - [x] Batch
    - [x] Special judge
    - [x] Reactive
    - [ ] Run twice

### Expand library code (a.k.a `Bundle`)

- Expansion command binaries
    - [x] C++
        - Based on `clang++ -E` command.

## Ambitious Goals

- Network commands
    - Download samples
    - Submission

## Installation

- `cargo install --path cpt-core`
- `cargo install --path cpt-extras`

## Usage

There are some hidden (not neccesary) arguments.
(eg) `--tl` for timelimit.

Check `cpt --help` for more information.

### Test 

#### Batch (Most basic test)

**(Notice) Judge with absolute/relative error is not supported.
Use special judge for this situation.**

```sh
cpt test batch -c "./main.exe" -d test

(short version)
cpt t b -c "./main.exe" -d test
```

- arguments
    - `-c`: command
    - `-d`: directory path which contains testcases
        - Intermediate files (including debug outputs from stderr) will be stored in this directory.

#### Special judge

```sh
cpt test batch -c "./main.exe" -j "./judge.exe" -d test

(short version)
cpt t s -c "./main.exe" -j "./judge.exe" -d test
```

- arguments
    - `-c`: command
    - `-j`: judge command
        - Judge command should take two arguments.  
        - `<judge_command> <input_path> <output_path>`
    - `-d`: directory path which contains testcases

#### Reactive judge


```sh
cpt test reactive -c "./main.exe" -j "./judge.exe" -d test

(short version)
cpt t r -c "./main.exe" -j "./judge.exe" -d test
```

- arguments
    - `-c`: command
    - `-j`: judge command
        - Judge command should take one arguments.  
        - `<judge_command> <input_path>`
    - `-d`: directory path which contains testcases

### Hackcase Generation

#### Batch

```sh
cpt hack batch -c "./main.exe" -i "./gen_input.exe" -d test

(short version)
cpt t b -c "./main.exe" -i "./gen_input.exe" -d test
```

- arguments
    - `-c`: command
    - `-i`: input generator
    - `-o`: *(Optional)* output generator
    - `-d`: directory path to generate testcase

#### Special judge

```sh
cpt hack special -c "./main.exe" -i "./gen_input.exe" -j "./judge.exe" -d test

(short version)
cpt h s -c "./main.exe" -i "./gen_input.exe" -j "./judge.exe" -d test
```

- arguments
    - `-c`: command
    - `-i`: input generator
    - `-j`: judge command
        - `<judge_command> <input_path> <output_path>`
    - `-d`: directory path which contains testcases

#### Reactive judge

```sh
cpt hack reactive -c "./main.exe" -i "./gen_input.exe" -j "./judge.exe" -d test

(short version)
cpt t r -c "./main.exe" -j "./judge.exe" -d test
```

- arguments
    - `-c`: command
    - `-i`: input generator
    - `-j`: judge command
        - `<judge_command> <input_path>`
    - `-d`: directory path which contains testcases

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
