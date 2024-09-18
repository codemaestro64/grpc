pub mod logic; // Client logic

use crate::grpc::ether_service::ether_service_client::EtherServiceClient;
use crate::grpc::ether_service::GreetRequest;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EtherServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GreetRequest {
        name: "Duncan".into(),
    });

    let response = client.greet(request).await?;

    println!("RESPONSE: {:?}", response);

    Ok(())
}
