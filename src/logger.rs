pub fn setup_logger(path: &Option<String>) -> Result<(), fern::InitError> {
    match path {
        None => {},
        Some(path) => {
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .level(log::LevelFilter::Debug)
                .chain(fern::log_file(path)?)
                .apply()?;
        }
    }
    Ok(())
}
