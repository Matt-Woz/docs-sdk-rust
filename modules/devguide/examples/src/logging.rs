use log::LevelFilter;
use std::io::Write;

pub fn env_logger() {
    // tag::env-logger[]
    env_logger::init();
    // end::env-logger[]

    // tag::env-logger-extended[]
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("rustls"), LevelFilter::Warn)
        .init();
    // end::env-logger-extended[]
}
