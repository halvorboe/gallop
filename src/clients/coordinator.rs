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
pub trait CoordintatorClientWrapper {
    fn discover() -> Vec<Node>;
    fn register(node: &Node);
}

#[derive(Clone)]
pub struct CoordinatorClientImpl {

}

impl CoordintatorClientWrapper for CoordinatorClientImpl {
}

mod tests {

    #[test]
    fn test_insert() {
        

    }

}
