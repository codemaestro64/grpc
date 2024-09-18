use grpc::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    client::run().await
}
