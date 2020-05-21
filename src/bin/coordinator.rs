#[macro_use]
extern crate log;

use futures::Future;
use gallop::clients::packer::{LocalPackerClient, PackerClientWrapper};
use gallop::core::index::TantivyIndex;
use grpcio::{RpcContext, UnarySink};

use gallop::protos::common::Error;
use gallop::core::grpc;
use gallop::protos::common::{Segment, SegmentId};
use gallop::protos::packer::SegmentRequest;
use gallop::protos::packer_grpc::PackerClient;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use gallop::protos::indexer::{
    BindRequest, CountRequest, CountResponse, QueryRequest, QueryResponse, UnBindRequest,
};
use gallop::{
    core::index::IndexWrapper,
    protos::indexer_grpc::{self, Indexer},
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Clone)]
struct CoordinatorService {
    inner: InnerCoordinatorService<FileStore>,
}

impl CoordinatorService {
    fn new() -> Self {
        Self {
            inner: InnerIndexerService::new(),
        }
    }
}

impl Coordinator for CoordinatorService {
}
#[derive(Clone)]
struct InnerCoordinatorService<S: Store> {
}

impl<S: FileStore> InnerIndexerService<S> {
    fn new() -> Self {
        Self {
            FileStore::new()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::{InnerCoordinatorService, SegmentId};
}

fn main() {
    let service = CoordinatorService::new();
    grpc::serve::default(coordinator_grpc::create_indexer(service), 8083);
}
