# @uldrenai/loom-wasm (WASM binding)

The Uldren Loom Rust core compiled to `wasm32` via wasm-bindgen - the browser/edge path and the universal fallback.

Licensed under **BUSL-1.1** (see the repo `LICENSE`).

## Build

```bash
rustup target add wasm32-unknown-unknown
wasm-pack build --target web --release
# emits pkg/ (js + .wasm + .d.ts)
```

## API

- `version(): string`
- `blob_digest(bytes: Uint8Array): string`
