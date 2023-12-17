use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentRequest, BtcPaymentResponse};
use tonic::{Request, Response, Status, transport::Server};

pub mod payments{
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService{}

#[tonic::async_trait]
impl Bitcoin for BitcoinService{
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>
    ) -> Result<Response<BtcPaymentResponse>, Status>{
        let req = request.into_inner();
        println!("Request = {:?}", req);
        let reply = BtcPaymentResponse{
            success: true,
            response : format!("Hello, request recieved from {:?} to {:?} for {:?}BTC", req.from_addr, req.to_addr, req.amount).into()
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();

    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}