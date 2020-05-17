#[macro_use]
extern crate log;

use futures::Future;
use grpcio::{RpcContext, UnarySink};

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
    core::grpc,
    protos::indexer_grpc::{self, Indexer},
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Clone)]
struct IndexerService {
    inner: InnerIndexerService<ConnectedPackerCaller>,
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
        unimplemented!();
    }

    fn bind(&mut self, ctx: RpcContext, req: BindRequest, sink: UnarySink<Error>) {
        info!("Got request to bid segment...");
        let segment_id = req.get_segment_id();
        let _segment = self.inner.pull(segment_id.clone());
        let mut resp = Error::default();
        resp.set_code(0);
        resp.set_message("OK!".to_string());
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }
    fn un_bind(&mut self, ctx: RpcContext, req: UnBindRequest, sink: UnarySink<Error>) {
        let mut resp = Error::default();
        resp.set_code(0);
        resp.set_message("OK!".to_string());
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }
}
#[derive(Clone)]
struct InnerIndexerService<C: PackerCaller> {
    packer_caller: C,
}

impl<C: PackerCaller> InnerIndexerService<C> {
    fn new() -> Self {
        Self {
            packer_caller: C::from("localhost:8081".to_string()),
        }
    }

    #[allow(dead_code)]
    fn from(packer_caller: C) -> Self {
        Self { packer_caller }
    }

    fn pull(&self, segment_id: SegmentId) -> Segment {
        let resp = self.packer_caller.segment(segment_id).unwrap();
        resp
    }
}

fn main() {
    let service = IndexerService::new();
    grpc::serve(indexer_grpc::create_indexer(service), 8082);
}

#[cfg_attr(test, automock)]
pub trait PackerCaller {
    fn from(host: String) -> Self;
    fn segment(&self, segment_id: SegmentId) -> Option<Segment>;
}

#[derive(Clone)]
pub struct ConnectedPackerCaller {}

impl PackerCaller for ConnectedPackerCaller {
    fn from(_host: String) -> Self {
        Self {}
    }

    fn segment(&self, segment_id: SegmentId) -> Option<Segment> {
        info!("Builing package fetcher...");
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect("localhost:8081");
        let client = PackerClient::new(ch);
        let mut req = SegmentRequest::new();
        req.set_segment_id(segment_id);
        info!("Sending request...");
        let _env = Arc::new(EnvBuilder::new().build());
        Some(client.segment(&req).expect("rpc").get_segment().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::{InnerIndexerService, MockPackerCaller, SegmentId};

    use gallop::protos::common::Segment;
    #[cfg(test)]
    #[test]
    fn test_basic_mock() {
        let mut mock = MockPackerCaller::new();
        mock.expect_segment().returning(|_x| Some(Segment::new()));
        let service = InnerIndexerService::from(mock);
        service.pull(SegmentId::new());
    }
}
