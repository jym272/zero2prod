## Configuration
### Linker:
#### Mold as default linker -> [mold](https://github.com/rui314/mold)

Create `.cargo/config.toml` in your project directory with the following:
```
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/path/to/mold"]
```

## Inner Development Loop
`cargo watch` will automatically run `cmd` when a file changes. Example:

`cargo watch -x check -x test -x run`

1. It will start by running cargo check.
2. If it succeeds, it launches cargo test.
3. If tests pass, it launches the application with cargo run.
## Code Coverage
[tarpaulin](https://github.com/xd009642/tarpaulin)`cargo tarpaulin --ignore-tests`