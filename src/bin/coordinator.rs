#[macro_use]
extern crate log;

use futures::Future;
use gallop::clients::packer::{PackerClientImpl, PackerClientWrapper};
use gallop::core::index::TantivyIndex;
use gallop::core::store::Store;
use grpcio::{RpcContext, UnarySink};

use gallop::core::grpc;
use gallop::core::{
    codec::node::{decode_node, encode_node},
    store::InMemoryStore,
};
use gallop::protos::common::Error;
use gallop::protos::common::{Segment, SegmentId, Row};
use gallop::protos::packer::SegmentRequest;
use gallop::protos::packer_grpc::PackerClient;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use gallop::protos::{
    coordinator::Node,
    coordinator_grpc::{self, Coordinator},
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Clone)]
struct CoordinatorService {
    inner: InnerCoordinatorService<PackerClientImpl, InMemoryStore>,
}

impl CoordinatorService {
    fn new() -> Self {
        Self {
            inner: InnerCoordinatorService::new(),
        }
    }
}

impl Coordinator for CoordinatorService {
    fn discover(
        &mut self,
        ctx: RpcContext,
        req: gallop::protos::coordinator::DiscoverRequest,
        sink: UnarySink<gallop::protos::coordinator::DiscoverResponse>,
    ) {
        todo!()
    }
    fn register(
        &mut self,
        ctx: RpcContext,
        req: gallop::protos::coordinator::NodeRequest,
        sink: UnarySink<Error>,
    ) {
        todo!()
    }
    fn select(
        &mut self,
        ctx: RpcContext,
        req: gallop::protos::coordinator::SelectRequest,
        sink: UnarySink<gallop::protos::coordinator::SelectResponse>,
    ) {
        todo!()
    }
    fn insert(
        &mut self,
        ctx: RpcContext,
        req: gallop::protos::coordinator::InsertRequest,
        sink: UnarySink<Error>,
    ) {
        todo!()
    }
    fn update(
        &mut self,
        ctx: RpcContext,
        req: gallop::protos::coordinator::UpdateRequest,
        sink: UnarySink<Error>,
    ) {
        todo!()
    }
    fn delete(
        &mut self,
        ctx: RpcContext,
        req: gallop::protos::coordinator::DeleteRequest,
        sink: UnarySink<Error>,
    ) {
        todo!()
    }
}
#[derive(Clone)]
struct InnerCoordinatorService<P: PackerClientWrapper, S: Store> {
    packer_client_wrapper: P,
    storage_wrapper: S,
}

impl<P: PackerClientWrapper, S: Store> InnerCoordinatorService<P, S> {
    fn new() -> Self {
        Self {
            packer_client_wrapper: P::from_addr("localhost:8081".to_string()),
            storage_wrapper: S::new(),
        }
    }

    fn discover(&mut self) -> Vec<Node> {
        let mut result: Vec<Node> = Vec::new();
        for (_, value) in self.storage_wrapper.items() {
            result.push(decode_node(value));
        }
        result
    }

    fn insert(&self, row: &Row) {
        self.packer_client_wrapper.insert(row);
    }

    fn register(&mut self, node: &Node) {
        self.storage_wrapper
            .set(node.get_id().to_string(), encode_node(node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gallop::protos::common::Row;


    fn test_register() {
        let mut service: InnerCoordinatorService<PackerClientImpl, InMemoryStore> = InnerCoordinatorService::new();
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

    #[test]
    fn test_insert() {
        // let mut service: InnerCoordinatorService<PackerClientImpl, InMemoryStore> = InnerCoordinatorService::new();
        // let row = Row::new();
        // service.insert(&row);
    }
}

fn main() {
    let service = CoordinatorService::new();
    grpc::serve::default(coordinator_grpc::create_coordinator(service), 8080);
}
