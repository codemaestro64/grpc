use tonic::{Request, Response, Status};
use crate::grpc::ether_service::{GreetRequest, GreetResponse};
use crate::grpc::ether_service::ether_service_server::EtherService;

#[derive(Default)]
pub struct EtherServiceImpl;

#[tonic::async_trait]
impl MyService for EtherServiceImpl {
    async fn say_hello(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        let response = GreetResponse {
            message: format!("Hello, {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }
}
