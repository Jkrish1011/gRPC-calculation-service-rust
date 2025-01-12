use proto::calculator_client::CalculatorClient;
use std::error::Error;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url: &str = "http://[::1]:50001";
    let mut client: CalculatorClient<tonic::transport::Channel> =
        CalculatorClient::connect(url).await?;

    let req: proto::CalculationRequest = proto::CalculationRequest { a: 4, b: 5 };
    let request: tonic::Request<proto::CalculationRequest> = tonic::Request::new(req);

    let response: tonic::Response<proto::CalculationResponse> = client.add(request).await?;

    println!("Response: {:?}", response.get_ref().result);
    Ok(())
}
