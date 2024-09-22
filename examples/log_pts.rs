#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

fn main() -> anyhow::Result<()> {
    let log_backend = std::env::args().nth(1).expect("no file for logging");
    println!("\n----> {}", log_backend);

    raalog::init()?
        .set_file_mode(&log_backend)?
        .set_level(raalog::LevelFilter::Trace);

    log::error!("mini error");
    log::info!("mini info");

    Ok(())
}
