## Configuration
### Linker:
#### Mold as default linker -> [mold](https://github.com/rui314/mold)

Create `.cargo/config.toml` in your project directory with the following:
```
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/path/to/mold"]
```