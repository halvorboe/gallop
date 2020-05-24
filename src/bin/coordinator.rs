#[macro_use]
extern crate log;

use gallop::core::store::Store;
use futures::Future;
use gallop::clients::packer::{LocalPackerClient, PackerClientWrapper};
use gallop::core::index::TantivyIndex;
use grpcio::{RpcContext, UnarySink};

use gallop::protos::common::Error;
use gallop::core::grpc;
use gallop::core::{codec::node::{decode_node, encode_node}, store::{InMemoryStore}};
use gallop::protos::common::{Segment, SegmentId};
use gallop::protos::packer::SegmentRequest;
use gallop::protos::packer_grpc::PackerClient;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use gallop::protos::{coordinator::Node, coordinator_grpc::{self, Coordinator}};


#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Clone)]
struct CoordinatorService {
    inner: InnerCoordinatorService<InMemoryStore>,
}

impl CoordinatorService {
    fn new() -> Self {
        Self {
            inner: InnerCoordinatorService::new(),
        }
    }
}

impl Coordinator for CoordinatorService {
    fn discover(&mut self, ctx: RpcContext, req: gallop::protos::coordinator::DiscoverRequest, sink: UnarySink<gallop::protos::coordinator::DiscoverResponse>) {
        todo!()
    }
    fn register(&mut self, ctx: RpcContext, req: gallop::protos::coordinator::NodeRequest, sink: UnarySink<Error>) {
        todo!()
    }
    fn select(&mut self, ctx: RpcContext, req: gallop::protos::coordinator::SelectRequest, sink: UnarySink<gallop::protos::coordinator::SelectResponse>) {
        todo!()
    }
    fn insert(&mut self, ctx: RpcContext, req: gallop::protos::coordinator::InsertRequest, sink: UnarySink<Error>) {
        todo!()
    }
    fn update(&mut self, ctx: RpcContext, req: gallop::protos::coordinator::UpdateRequest, sink: UnarySink<Error>) {
        todo!()
    }
    fn delete(&mut self, ctx: RpcContext, req: gallop::protos::coordinator::DeleteRequest, sink: UnarySink<Error>) {
        todo!()
    }
    
}
#[derive(Clone)]
struct InnerCoordinatorService<S: Store> {
    storage: S,
}

impl<S: Store> InnerCoordinatorService<S> {
    fn new() -> Self {
        Self {
            storage: Store::new(),
        }
    }

    fn discover(&mut self) -> Vec<Node> {
        let mut result: Vec<Node> = Vec::new();
        for (_, value) in self.storage.items() {
            result.push(decode_node(value));
        }
        result
    }

    fn register(&mut self, node: &Node) {
        self.storage.set(node.get_id().to_string(), encode_node(node))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut service: InnerCoordinatorService<InMemoryStore> = InnerCoordinatorService::new();
        let mut a = Node::new();
        a.set_id("1".to_string());
        let mut b = Node::new();
        b.set_id("2".to_string());
        let mut c = Node::new();
        c.set_id("3".to_string());
        service.register(&a);
        service.register(&b);
        service.register(&c);
        assert_eq!(service.discover().len(), 3);
    }

}

fn main() {
    let service = CoordinatorService::new();
    grpc::serve::default(coordinator_grpc::create_coordinator(service), 7070);
}
