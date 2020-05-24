use crate::protos::{
    common::{Segment, SegmentId, Row},
    packer::{PackerInsertRequest, SegmentRequest},
    packer_grpc::PackerClient,
};
use grpcio::{ChannelBuilder, EnvBuilder};
#[cfg(test)]
use mockall::{automock, predicate::*};
use std::sync::Arc;

#[cfg_attr(test, automock)]
pub trait PackerClientWrapper {
    fn from_addr(addr: String) -> Self;
    fn segment(&self, segment_id: SegmentId) -> Option<Segment>;
    fn insert(&self, row: &Row);
}

#[derive(Clone)]
pub struct PackerClientImpl {}

impl PackerClientWrapper for PackerClientImpl {
    fn from_addr(addr: String) -> Self {
        Self {}
    }

    fn segment(&self, segment_id: SegmentId) -> Option<Segment> {
        info!("Builing package fetcher...");
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect("localhost:8081");
        let client = PackerClient::new(ch);
        let mut req = SegmentRequest::new();
        req.set_segment_id(segment_id);
        let _env = Arc::new(EnvBuilder::new().build());
        info!("Sending request...");
        Some(client.segment(&req).expect("rpc").get_segment().clone())
    }

    fn insert(&self, row: &Row) {
        info!("Builing package fetcher...");
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect("localhost:8081");
        let client = PackerClient::new(ch);
        let mut req = PackerInsertRequest::new();
        let _env = Arc::new(EnvBuilder::new().build());
        info!("Sending request...");
        client.insert(&req).expect("rpc");
    }
}

mod tests {

    use super::{Segment, SegmentId, *};

    #[test]
    fn test_segment() {
        let mut mock = MockPackerClientWrapper::new();
        mock.expect_segment().returning(|_x| Some(Segment::new()));
        mock.segment(SegmentId::new());
    }


    #[test]
    fn test_insert() {
        // let mut mock = MockPackerClientWrapper::new();
    }

}
