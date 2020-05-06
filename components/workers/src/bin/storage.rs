use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use gallop::protos::common::{Row, Error};
use gallop::protos::storage::{SegmentRequest, SegmentResponse, SegmentsRequest, SegmentsResponse};
use gallop::protos::storage_grpc::{self, Storage};

use gallop::core::grpc;


#[derive(Debug, Clone)]
struct StorageService {
    inner: InnerStorageService,
}

impl StorageService {

    fn new() -> Self {
        Self {
            inner: InnerStorageService::new()
        }
    }
}

impl Storage for StorageService {

    fn insert(&mut self, ctx: RpcContext, req: Row, sink: UnarySink<Error>) {

    }
    fn segment(&mut self, ctx: RpcContext, req: SegmentRequest, sink: UnarySink<SegmentResponse>) {

    }
    fn segments(&mut self, ctx: RpcContext, req: SegmentsRequest, sink: UnarySink<SegmentsResponse>) {

    }
}


#[derive(Debug, Clone)]struct InnerStorageService {

}

impl InnerStorageService {
    
    fn new() -> Self {
        Self{}
    }

    fn insert(row: Row) {
        // Figure out what segment it belogs to.
        // Append it to that segment.
    }

}



fn main() {
    grpc::serve(storage_grpc::create_storage(StorageService::new()));
}
