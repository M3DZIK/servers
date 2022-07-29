use tracing::metadata::LevelFilter;

pub fn init() {
    better_panic::install();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();
}
