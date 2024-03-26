use std::time::SystemTime;
use colored::Colorize;
use log::Level;

pub fn setup_logger() -> Result<(), fern::InitError> {
    let output_file = "output.log";
    fern::Dispatch::new()
        .format(|out, message, record| {
            let time = humantime::format_rfc3339_seconds(SystemTime::now());
            let level = record.level();
            let level = match level {
                Level::Error => format!("{level}").red(),
                Level::Warn => format!("{level}").yellow(),
                level => format!("{level}").normal(),
            };
            out.finish(format_args!(
                "[{} {} {}] {}",
                format!("{}", time).blue(),
                level,
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .filter(|metadata| !metadata.target().starts_with("rocket")) // Disable rocket logging
        .chain(std::io::stdout())
        .chain(fern::log_file(output_file)?)
        .apply()?;
    Ok(())
}
