use crate::protos::indexer::BindRequest;
use grpcio::RpcContext;
use grpcio::UnarySink;
use std::fmt::Debug;
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, ServerBuilder};

use crate::protos::common::Error;
use grpcio::Service;

pub fn ok<T: 'static + Debug + Send + Sync>(ctx: RpcContext, req: T, sink: UnarySink<Error>) {
    let mut resp = Error::default();
    resp.set_code(0);
    resp.set_message("OK!".to_string());
    let f = sink
        .success(resp)
        .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
        .map(|_| ());
    ctx.spawn(f);
}
