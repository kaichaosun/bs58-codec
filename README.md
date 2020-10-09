# Base58 converter for PeerId

## Install

```shell
cargo install wasm-pack
cargo +nightly install miniserve
```

## Run

```shell
wasm-pack build --target web --out-name wasm --out-dir ./static
miniserve ./static --index index.html
```