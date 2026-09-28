pub mod config;
pub mod trace;

#[tracing::instrument]
pub fn hello() {
    tracing::info!("byebye world");
}
