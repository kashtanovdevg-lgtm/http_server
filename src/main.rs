use http_server::run_server;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    run_server().await
}
