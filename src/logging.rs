use env_logger::Builder;
use log::LevelFilter;

pub fn start_logger(log_level: Option<LevelFilter>) {
    let log_level = log_level.unwrap_or(LevelFilter::Trace);

    let mut builder = Builder::from_default_env();
    builder.filter(Some("eth_crawler"), log_level);
    builder.init();
}
