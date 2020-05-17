
use crate::core::codec::row::encode_timestamp;
use crate::protos::common::{SegmentResolution, Row};
use tantivy::collector::Count;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, ReloadPolicy};
use tempfile::TempDir;



pub trait IndexWrapper {
    fn new() -> Self;
    fn index(&mut self, rows: Vec<Row>);
    fn count(&mut self, query: String) -> usize;
}


#[derive(Clone)]
pub struct TantivyIndex {
    index: tantivy::Index,
    schema: tantivy::schema::Schema,
}

impl IndexWrapper for TantivyIndex {

    fn new() -> Self {
        let index_path = TempDir::new().unwrap();
        let mut schema_builder = Schema::builder();
        for field in vec!["timestamp"] {
            schema_builder.add_text_field(field, TEXT | STORED);
        };
        let schema = schema_builder.build();
        let index = Index::create_in_dir(&index_path, schema.clone()).unwrap();
        Self{
            index,
            schema
        }
    }
    fn index(&mut self, rows: Vec<Row>) {
        let mut writer = self.index.writer(50_000_000).unwrap();
        let timestamp = self.schema.get_field("timestamp").unwrap();
        for row in rows {
            let mut doc = Document::default();
            doc.add_text(timestamp, &encode_timestamp(row.get_timestamp(), SegmentResolution::DAY));
            writer.add_document(doc);
        }
        writer.commit().unwrap();

    }
    fn count(&mut self, query: String) -> usize {
        let reader = self.index.reader().unwrap();
        let searcher = reader.searcher();
        let timestamp = self.schema.get_field("timestamp").unwrap();
        let query_parser = QueryParser::for_index(&self.index, vec![timestamp]);
        searcher.search(&query_parser.parse_query(&query).unwrap(), &Count).unwrap()
    }


}