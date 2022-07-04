# near

## Reference
[near](https://docs.near.org/docs/concepts/new-to-near)

## Run
### Build
```
cargo build --all
```

### Format
```
cargo +nightly fmt
```

### Lint
```
cargo clippy --all --all-targets --release
```

### Test
```
ENVIRONMENT=.env cargo test -all
```