> 富强、民主、文明、和谐
>
> 自由、平等、公正、法治
>
> 爱国、敬业、诚信、友善

Let the evil man in the middle know WHAT WE BELIEVE IN :)

[![Build status](https://img.shields.io/travis/QSCTech/BaseCoreSocialistValues/master.svg)](https://travis-ci.org/QSCTech/BaseCoreSocialistValues)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/QSCTech/BaseCoreSocialistValues/blob/master/LICENSE)
[![Crate version](https://img.shields.io/crates/v/base_core_socialist_values.svg)](https://crates.io/crates/base_core_socialist_values)
[![Rust Docs](https://docs.rs/base_core_socialist_values/badge.svg)](https://docs.rs/base_core_socialist_values)

## `Encoder`

Implemented `Write`

```rust
// examples/encoder.rs
use base_core_socialist_values::Encoder;
use std::io::{self, stdout, stdin};

fn main() -> io::Result<()> {
    let mut reader = stdin();
    let mut writer = stdout();
    io::copy(&mut reader, &mut Encoder::new(&mut writer))?;
    Ok(())
}
```

```bash
cargo run --example encoder
```

run, enter and `Ctrl D` (there is buffer in `stdout`)

## `Decoder`

Implemented `Write`

```rust
// examples/decoder.rs
use base_core_socialist_values::Decoder;
use std::io::{self, stdout, stdin};

fn main() -> io::Result<()> {
    let mut reader = stdin();
    let mut writer = stdout();
    io::copy(&mut reader, &mut Decoder::new(&mut writer))?;
    Ok(())
}
```

```bash
cargo run --example decoder
```

run, enter and `Ctrl D` (there is buffer in `stdout`)
