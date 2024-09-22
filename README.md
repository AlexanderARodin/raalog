# primitive implementation of [log](https://github.com/rust-lang/log) for [Rust](https://github.com/rust-lang)

## usage
```rust
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};
```

## not implemented (yet)
- `flush()`

## logging modes
- [x] silent
- [x] to Buffer
- [x] to StdErr
- [x] to StdOut
- [x] to File
