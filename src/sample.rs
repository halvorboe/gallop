//! This example demonstrates asynchronous subscriptions with warp and tokio 0.2

// use std::{pin::Pin, sync::Arc, time::Duration};

// use futures::{Future, FutureExt as _, Stream};
// use juniper::{DefaultScalarValue, EmptyMutation, FieldError, RootNode};
// use juniper_subscriptions::Coordinator;
// use juniper_warp::{playground_filter, subscriptions::graphql_subscriptions};
// use warp::{http::Response, Filter};
// Tantivy
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, ReloadPolicy};
use tempfile::TempDir;

// #[derive(Clone)]
// struct Context {}

// impl juniper::Context for Context {}

// #[derive(Clone, Copy, juniper::GraphQLEnum)]
// enum UserKind {
//     Admin,
//     User,
//     Guest,
// }

// struct User {
//     id: i32,
//     kind: UserKind,
//     name: String,
// }

// // Field resolvers implementation
// #[juniper::graphql_object(Context = Context)]
// impl User {
//     fn id(&self) -> i32 {
//         self.id
//     }

//     fn kind(&self) -> UserKind {
//         self.kind
//     }

//     fn name(&self) -> &str {
//         &self.name
//     }

//     async fn friends(&self) -> Vec<User> {
//         if self.id == 1 {
//             return vec![
//                 User {
//                     id: 11,
//                     kind: UserKind::User,
//                     name: "user11".into(),
//                 },
//                 User {
//                     id: 12,
//                     kind: UserKind::Admin,
//                     name: "user12".into(),
//                 },
//                 User {
//                     id: 13,
//                     kind: UserKind::Guest,
//                     name: "user13".into(),
//                 },
//             ];
//         } else if self.id == 2 {
//             return vec![User {
//                 id: 21,
//                 kind: UserKind::User,
//                 name: "user21".into(),
//             }];
//         } else if self.id == 3 {
//             return vec![
//                 User {
//                     id: 31,
//                     kind: UserKind::User,
//                     name: "user31".into(),
//                 },
//                 User {
//                     id: 32,
//                     kind: UserKind::Guest,
//                     name: "user32".into(),
//                 },
//             ];
//         } else {
//             return vec![];
//         }
//     }
// }

// struct Query;

// #[juniper::graphql_object(Context = Context)]
// impl Query {
//     async fn users(id: i32) -> Vec<User> {
//         vec![User {
//             id,
//             kind: UserKind::Admin,
//             name: "User Name".into(),
//         }]
//     }
// }

// type UsersStream = Pin<Box<dyn Stream<Item = Result<User, FieldError>> + Send>>;

// struct Subscription;

// #[juniper::graphql_subscription(Context = Context)]
// impl Subscription {
//     async fn users() -> UsersStream {
//         let mut counter = 0;
//         let stream = tokio::time::interval(Duration::from_secs(5)).map(move |_| {
//             counter += 1;
//             if counter == 2 {
//                 Err(FieldError::new(
//                     "some field error from handler",
//                     Value::Scalar(DefaultScalarValue::String(
//                         "some additional string".to_string(),
//                     )),
//                 ))
//             } else {
//                 Ok(User {
//                     id: counter,
//                     kind: UserKind::Admin,
//                     name: "stream user".to_string(),
//                 })
//             }
//         });

//         Box::pin(stream)
//     }
// }

// type Schema = RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

// fn schema() -> Schema {
//     Schema::new(Query, EmptyMutation::new(), Subscription)
// }

// #[tokio::main]
// async fn main() {
//     ::std::env::set_var("RUST_LOG", "warp_subscriptions");
//     env_logger::init();

//     let log = warp::log("warp_server");

//     let homepage = warp::path::end().map(|| {
//         Response::builder()
//             .header("content-type", "text/html")
//             .body("<html><h1>juniper_subscriptions demo</h1><div>visit <a href=\"/playground\">graphql playground</a></html>".to_string())
//     });

//     let qm_schema = schema();
//     let qm_state = warp::any().map(move || Context {});
//     let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

//     let sub_state = warp::any().map(move || Context {});
//     let coordinator = Arc::new(juniper_subscriptions::Coordinator::new(schema()));

//     log::info!("Listening on 127.0.0.1:8080");

//     let routes = (warp::path("subscriptions")
//         .and(warp::ws())
//         .and(sub_state.clone())
//         .and(warp::any().map(move || Arc::clone(&coordinator)))
//         .map(
//             |ws: warp::ws::Ws,
//              ctx: Context,
//              coordinator: Arc<Coordinator<'static, _, _, _, _, _>>| {
//                 ws.on_upgrade(|websocket| -> Pin<Box<dyn Future<Output = ()> + Send>> {
//                     graphql_subscriptions(websocket, coordinator, ctx)
//                         .map(|r| {
//                             if let Err(e) = r {
//                                 println!("Websocket error: {}", e);
//                             }
//                         })
//                         .boxed()
//                 })
//             },
//         ))
//     .map(|reply| {
//         // TODO#584: remove this workaround
//         warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
//     })
//     .or(warp::post()
//         .and(warp::path("graphql"))
//         .and(qm_graphql_filter))
//     .or(warp::get()
//         .and(warp::path("playground"))
//         .and(playground_filter("/graphql", Some("/subscriptions"))))
//     .or(homepage)
//     .with(log);

//     warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
// }


fn main() -> tantivy::Result<()> {
      // Let's create a temporary directory for the
    // sake of this example
    let index_path = TempDir::new()?;

    // # Defining the schema
    //
    // The Tantivy index requires a very strict schema.
    // The schema declares which fields are in the index,
    // and for each field, its type and "the way it should
    // be indexed".

    // First we need to define a schema ...
    let mut schema_builder = Schema::builder();

    // Our first field is title.
    // We want full-text search for it, and we also want
    // to be able to retrieve the document after the search.
    //
    // `TEXT | STORED` is some syntactic sugar to describe
    // that.
    //
    // `TEXT` means the field should be tokenized and indexed,
    // along with its term frequency and term positions.
    //
    // `STORED` means that the field will also be saved
    // in a compressed, row-oriented key-value store.
    // This store is useful for reconstructing the
    // documents that were selected during the search phase.
    schema_builder.add_text_field("title", TEXT | STORED);

    // Our second field is body.
    // We want full-text search for it, but we do not
    // need to be able to be able to retrieve it
    // for our application.
    //
    // We can make our index lighter by omitting the `STORED` flag.
    schema_builder.add_text_field("body", TEXT);

    let schema = schema_builder.build();

    // # Indexing documents
    //
    // Let's create a brand new index.
    //
    // This will actually just save a meta.json
    // with our schema in the directory.
    let index = Index::create_in_dir(&index_path, schema.clone())?;

    // To insert a document we will need an index writer.
    // There must be only one writer at a time.
    // This single `IndexWriter` is already
    // multithreaded.
    //
    // Here we give tantivy a budget of `50MB`.
    // Using a bigger heap for the indexer may increase
    // throughput, but 50 MB is already plenty.
    let mut index_writer = index.writer(50_000_000)?;

    // Let's index our documents!
    // We first need a handle on the title and the body field.

    // ### Adding documents
    //
    // We can create a document manually, by setting the fields
    // one by one in a Document object.
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let mut old_man_doc = Document::default();
    old_man_doc.add_text(title, "The Old Man and the Sea");
    old_man_doc.add_text(
        body,
        "He was an old man who fished alone in a skiff in the Gulf Stream and \
         he had gone eighty-four days now without taking a fish.",
    );

    // ... and add it to the `IndexWriter`.
    index_writer.add_document(old_man_doc);

    // For convenience, tantivy also comes with a macro to
    // reduce the boilerplate above.
    index_writer.add_document(doc!(
    title => "Of Mice and Men",
    body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
            bank and runs deep and green. The water is warm too, for it has slipped twinkling \
            over the yellow sands in the sunlight before reaching the narrow pool. On one \
            side of the river the golden foothill slopes curve up to the strong and rocky \
            Gabilan Mountains, but on the valley side the water is lined with trees—willows \
            fresh and green with every spring, carrying in their lower leaf junctures the \
            debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
            limbs and branches that arch over the pool"
    ));

    index_writer.add_document(doc!(
    title => "Of Mice and Men",
    body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
            bank and runs deep and green. The water is warm too, for it has slipped twinkling \
            over the yellow sands in the sunlight before reaching the narrow pool. On one \
            side of the river the golden foothill slopes curve up to the strong and rocky \
            Gabilan Mountains, but on the valley side the water is lined with trees—willows \
            fresh and green with every spring, carrying in their lower leaf junctures the \
            debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
            limbs and branches that arch over the pool"
    ));

    // Multivalued field just need to be repeated.
    index_writer.add_document(doc!(
    title => "Frankenstein",
    title => "The Modern Prometheus",
    body => "You will rejoice to hear that no disaster has accompanied the commencement of an \
             enterprise which you have regarded with such evil forebodings.  I arrived here \
             yesterday, and my first task is to assure my dear sister of my welfare and \
             increasing confidence in the success of my undertaking."
    ));

    // This is an example, so we will only index 3 documents
    // here. You can check out tantivy's tutorial to index
    // the English wikipedia. Tantivy's indexing is rather fast.
    // Indexing 5 million articles of the English wikipedia takes
    // around 3 minutes on my computer!

    // ### Committing
    //
    // At this point our documents are not searchable.
    //
    //
    // We need to call `.commit()` explicitly to force the
    // `index_writer` to finish processing the documents in the queue,
    // flush the current index to the disk, and advertise
    // the existence of new documents.
    //
    // This call is blocking.
    index_writer.commit()?;

    // If `.commit()` returns correctly, then all of the
    // documents that have been added are guaranteed to be
    // persistently indexed.
    //
    // In the scenario of a crash or a power failure,
    // tantivy behaves as if it has rolled back to its last
    // commit.

    // # Searching
    //
    // ### Searcher
    //
    // A reader is required first in order to search an index.
    // It acts as a `Searcher` pool that reloads itself,
    // depending on a `ReloadPolicy`.
    //
    // For a search server you will typically create one reader for the entire lifetime of your
    // program, and acquire a new searcher for every single request.
    //
    // In the code below, we rely on the 'ON_COMMIT' policy: the reader
    // will reload the index automatically after each commit.
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    // We now need to acquire a searcher.
    //
    // A searcher points to a snapshotted, immutable version of the index.
    //
    // Some search experience might require more than
    // one query. Using the same searcher ensures that all of these queries will run on the
    // same version of the index.
    //
    // Acquiring a `searcher` is very cheap.
    //
    // You should acquire a searcher every time you start processing a request and
    // and release it right after your query is finished.
    let searcher = reader.searcher();

    // ### Query

    // The query parser can interpret human queries.
    // Here, if the user does not specify which
    // field they want to search, tantivy will search
    // in both title and body.
    let query_parser = QueryParser::for_index(&index, vec![title, body]);

    // `QueryParser` may fail if the query is not in the right
    // format. For user facing applications, this can be a problem.
    // A ticket has been opened regarding this problem.
    let query = query_parser.parse_query("sea whale")?;

    // A query defines a set of documents, as
    // well as the way they should be scored.
    //
    // A query created by the query parser is scored according
    // to a metric called Tf-Idf, and will consider
    // any document matching at least one of our terms.

    // ### Collectors
    //
    // We are not interested in all of the documents but
    // only in the top 10. Keeping track of our top 10 best documents
    // is the role of the `TopDocs` collector.

    // We can now perform our query.
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    // The actual documents still need to be
    // retrieved from Tantivy's store.
    //
    // Since the body field was not configured as stored,
    // the document returned will only contain
    // a title.
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", schema.to_json(&retrieved_doc));
    }

    Ok(())
}