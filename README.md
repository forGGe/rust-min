# Minimal Embedded Rust Template

This template is nothing but a delight for a seasoned C programmer:

* only ~100 bytes of code to blink an LED
* everything is unsafe
* basic `core` dependencies
* function that sets registers with no checks, like good old C does
* no pesky HAL to hide important register details
* can support only one MCU at a time and this is great

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

# ...

$ cargo size
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
   text	   data	    bss	    dec	    hex	filename
   4341	      0	      0	   4341	   10f5	rust_min

```

## Flash

```sh
$ cargo flash --chip STM32F303RBTx --connect-under-reset --speed 100
```

---

## Build Release

```sh
$ cargo build
```

## Analyze Release

```sh
$ cargo nm --release
    Finished release [optimized] target(s) in 0.00s
08000004 R RESET_VECTOR
08000008 T Reset

$ cargo size --release
    Finished release [optimized] target(s) in 0.00s
   text	   data	    bss	    dec	    hex	filename
     92	      0	      0	     92	     5c	rust_min
```

## Flash Release

```sh
$ cargo flash --release --chip STM32F303RBTx --connect-under-reset --speed 100
```