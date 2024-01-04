# zarthus_env_logger

[![version](https://img.shields.io/crates/v/zarthus_env_logger?color=blue&logo=rust&style=flat-square)](https://crates.io/crates/zarthus_env_logger)

The most opinionated logger you'll find.

Does nothing if `RUST_LOG` is set.

## Usage

```bash
cargo add log
cargo add zarthus_env_logger
```

```rust
#[macro_use]
extern crate log;

fn main() {
    zarthus_env_logger::init();

    info!("Hello, world!");    
}
```

### What does it do differently?

- Absolutely nothing if `RUST_LOG` is set.
- The name of the package is shortened (to `@`)
- Log filters to `debug` by default for your own package, and `error` for all other packages.
- Timestamps are handled with `chrono` or `time` (if possible), and included by default
- Opinionated colouring

![image/ascii.gif](image/ascii.gif)

## License

Licensed under the following licenses at your option:

- Apache License, Version 2.0 <[LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0>
- MIT license <[LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT>

Files in the project may not be copied, modified, or distributed except according to those terms.
