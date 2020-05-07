#[macro_use]
extern crate log;

use std::{io::Read, sync::Arc};
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use protobuf::RepeatedField;

use gallop::protos::common::{Error, Row};
use gallop::protos::packer::SegmentId;
use gallop::protos::packer::{
    ConfigureRequest, InsertRequest, SegmentRequest, SegmentResponse, SegmentsRequest,
    SegmentsResponse,
};
use gallop::protos::packer_grpc::{self, Packer};

use gallop::core::codec;
use gallop::core::config::Configuration;
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
        println!("Inserting row...");
        let table = req.get_table().to_string();
        let row = req.get_row();
        println!("Table: {}", table);
        self.inner.insert(table, row.clone());
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
        unimplemented!("segments not implemented");
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
        unimplemented!("segments not implemented");
        debug!(
            "Fetching {} segment...",
            codec::segment::encode_id(req.get_segment_id().clone())
        );
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

    fn insert(&mut self, table: String, row: Row) {
        // Build the segment...
        let mut segment_id = SegmentId::new();
        segment_id.set_table(table.clone());
        segment_id.set_resolution(self.config.segment_resolution);
        segment_id.set_timestamp(codec::row::encode_timestamp(
            row.get_timestamp(),
            self.config.segment_resolution,
        ));
        // Encode the segment.
        let encoded_segment_id = codec::segment::encode_id(dbg!(segment_id));
        println!("Encoded segment: {}", encoded_segment_id);
        // Encode the row...
        let encoded_row = codec::row::encode(&row);
        // Append to the segment.
        self.directory.append(encoded_segment_id, encoded_row)
    }

    fn segments(&self) -> Vec<SegmentId> {
        let raw_segments = self.directory.list();
        raw_segments
            .iter()
            .map(|it| codec::segment::decode_id(it.clone()))
            .collect()
    }

    fn segment(&self, name: String) -> Option<Vec<Row>> {
        println!("Reading {} from {}", name, self.directory.list()[0]);
        let lines = self.directory.read(name)?;
        Some(lines.iter().map(|it| codec::row::decode(it)).collect())
    }
}

fn main() {
    let service = PackerService::new();
    grpc::serve(packer_grpc::create_packer(service), 8081);
}

mod tests {

    use super::InnerPackerService;
    use gallop::core::codec;
    use gallop::protos::common::Row;

    #[test]
    fn test_different_segements() {
        let mut service = InnerPackerService::default();

        let mut row = generate_row(1000);
        service.insert(String::from("a"), row.clone());
        service.insert(String::from("b"), row.clone());
        service.insert(String::from("c"), row.clone());
        assert!(service.segments().len() == 3);
    }

    #[test]
    fn test_same_segment() {
        let mut service = InnerPackerService::default();

        let table = "a".to_string();
        service.insert(table.clone(), generate_row(1000));
        service.insert(table.clone(), generate_row(1000));
        service.insert(table.clone(), generate_row(1000));
        assert!(service.segments().len() == 1);
    }

    #[test]
    fn test_content() {
        let mut service = InnerPackerService::default();

        let table = "a".to_string();
        let row = generate_row(1000);

        service.insert(table.clone(), row.clone());

        let segment_id = service.segments().iter().next().unwrap().clone();

        let content = service.segment(codec::segment::encode_id(segment_id));

        assert_eq!(row, content.unwrap()[0]);
    }

    fn generate_row(timestamp: u64) -> Row {
        let mut row = Row::new();
        row.set_timestamp(timestamp);
        row.set_data(String::from("{\"title\":\"Hello, world!\"}"));
        row
    }
}
