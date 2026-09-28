pub mod config;
pub mod trace;

#[tracing::instrument(skip_all)]
pub fn hello(msg: &str) {
    tracing::info!("byebye bye world from the {msg} environment");
}
