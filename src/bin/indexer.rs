#[macro_use]
extern crate log;

use uuid::Uuid;

use std::io::Read;

use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use protobuf::RepeatedField;

use gallop::protos::common::{Error, Row};
use gallop::protos::common::{Segment, SegmentId};
use gallop::protos::packer::{
    InsertRequest, SegmentRequest, SegmentResponse, SegmentsRequest, SegmentsResponse,
};
use gallop::protos::packer_grpc::{self, Packer, PackerClient};
use std::sync::Arc;

use gallop::core::codec;
use gallop::core::config::Configuration;

use gallop::core::directory::os::OSDirectory;
use gallop::core::directory::Directory;
use gallop::core::grpc;

use grpcio::{ChannelBuilder, EnvBuilder};

use gallop::protos::indexer::{BindRequest, UnBindRequest};
use gallop::protos::indexer_grpc::{self, Indexer};

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

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
    fn bind(&mut self, ctx: RpcContext, req: BindRequest, sink: UnarySink<Error>) {
        let segment_id = req.get_segment_id();
        let segment = self.inner.pull(segment_id.clone());
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
            packer_caller: C::from("localhost:8080".to_string()),
        }
    }

    fn from(packer_caller: C) -> Self {
        Self {
            packer_caller: packer_caller,
        }
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
    fn foo(&self, x: u32) -> u32;
    fn segment(&self, segment_id: SegmentId) -> Option<Segment>;
}

#[derive(Clone)]
pub struct ConnectedPackerCaller {}

impl PackerCaller for ConnectedPackerCaller {
    fn from(host: String) -> Self {
        return Self {};
    }
    fn foo(&self, x: u32) -> u32 {
        x + 1
    }

    fn segment(&self, segment_id: SegmentId) -> Option<Segment> {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect("localhost:50051");
        let client = PackerClient::new(ch);
        let mut req = SegmentRequest::new();
        req.set_segment_id(segment_id);
        Some(client.segment(&req).expect("rpc").get_segment().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::{InnerIndexerService, MockPackerCaller, SegmentId};

    use gallop::protos::common::Segment;
    #[cfg(test)]
    use mockall::{automock, mock, predicate};

    #[test]
    fn test_basic_mock() {
        let mut mock = MockPackerCaller::new();
        mock.expect_segment().returning(|x| Some(Segment::new()));
        mock.expect_foo()
            .with(predicate::eq(4))
            .times(1)
            .returning(|x| x + 1);
        let service = InnerIndexerService::from(mock);
        service.pull(SegmentId::new());
    }
}
