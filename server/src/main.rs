use server::init_app;

#[tokio::main]
async fn main() {
    init_app().await;
}
