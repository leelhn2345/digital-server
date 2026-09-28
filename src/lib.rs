pub mod config;
pub mod trace;

#[tracing::instrument(skip_all)]
pub fn hello(msg: impl Into<String>) {
    let msg = msg.into();
    tracing::info!("byebye bye world from the {msg} environment");
}
