use log::LevelFilter;
use std::io::Write;

fn main() {
    match std::env::var("RUST_LOG_STYLE") {
        Ok(s) if s == "SYSTEMD" => env_logger::builder()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "<{}>{}: {}",
                    match record.level() {
                        log::Level::Error => 3,
                        log::Level::Warn => 4,
                        log::Level::Info => 6,
                        log::Level::Debug => 7,
                        log::Level::Trace => 7,
                    },
                    record.target(),
                    record.args()
                )
            })
            .filter(None, LevelFilter::Info)
            .parse_default_env()
            .init(),
        _ => env_logger::builder()
            .filter_level(LevelFilter::Info)
            .parse_default_env()
            .init(),
    };

    terrarium::run();
}
