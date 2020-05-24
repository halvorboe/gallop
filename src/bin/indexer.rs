#[macro_use]
extern crate log;

use futures::Future;
use gallop::clients::packer::{PackerClientImpl, PackerClientWrapper};
use gallop::core::index::TantivyIndex;
use grpcio::{RpcContext, UnarySink};

use gallop::core::grpc;
use gallop::protos::common::Error;
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
struct IndexerService {
    inner: InnerIndexerService<PackerClientImpl, TantivyIndex>,
}

impl IndexerService {
    fn new() -> Self {
        Self {
            inner: InnerIndexerService::new(),
        }
    }
}

impl Indexer for IndexerService {
    fn query(&mut self, _ctx: RpcContext, _req: QueryRequest, _sink: UnarySink<QueryResponse>) {
        unimplemented!();
    }

    fn count(&mut self, _ctx: RpcContext, _req: CountRequest, _sink: UnarySink<CountResponse>) {
        unimplemented!()
    }

    fn bind(&mut self, ctx: RpcContext, req: BindRequest, sink: UnarySink<Error>) {
        info!("Got request to bid segment...");
        let segment_id = req.get_segment_id();
        let segment = self.inner.pull(segment_id.clone());
        info!("Indexing segment...");
        self.inner.index_wrapper.index(segment.rows.to_vec());
        // Response
        grpc::errors::ok(ctx, req, sink);
    }
    fn un_bind(&mut self, ctx: RpcContext, req: UnBindRequest, sink: UnarySink<Error>) {
        grpc::errors::ok(ctx, req, sink);
    }
}
#[derive(Clone)]
struct InnerIndexerService<P: PackerClientWrapper, I: IndexWrapper> {
    packer_client_wrapper: P,
    index_wrapper: I,
}

impl<P: PackerClientWrapper, I: IndexWrapper> InnerIndexerService<P, I> {
    fn new() -> Self {
        Self {
            packer_client_wrapper: P::from_addr("localhost:8081".to_string()),
            index_wrapper: I::new(),
        }
    }

    #[allow(dead_code)]
    fn from(packer_client_wrapper: P) -> Self {
        let index_wrapper = I::new();
        Self {
            packer_client_wrapper,
            index_wrapper,
        }
    }

    fn pull(&self, segment_id: SegmentId) -> Segment {
        let resp = self.packer_client_wrapper.segment(segment_id).unwrap();
        resp
    }
}

#[cfg(test)]
mod tests {
    use super::{InnerIndexerService, SegmentId};
}

fn main() {
    let service = IndexerService::new();
    grpc::serve::default(indexer_grpc::create_indexer(service), 7071);
}
