#[test]
fn simple_stdout() -> anyhow::Result<()> {
    let l = raalog::init()?
        .set_file_mode("/dev/pts/222")?;

    log::info!("mini info");
    log::info!("mini info");
    log::error!("mini info");

    Ok(())
}
