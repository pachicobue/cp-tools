# cp-tools(cpt)

Commandline tools for competitive programming.

## Features

- Build/Run wrapper (w/ language detection)
- Expand library imports/includes in source file (a.k.a 'Bundle')
    - ⚠ Now only supports C++. Please specify your own command for other langs.
 - Customizable command
    - Build/Run/Expand command can be specified with `language.toml`

### Future support

- Test utilities
    - Test
    - Generate input/output
    - Generate hack case

### Limitation

- Only tested for linux
- Network command such as 'Download samples' or 'Submit file' not supported.

## Installation

`cargo install --path .`

## Usage

Build/Run/Expand commands are determined by `language.toml`.  
You can place your own `~/.local/share/cpt/language.toml` to overwrite default.

### Build

- `cpt build <src_file> [-o dst_file] [--release]`

### Run

- `cpt run <src_file> [-o dst_file] [--release]`

### Expand

- `cpt expand <src_file> [-o dst_file]`

## Credits

See [THIRD-PARTY-LICENSES.toml](THIRD-PARTY-LICENSES.toml). 

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
