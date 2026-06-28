<h1><a href="https://itch.io/jam/killyourdarlingsno1">KILL YOUR DARLINGS Mini Jam No. 1</a></h1>

## Feather Falling IV

A simple 2D game.  Our submission to the KILL YOUR DARLINGS game jam.  The idea is to make _anything_, rather than creating something overly ambitious and ultimately falling short.

### Quick Start

```commandline
$ cargo run
```

### Compiling to WASM

To compile an archive with WASAM (for uploading to itch.io), you will need `wasm-bindgen`.  However, the `wasm-bindgen` version must match the version specified in [`Cargo.toml`](./Cargo.toml)`.  At time of writing, that is v0.2.106:

```commandline
$ cargo install -f wasm-bindgen-cli --version 0.2.106
```

Then you can run

```commandline
$ ./wasm.sh
```
