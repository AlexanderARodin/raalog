#[test]
fn simple_stdout_stderr() -> anyhow::Result<()> {
    let l = raalog::init()?;

    l.set_stdout_mode();
    log::info!("mini info");

    l.set_stderr_mode();
    log::info!("mini info");

    l.set_silent_mode();
    log::error!("mini info");

    Ok(())
}
#[test]
fn mutliline_stderr() -> anyhow::Result<()> {
    let l = raalog::init()?;
    l.set_stderr_mode();

    log::info!("");
    log::info!("mini\ninfo");
    log::error!("mini\ninfo\nrmation!");

    Ok(())
}
