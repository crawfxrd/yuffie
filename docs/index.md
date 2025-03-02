# Yuffie

Yuffie is a library for creating [UEFI] modules in [Rust].

## Rust toolchain

The `rust-toolchain.toml` file specifies the exact version of the toolchain
used, along with required components. [rustup][install] will manage the
toolchain installation based on this file.

```
rustup show active-toolchain || rustup toolchain install
```

## Building

```
cargo build --target x86_64-unknown-uefi --release
```


[Rust]: https://www.rust-lang.org/
[UEFI]: https://en.wikipedia.org/wiki/UEFI
[install]: https://www.rust-lang.org/tools/install
