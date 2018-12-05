pub mod application;
pub mod core_macros;
pub mod entry_point;

#[cfg(debug_assertions)]
pub fn setup_logger() ->Result<(), fern::InitError> {

    use fern::colors::Color;
    use fern::colors::ColoredLevelConfig;

    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::Green)
        .trace(Color::Blue);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                    "{color_line}[{date}][{target}][{level}] {message}\x1B[0m",
                    color_line = format_args!("\x1B[{}m", colors_line.get_color(&record.level()).to_fg_str()),
                    date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    target = record.target(),
                    level = record.level(),
                    message = message,
                    ));
        })
    .level(log::LevelFilter::Trace)
        .chain(std::io::stderr())
        .chain(fern::log_file("magnus_log.log")?)
        .apply()?;
    Ok(())
}

#[cfg(not(debug_assertions))]
pub fn setup_logger() ->Result<(), fern::InitError> {

    use fern::colors::Color;
    use fern::colors::ColoredLevelConfig;

    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::Green)
        .trace(Color::Blue);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                    "{color_line}[{date}][{target}][{level}] {message}\x1B[0m",
                    color_line = format_args!("\x1B[{}m", colors_line.get_color(&record.level()).to_fg_str()),
                    date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    target = record.target(),
                    level = record.level(),
                    message = message,
                    ));
        })
    .level(log::LevelFilter::Warn)
        .chain(std::io::stderr())
        .chain(fern::log_file("magnus_log.log")?)
        .apply()?;
    Ok(())
}
