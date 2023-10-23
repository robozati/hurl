mod scanner;
mod server;
mod token;

#[tokio::main]
async fn main() {
    server::run().await;
}
