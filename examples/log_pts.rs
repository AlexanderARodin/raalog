fn main() -> anyhow::Result<()> {
    let log_backend = std::env::args().nth(1).expect("no file for logging");
    println!("\n----> {}", log_backend);

    let l = raalog::init()?;

    let _ = l.set_file_mode(&log_backend);
    log::error!("mini error");
    log::info!("mini info");

    Ok(())
}
