# xkcp-rs

Bindings and wrappers to the [eXtended Keccak Code Package (XKCP)](https://github.com/XKCP/XKCP) library.

## Requirements

To build on Linux or macOS, the following tools are needed:
- [GCC](https://gcc.gnu.org/) or [Clang](https://clang.llvm.org/)
- [GNU `make`](https://www.gnu.org/software/make/)
- `xsltproc` (part of [`libxslt`](https://gitlab.gnome.org/GNOME/libxslt/-/wikis/home))

Windows is not supported at the moment.

CC flags are selected using [`cc-rs`](https://github.com/rust-lang/cc-rs).
Refer to [their documentation](https://github.com/rust-lang/cc-rs#external-configuration-via-environment-variables)
on how to configure `cc-rs` externally.

### XKCP targets

[XKCP targets](https://github.com/XKCP/XKCP#how-can-i-build-the-xkcp) are automatically selected through
`CARGO_*` environment variables. This behavior can be overridden by using
feature flags ([see below](#feature-flags)) or by directly specifying the XKCP
target with the `XKCP_RS_TARGET` environment variable.

Refer to the [XKCP documentation](https://github.com/XKCP/XKCP#how-can-i-build-the-xkcp) for more details.

## Usage

`Cargo.toml`:

```toml
[dependencies]
xkcp-rs = "0.0.2"
```

`src/main.rs`:

```rust
fn main() {
    let mut output = [0u8; 32];
    xkcp_rs::sha3_256(b"Hello, World!", &mut output);
    println!("{output:x?}");
}
```

## Feature flags

`xkcp-rs` only:
- `std`: Enable `std`-only features, like implementations of [`std::error::Error`](https://doc.rust-lang.org/stable/std/error/trait.Error.html). Enabled by default.

`xkcp-rs` and `xkcp-sys`:
- `avr8`: Forces building for the `AVR8` (8-bit AVR) XKCP target.
- `force-compact`: Forces building for the `compact` XKCP target. **WARNING**: this is generally much slower than anything else.
- `force-generic`: Forces building for the `generic32` or `generic64` XKCP target. Falls back to `compact` if not building for a 32 or 64 bit architecture.
- `generic-lc`: Uses the `generic{32,64}lc` XKCP targets instead of `generic{32,64}`.

## Supported Rust Versions

<!--
When updating this, also update:
- clippy.toml
- Cargo.toml
- .github/workflows/ci.yml
-->

xkcp-rs will keep a rolling MSRV (minimum supported rust version) policy of **at
least** 6 months. When increasing the MSRV, the new Rust version must have been
released at least six months ago. The current MSRV is 1.65.0.

Note that the MSRV is not increased automatically, and only as part of a minor
release.

## License

xkcp-rs redistributes the XKCP library, `libXKCP`, which is mostly released to
the **public domain** and associated to the [CC0](http://creativecommons.org/publicdomain/zero/1.0/)
deed, but there are exceptions.
Please refer to the [LICENSE](LICENSE) file for more information.
