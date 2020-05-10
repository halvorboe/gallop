#[macro_use]
extern crate log;

use grpcio::{ChannelBuilder, EnvBuilder, Environment, RpcContext, ServerBuilder, UnarySink};

use gallop::protos::common::Error;

use gallop::protos::indexer::{BindRequest, UnBindRequest};
use gallop::protos::indexer_grpc::{self, Indexer};

use gallop::callers::packer::{ConnectedPackerCaller, PackerCaller};

use gallop::core::directory::Directory;
use gallop::core::grpc;

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
    fn bind(&mut self, _ctx: RpcContext, req: BindRequest, _sink: UnarySink<Error>) {
        let _segment_id = req.get_segment_id();
        // let segment = self.inner.pull(segment_id.clone());
    }
    fn un_bind(&mut self, _ctx: RpcContext, _req: UnBindRequest, _sink: UnarySink<Error>) {
        todo!();
    }
}
#[derive(Clone)]
struct InnerIndexerService<C: PackerCaller> {
    packer_caller: C,
}

impl<C: PackerCaller> InnerIndexerService<C> {
    fn new() -> Self {
        // let env = Arc::new(EnvBuilder::new().build());
        // let channel = ChannelBuilder::new(env).connect("localhost:8081");
        // let client = PackerClient::new(channel);
        // Self {
        //     packer_client: client,
        // }
        Self {
            packer_caller: C::new(),
        }
    }

    // fn pull(&self, segment_id: SegmentId) -> Segment {
    //     let mut req = SegmentRequest::new();
    //     req.set_segment_id(segment_id);
    //     let resp = self.packer_client.segment(&req).unwrap();
    //     resp.get_segment().clone()
    // }
}

fn main() {
    let service = IndexerService::new();
    grpc::serve(indexer_grpc::create_indexer(service), 8082);
}
