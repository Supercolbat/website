use log::LevelFilter;
use time::OffsetDateTime;
use std::{
    io::Write,
    fs::File
};

// Return the time using the local timezone. If that fails, return the time in UTC.
fn get_time_formatted() -> String {
    let date = OffsetDateTime::now_local()
        .unwrap_or(OffsetDateTime::now_utc());
    format!(
        "{}-{}-{}T{:02}:{:02}:{:02}",
        date.year(), date.month(),  date.day(),
        date.hour(), date.minute(), date.second()
    )
}

pub fn init_logging() {
    let now = get_time_formatted();
    let target = Box::new(File::create(format!("logs/{}.txt", now)).expect("Can't create file"));

    env_logger::Builder::new()
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
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Info)
        .init();
}
