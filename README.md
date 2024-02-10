## Preparation

```sh
# Proper toolchain and target support
$ rustup update
$ rustup target add thumbv7m-none-eabi

# Nice tools
$ cargo install cargo-binutils
$ rustup component add llvm-tools
```

## Build

```sh
$ cargo build
```

## Analyze

```sh
$ cargo nm
$ cargo size
```

## Flash

