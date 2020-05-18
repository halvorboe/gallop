use std::fmt::Debug;
use grpcio::UnarySink;
use grpcio::RpcContext;
use crate::protos::indexer::BindRequest;
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, ServerBuilder};

use grpcio::Service;
use crate::protos::common::Error;


pub fn ok<T: 'static + Debug + Send + Sync>( ctx: RpcContext, req: T, sink: UnarySink<Error>) {
    let mut resp = Error::default();
    resp.set_code(0);
    resp.set_message("OK!".to_string());
    let f = sink
        .success(resp)
        .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
        .map(|_| ());
    ctx.spawn(f);
}