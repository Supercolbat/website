use env_logger::{
    Target,
    WriteStyle
};
use log::LevelFilter;
use time::OffsetDateTime;
use std::{
    io::Write,
    fs::File
};

/// Return the time using the local timezone. If that fails, return the time in UTC.
fn get_time_formatted() -> String {
    let date = OffsetDateTime::now_local()
        .unwrap_or(OffsetDateTime::now_utc());
    format!(
        "{}-{}-{}T{:02}:{:02}:{:02}",
        date.year(), date.month(),  date.day(),
        date.hour(), date.minute(), date.second()
    )
}

/// Initializes `env_logger`
pub fn init_logging() {
    env_logger::Builder::new()
        // Define the logging format
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}] {}",
                get_time_formatted(),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args(),
            )
        })

        // If debugging is disabled, log to a file instead of STDOUT
        .target(
            if cfg!(debug_assertions) {
                Target::Stdout
            } else {
                let target = Box::new(File::create(format!("logs/{}.txt", get_time_formatted())).expect("Can't create file"));
                Target::Pipe(target)
            }
        )

        // Minimum logging level
        .filter(None, LevelFilter::Info)

        // Enable colors when possible
        .write_style(WriteStyle::Auto)

        .init();
}
