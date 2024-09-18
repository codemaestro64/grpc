pub mod service; // Server service logic

use tonic::{transport::Server};
use crate::grpc::ether_service::ether_service_server::{EtherService, EtherServiceServer};
use service::EtherServiceImpl;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = EtherServiceImpl::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(EtherServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
