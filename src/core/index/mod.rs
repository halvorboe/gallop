
use crate::protos::common::Row;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, ReloadPolicy};
use tempfile::TempDir;



pub trait IndexWrapper {
    fn new() -> Self;
}


#[derive(Clone)]
pub struct TantivyIndex {
    // index: tantivy::index::Index,
}

impl IndexWrapper for TantivyIndex {

    fn new() -> Self {
        let index_path = TempDir::new().unwrap();
        let mut schema_builder = Schema::builder();
        for field in vec!["title", "body"] {
            schema_builder.add_text_field(field, TEXT | STORED);
        };
        let schema = schema_builder.build();
        let index = Index::create_in_dir(&index_path, schema.clone()).unwrap();
        Self{
            // index
        }
    }

}