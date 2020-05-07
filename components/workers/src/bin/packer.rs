#[macro_use] extern crate log;


use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use protobuf::{RepeatedField};

use gallop::protos::common::{Error, Row};
use gallop::protos::packer::SegmentId;
use gallop::protos::packer::{
    ConfigureRequest, InsertRequest, SegmentRequest, SegmentResponse, SegmentsRequest,
    SegmentsResponse,
};
use gallop::protos::packer_grpc::{self, Packer};

use gallop::core::codec;
use gallop::core::config::{Configuration};
use gallop::core::directory::{Directory, InMemoryDirectory};
use gallop::core::grpc;

use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Clone)]
struct PackerService {
    inner: InnerPackerService,
}

impl PackerService {
    fn new() -> Self {
        Self {
            inner: InnerPackerService::default(),
        }
    }
}

impl Packer for PackerService {
    
    fn insert(&mut self, ctx: RpcContext, req: InsertRequest, sink: UnarySink<Error>) {
        debug!("Inserting row...");
        let table_name = req.get_table_name().to_string();
        let row = req.get_row();
        self.inner.insert(table_name, row.clone());
        let mut resp = Error::default();
        resp.set_code(0);
        resp.set_message("OK!".to_string());
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn segments(
        &mut self,
        ctx: RpcContext,
        req: SegmentsRequest,
        sink: UnarySink<SegmentsResponse>,
    ) {
        debug!("Fetching segments...");
        let segments = self.inner.segments();
        debug!("Found {} segments...", segments.len());
        let mut resp = SegmentsResponse::default();
        resp.set_segments(RepeatedField::from_vec(segments));
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f) 
    }

    fn segment(&mut self, ctx: RpcContext, req: SegmentRequest, sink: UnarySink<SegmentResponse>) {
        debug!("Fetching {} segment...", codec::segment::encode_id( req.get_segment_id().clone()));
        let segment_id = req.get_segment_id();
        let segment = self.inner.segment("124".to_string());
        let mut resp = SegmentResponse::default();


        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f) 

    }

}

#[derive(Clone)]
struct InnerPackerService {
    config: Configuration,
    directory: InMemoryDirectory,
}

impl InnerPackerService {
    fn default() -> Self {
        Self {
            config: Configuration::default(),
            directory: InMemoryDirectory::new(),
        }
    }

    fn insert(&mut self, table_name: String, row: Row) {
        let mut segment_id = SegmentId::new();
        segment_id.set_table(table_name.clone());
        segment_id.set_resolution(self.config.segment_resolution);
        segment_id.set_timestamp(codec::row::encode_timestamp(row.get_timestamp() as u128, self.config.segment_resolution));
        self.directory.append(table_name, codec::row::encode(&row))
    }

    fn segments(&self) -> Vec<SegmentId> {
        self.directory.list().iter().map(|it| codec::segment::decode_id(it.clone())).collect()
    }

    fn segment(&self, name: String) -> Vec<Row> {
        self.directory
            .read(name)
            .iter()
            .map(|it| codec::row::decode(it))
            .collect()
    }
}

fn main() {
    let service = PackerService::new();
    grpc::serve(packer_grpc::create_packer(service), 8081);
}

mod tests {

    use super::InnerPackerService;
    use gallop::protos::common::Row;

    #[test]
    fn test_basic() {
        let mut row = Row::new();
        row.set_timestamp(10_000_000_000_000_000);
        row.set_data(String::from("{\"title\":\"Hello, world!\"}"));
        let mut service = InnerPackerService::default();
        service.insert(String::from("a"), row.clone());
        service.insert(String::from("b"), row.clone());
        assert!(service.segments().len() == 2);
    }

    #[test]
    fn test_file_content() {
        let mut row = Row::new();
        row.set_timestamp(10_000_000_000_000_000);
        row.set_data(String::from("{\"title\":\"Hello, world!\"}"));
        let mut service = InnerPackerService::default();
        service.insert(String::from("a"), row.clone());
        let segment = &service.segments()[0];
        let rows = service.segment("123".to_string());
        assert_eq!(rows[0], row);
    }
}
