#[utoipa::path(
    get,
    tag = "health_check",
    path = "/hello",
    responses(
        (status = 200, description = "list of available restaurants"))
)]
pub async fn hello_world() -> String {
    "hello world".to_string()
}
