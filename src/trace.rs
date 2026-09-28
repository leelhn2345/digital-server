use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    Layer, filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::config::environment::Environment;

pub fn init_tracing(env: &Environment, targets: &[String]) {
    let trace_level = match &env {
        Environment::Development | Environment::Staging => LevelFilter::DEBUG,
        Environment::Production => LevelFilter::INFO,
    };

    let targets_with_level: Vec<(String, LevelFilter)> =
        targets.iter().map(|s| (s.into(), trace_level)).collect();

    let target_filter = Targets::new().with_targets(targets_with_level);

    let format_layer = fmt::layer()
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    let format_layer = match env {
        Environment::Production => format_layer.json().flatten_event(true).boxed(),
        Environment::Development | Environment::Staging => format_layer.boxed(),
    };

    tracing_subscriber::registry()
        .with(format_layer)
        .with(target_filter)
        .init();
}
