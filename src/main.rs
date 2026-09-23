use server::config;
use server::hello;
use server::logs::init_tracing;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let env = config.get_environment();
    init_tracing(env, &["server"]);
    println!("{env:#?}");
    hello();
}
