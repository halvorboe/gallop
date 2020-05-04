extern crate futures;
extern crate grpcio;
extern crate protos;


use protos::indexer::{HelloReply, HelloRequest};
use protos::indexer_grpc::{self, Greeter};


fn main() {
    println!("Hello, world!")
}