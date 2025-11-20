use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

pub fn setup_logger(level: LevelFilter) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .trace(Color::BrightBlack)
        .debug(Color::BrightBlue)
        .info(Color::BrightGreen)
        .warn(Color::BrightYellow)
        .error(Color::BrightRed);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{:<5}] {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
