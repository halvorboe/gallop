extern crate futures;
extern crate grpcio;
extern crate protos;

use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use protos::indexer::{HelloReply, HelloRequest};
use protos::indexer_grpc::{self, Greeter};

struct IndexerService;

impl Greeter for IndexerService {

    fn say_hello(&self, ctx: RpcContext, hello_request: HelloRequest, sink: UnarySink<HelloReply>) {
        let mut hello_reply = HelloReply::new();
        let f = sink
            .success(hello_reply.clone())
            .map(move |_| println!("Responded with HelloReply {{ {:?} }}", hello_reply)
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f);
    }
}

fn main() {
    println!("Hello, world!")
}