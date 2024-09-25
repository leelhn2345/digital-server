use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::Environment;

pub fn init_tracing(env: &Environment, targets: Vec<&str>) {
    let trace_level = match &env {
        Environment::Local => LevelFilter::DEBUG,
        Environment::Production => LevelFilter::INFO,
    };
    let targets_with_level: Vec<(&str, LevelFilter)> =
        targets.into_iter().map(|s| (s, trace_level)).collect();

    let target_filter = Targets::new().with_targets(targets_with_level);

    let format_layer = fmt::layer()
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(format_layer)
        .with(target_filter)
        .init();
}
