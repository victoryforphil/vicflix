use tonic::{transport::Server, Request, Response, Status};


pub mod vix_flix_api {
    tonic::include_proto!("vix_flix_api"); // The string specified here must match the proto package name
}