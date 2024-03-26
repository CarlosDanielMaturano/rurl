use std::time::SystemTime;

pub fn setup_logger() -> Result<(), fern::InitError> {
    let output_file = "output.log";
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .filter(|metadata| !metadata.target().starts_with("rocket")) // Disable rocket logging
        .chain(std::io::stdout())
        .chain(fern::log_file(output_file)?)
        .apply()?;
    Ok(())
}
