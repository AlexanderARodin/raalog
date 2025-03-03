# primitive implementation of [log](https://github.com/rust-lang/log) for [Rust](https://github.com/rust-lang)

## usage
```rust
use eyre::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

fn main() -> Result<()> {
    log_init();

    trace!("############\n<-----\n.\n ");
    Ok(())
}

//  //  //  //  //  //  //  //
fn log_init() {
    raalog::init()
        .expect("unable init log system")
        .set_file_mode(&"/tmp/rust_debug.log")
        .expect("unable to set file mode of logger")
        .set_level(raalog::LevelFilter::Trace);

    trace!("\n.\n----->\n############");
    set_panic_hook();
}

//  //  //  //  //  //  //  //
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        error!("############\nFATAL!\n{}\n<-----\n.\n ", info);
        hook(info);
    }));
}
```

## not implemented (yet)
- `flush()`

## logging modes
- [x] silent
- [x] to Buffer
- [x] to StdErr
- [x] to StdOut
- [x] to File

## others
- [x] multi-line
