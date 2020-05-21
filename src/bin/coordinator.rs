#[macro_use]
extern crate log;

use gallop::core::store::Store;
use futures::Future;
use gallop::clients::packer::{LocalPackerClient, PackerClientWrapper};
use gallop::core::index::TantivyIndex;
use grpcio::{RpcContext, UnarySink};

use gallop::protos::common::Error;
use gallop::core::grpc;
use gallop::core::store::{FileStore};
use gallop::protos::common::{Segment, SegmentId};
use gallop::protos::packer::SegmentRequest;
use gallop::protos::packer_grpc::PackerClient;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use gallop::protos::coordinator_grpc::{self, Coordinator};


#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Clone)]
struct CoordinatorService {
    inner: InnerCoordinatorService<FileStore>,
}

impl CoordinatorService {
    fn new() -> Self {
        Self {
            inner: InnerCoordinatorService::new(),
        }
    }
}

impl Coordinator for CoordinatorService {
    fn cluster(&mut self, ctx: RpcContext, req: gallop::protos::coordinator::ClusterRequest, sink: UnarySink<gallop::protos::coordinator::ClusterResponse>) {
        todo!()
    }
    fn register(&mut self, ctx: RpcContext, req: gallop::protos::coordinator::NodeRequest, sink: UnarySink<Error>) {
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

}

#[cfg(test)]
mod tests {
    use super::{InnerCoordinatorService, SegmentId};
}

fn main() {
    let service = CoordinatorService::new();
    grpc::serve::default(coordinator_grpc::create_coordinator(service), 8083);
}
