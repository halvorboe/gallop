use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use tantivy::collector::{TopDocs, Count};
use tantivy::query::{QueryParser, AllQuery};
use tantivy::schema::*;
use tantivy::{doc, Index, IndexReader, ReloadPolicy};
use tempfile::TempDir;
use uuid::Uuid;
use std::path::Path;

use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug)]
struct Event {
    id: Uuid,
    session_id: Uuid,
    timestamp: u128,
    tag: String,
}

const RANDOM_WINDOW_LOW: u32 = 1;
const RANDOM_WINDOW_HIGH: u32 = RANDOM_WINDOW_LOW + 1_000_000_000;

fn generate_random_event() -> Event {
    // Generate a random timestamp.
    let start = SystemTime::now();
    let mut rng = rand::thread_rng();
    let random_time = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos()
        - (rng.gen_range(RANDOM_WINDOW_LOW, RANDOM_WINDOW_HIGH) as u128) * 60 * 60 * 24 * 365;
    // Use the timestamp to generate a random event.
    let event = Event {
        id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
        timestamp: random_time,
        tag: String::from("view"),
    };
    // Print and return the event.
    event
}

fn tokenize_timestamp(timestamp: u128) -> String {
    let naive = NaiveDateTime::from_timestamp(
        (timestamp / 1_000_000_000) as i64,
        timestamp as u32 % 1_000_000_000,
    );
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime
        .format("YEAR#%Y# MONTH#%m# DAY#%d# HOUR#%H# MINUTE#%M#")
        .to_string()
}

fn build_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("timestamp", TEXT | STORED);
    schema_builder.add_text_field("raw_timestamp", STORED);
    schema_builder.add_text_field("id", STORED);
    schema_builder.add_text_field("session_id", STORED);
    schema_builder.add_text_field("tag", TEXT | STORED);
    schema_builder.build()
}

fn build_index(schema: Schema) -> Index {
    Index::create_in_dir(&Path::new("./data"), schema.clone()).unwrap()
}

fn load_index() -> Index {
    Index::open_in_dir(&Path::new("./data")).unwrap()
}

fn event_to_doc(schema: Schema, event: Event) -> Document {
    let timestamp = schema.get_field("timestamp").unwrap();
    let raw_timestamp = schema.get_field("raw_timestamp").unwrap();
    let id = schema.get_field("id").unwrap();
    let session_id = schema.get_field("session_id").unwrap();
    let tag = schema.get_field("tag").unwrap();
    let mut doc = Document::default();
    doc.add_text(timestamp, &tokenize_timestamp(event.timestamp));
    doc.add_text(raw_timestamp, &format!("{}", event.timestamp));
    doc.add_text(id, &format!("{}", event.id));
    doc.add_text(session_id, &format!("{}", event.session_id));
    doc.add_text(tag, &event.tag);
    doc
}

fn index_random_events(number_of_events: u32) {
    println!("[START] Indexing {} random events...", number_of_events);
    println!("[RUNNING] Setting up...");
    let schema = build_schema();
    let index = build_index(schema.clone());
    let mut index_writer = index.writer(1_000_000_000).unwrap();
    println!("[RUNNING] Indexing events...");
    for i in 0..number_of_events {
        let event = generate_random_event();
        let doc = event_to_doc(schema.clone(), event);
        index_writer.add_document(doc);
        if i % 10_000 == 0 {
            println!("[RUNNING] {} of {}", i, number_of_events);
        }
    }
    println!("[RUNNING] Commiting...");
    index_writer.commit().unwrap();
    println!("[DONE] ...");
}

fn test_index() {
    let schema = build_schema();
    let index = load_index();
    let reader: IndexReader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();
    let searcher = reader.searcher();
    println!("number_of_docs: {}", searcher.num_docs());
    let query_parser = QueryParser::for_index(&index, vec![schema.get_field("timestamp").unwrap()]);
    let query = query_parser.parse_query("+YEAR#2019# +MONTH#05# +DAY#04#").unwrap();
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();
    println!("number_of_hits: {}", top_docs.len());
    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        println!("score: {}\n{}\n-----", score, schema.to_json(&retrieved_doc));
    }
    let count = searcher.search(&query, &Count).unwrap();
    println!("number_of_hits: {}", count);
}

fn main() {
    // index_random_events(10_000_000);
    test_index();

}
