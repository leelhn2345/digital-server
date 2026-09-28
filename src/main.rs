use server::config;
use server::hello;
use server::trace::init_tracing;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let env = config.get_environment();
    init_tracing(env, config.get_log_targets());

    tracing::debug!("{config:#?}");
    hello(env.as_str());
    let pool = config.get_database().get_connection_pool().await;
}
