use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use gallop::protos::common::{Error, Row};
use gallop::protos::storage::{SegmentMeta};
use gallop::protos::storage::{
    ConfigureRequest, InsertRequest, SegmentRequest, SegmentResponse, SegmentsRequest, SegmentsResponse,
};
use gallop::protos::storage_grpc::{self, Storage};

use gallop::core::config::{Configuration, SegmentResolution};
use gallop::core::directory::{Directory, InMemoryDirectory};
use gallop::core::grpc;

use chrono::{DateTime, NaiveDateTime, Utc};

struct StorageService {
    inner: InnerStorageService,
}

impl StorageService {
    fn new() -> Self {
        Self {
            inner: InnerStorageService::default(),
        }
    }
}

impl Storage for StorageService {
    fn configure(&mut self, ctx: RpcContext, req: ConfigureRequest, sink: UnarySink<Error>) {}
    fn insert(&mut self, ctx: RpcContext, req: InsertRequest, sink: UnarySink<Error>) {}
    fn segment(&mut self, ctx: RpcContext, req: SegmentRequest, sink: UnarySink<SegmentResponse>) {}
    fn segments(
        &mut self,
        ctx: RpcContext,
        req: SegmentsRequest,
        sink: UnarySink<SegmentsResponse>,
    ) {
    }
}

struct InnerStorageService {
    config: Configuration,
    directory: InMemoryDirectory,
}

impl InnerStorageService {

    fn default() -> Self {
        Self {
            config: Configuration::default(),
            directory: InMemoryDirectory::new(),
        }
    }

    fn segment_for_row(&self, row: Row) -> String {
        let timestamp = row.get_timestamp();
        let naive = NaiveDateTime::from_timestamp(
            (timestamp / 1_000_000_000) as i64,
            timestamp as u32 % 1_000_000_000,
        );

        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let formatter = match self.config.segment_resolution {
            SegmentResolution::HOUR => "%Y#%m#%d#%H",
            SegmentResolution::DAY => "%Y#%m#%d"
        };
        datetime
            .format(formatter)
            .to_string()
    }

    fn insert(&mut self, table_name: String, row: Row) {
        // Figure out what segment it belogs to.
        let segment = table_name + "-" + &self.segment_for_row(row.clone());
        // Append it to that segment.
        self.directory.append(segment, row.get_timestamp().to_string() + "|" + &row.get_data().to_string())
    }

    fn segments(&self) -> Vec<String> {
        self.directory.list()
    } 

    fn segment(&self, name: String) -> Vec<Row> {
        dbg!(self.directory.read(name));
        vec![]
    }
}

fn main() {    
    // grpc::serve(storage_grpc::create_storage(StorageService::new()));
}

mod tests {

    use gallop::protos::common::{Row};
    use super::InnerStorageService;



    #[test]
    fn test_basic() {
        let mut row = Row::new();
        row.set_timestamp(10_000_000_000_000_000);
        row.set_data(String::from("{\"title\":\"Hello, world!\"}"));
        let mut service = InnerStorageService::default();
        service.insert(String::from("a"), row.clone());
        service.insert(String::from("b"), row.clone());
        assert!(service.segments().len() == 2);
    }
    
    #[test]
    fn test_file_content() {
        let mut row = Row::new();
        row.set_timestamp(10_000_000_000_000_000);
        row.set_data(String::from("{\"title\":\"Hello, world!\"}"));
        let mut service = InnerStorageService::default();
        service.insert(String::from("a"), row.clone());
        let segment = &service.segments()[0];
        let rows = dbg!(service.segment(segment.to_string()));
        assert!(false);

    }
}