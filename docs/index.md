# Development documentation

Yuffie is a library for creating [UEFI] applications, libraries, and drivers
in [Rust].

## Installing dependencies

A set of scripts are provided to install dependencies.

```
./scripts/install-deps.sh
./scripts/install-rust.sh
```

### Rust toolchains

A [stable channel][channels] is used for normal Yuffie development. A nightly
channel is required for formatting the code with rustfmt.

The `rust-toolchain.toml` file specifies the exact version of the toolchain to
use, along with required components.

## Building

[Just][just] is used to build the project.

```
just build --release
```

A list of recipes is available with `just help`.


[Rust]: https://www.rust-lang.org/
[UEFI]: https://en.wikipedia.org/wiki/UEFI
[channels]: https://rust-lang.github.io/rustup/concepts/channels.html
[just]: https://github.com/casey/just
