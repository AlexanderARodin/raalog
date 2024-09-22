#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

fn main() -> anyhow::Result<()> {
    let log_backend = std::env::args().nth(1).expect("no file for logging");
    println!("\n----> {}", log_backend);

    let l = raalog::init()?
        .set_file_mode("/dev/pts/2")?
        .set_level(raalog::LevelFilter::Trace);

    let _ = l.set_file_mode(&log_backend);
    log::error!("mini error");
    log::info!("mini info");

    Ok(())
}
