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
use gallop::protos::packer_grpc::{self, Packer};

use gallop::core::codec;
use gallop::core::config::Configuration;

use gallop::core::directory::os::OSDirectory;
use gallop::core::directory::Directory;
use gallop::core::grpc;

#[derive(Clone)]
struct PackerService {
    inner: InnerPackerService<OSDirectory>,
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
        // println!("Inserting row...");
        let table = req.get_table().to_string();
        let row = req.get_row();
        // println!("Table: {}", table);
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
        println!("Fetching segments...");
        let segments = self.inner.segments();
        println!("Found {} segments...", segments.len());
        let mut resp = SegmentsResponse::default();
        resp.set_segments(RepeatedField::from_vec(segments));
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn segment(&mut self, ctx: RpcContext, req: SegmentRequest, sink: UnarySink<SegmentResponse>) {
        let segment_id = req.get_segment_id().clone();
        let segment_name = codec::segment::encode_id(segment_id.clone());
        let segment = self.inner.segment(segment_name);
        let mut resp = SegmentResponse::default();
        let mut segment_for_resp = Segment::new();
        segment_for_resp.set_meta(segment_id);
        segment_for_resp.set_rows(RepeatedField::from_vec(
            segment
                .unwrap()
                .iter()
                .map(|it| codec::row::decode(it))
                .collect(),
        ));
        resp.set_segment(segment_for_resp);
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }
}

#[derive(Clone)]
struct InnerPackerService<D: Directory> {
    config: Configuration,
    directory: D,
}

impl<D: Directory> InnerPackerService<D> {
    fn default() -> Self {
        Self {
            config: Configuration::default(),
            directory: D::new(),
        }
    }

    fn calculate_segment_id(&self, segment_id: &mut SegmentId) -> String {
        // Look at existing segments...
        let existing_segments = self.directory.list();
        // Prefix for new segment...
        let segment_id_prefix = codec::segment::encode_id(segment_id.clone());
        // Try to find existing segment...
        for existing_segment_id in existing_segments {
            if existing_segment_id.starts_with(&segment_id_prefix) {
                // Check if segment is too large.
                let existing_segment_size = self.segment_size(existing_segment_id.clone()).unwrap();
                if existing_segment_size < self.config.segment_max_size {
                    return existing_segment_id;
                }
            }
        }
        segment_id.set_partition_id(format!("{}", Uuid::new_v4()));
        codec::segment::encode_id(segment_id.clone())
    }

    fn insert(&mut self, table: String, row: Row) {
        // Build the segment...
        let mut segment_id = SegmentId::new();
        segment_id.set_table(table);
        segment_id.set_resolution(self.config.segment_resolution);
        segment_id.set_timestamp(codec::row::encode_timestamp(
            row.get_timestamp(),
            self.config.segment_resolution,
        ));
        // Encode the segment.
        let encoded_segment_id = self.calculate_segment_id(&mut segment_id);
        // println!("Encoded segment: {}", encoded_segment_id);
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

    fn segment(&self, name: String) -> Option<Vec<String>> {
        self.directory.read(name)
    }

    fn segment_size(&self, name: String) -> Option<u64> {
        self.directory.read_size(name)
    }
}

fn main() {
    let service = PackerService::new();
    grpc::serve(packer_grpc::create_packer(service), 8081);
}

mod tests {

    use super::*;
    use gallop::core::directory::memory::InMemoryDirectory;

    use gallop::protos::common::Row;

    #[test]
    fn test_different_segements() {
        let mut service: InnerPackerService<InMemoryDirectory> = InnerPackerService::default();

        let row = generate_row(1000);
        service.insert(String::from("a"), row.clone());
        service.insert(String::from("b"), row.clone());
        service.insert(String::from("c"), row);
        assert!(service.segments().len() == 3);
    }

    #[test]
    fn test_same_segment() {
        let mut service: InnerPackerService<InMemoryDirectory> = InnerPackerService::default();

        let table = "a".to_string();
        service.insert(table.clone(), generate_row(1000));
        service.insert(table.clone(), generate_row(1000));
        service.insert(table, generate_row(1000));
        assert!(service.segments().len() == 1);
    }

    #[test]
    fn test_content() {
        let mut service: InnerPackerService<InMemoryDirectory> = InnerPackerService::default();

        let table = "a".to_string();
        let row = generate_row(1000);

        service.insert(table, row.clone());

        let segment_id = service.segments().iter().next().unwrap().clone();

        let content = service.segment(codec::segment::encode_id(segment_id));

        assert_eq!(codec::row::encode(&row), content.unwrap()[0]);
    }

    fn generate_row(timestamp: u64) -> Row {
        let mut row = Row::new();
        row.set_timestamp(timestamp);
        row.set_data(String::from("{\"title\":\"Hello, world!\"}"));
        row
    }
}
