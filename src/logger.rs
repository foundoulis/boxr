use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
// use log::{debug, error, info, trace, warn};

#[mutants::skip]
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
                // chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
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
