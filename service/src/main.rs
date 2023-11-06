use server::vix_flix_api::{ImportMediaRequest, ImportMediaResponse, storage_api_server::{StorageApi, StorageApiServer}, QueryMediaRequest, QueryMediaResponse};

use tonic::{Request, Status, Response, transport::Server};

pub mod server;



#[derive(Debug, Default)]
pub struct API {}

#[tonic::async_trait]
impl StorageApi for API {
    async fn import_media(
        &self,
        request: Request<ImportMediaRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<ImportMediaResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a import_media: {:?}", request);

        let reply = ImportMediaResponse { message: "Good".to_string(), video_id: "0".to_string(), path: "test" .to_string()};

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn query_media(
        &self,
        request: Request<QueryMediaRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<QueryMediaResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a query_media: {:?}", request);

        let reply = QueryMediaResponse { message: "Bad?".to_string(), media_list: vec![]};

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let api = API::default();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(StorageApiServer::new(api))
        .serve(addr)
        .await?;
    
    Ok(())
}