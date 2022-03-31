use log::LevelFilter;

fn main() {
    let mut logger = env_logger::Builder::new();
    logger
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    terrarium::run();
}
