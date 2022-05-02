# Developing

## Requirements
1. Rust - [https://rustup.rs/](https://rustup.rs/)
2. Wasm-pack - [https://rustwasm.github.io/wasm-pack/](https://rustwasm.github.io/wasm-pack/)

### Unit Tests
```
cargo test
```

### Building

```
wasm-pack build
```

### Browser Integration Tests

```
wasm-pack test --headless --firefox
```