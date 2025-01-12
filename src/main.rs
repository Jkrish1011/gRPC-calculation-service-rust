// Created by the tonic package automatically
use proto::calculator_server::{Calculator, CalculatorServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    // tonic::Request<proto::CalculationRequest> - A tonic request which encapsulates a CalculationRequest
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        println!("Got a Addition request: {:?}", request);
        let input = request.get_ref();

        let response: proto::CalculationResponse = proto::CalculationResponse {
            result: input.a + input.b,
        };
        Ok(tonic::Response::new(response))
    }

    async fn divide(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        println!("Got a division Request: {:?}", request);

        let input = request.get_ref();

        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("Division by zero!"));
        }

        let response: proto::CalculationResponse = proto::CalculationResponse {
            result: input.a / input.b,
        };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = "[::1]:50001".parse()?;

    let calc: CalculatorService = CalculatorService::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;
    Ok(())
}
