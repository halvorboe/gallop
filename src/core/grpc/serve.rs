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



pub fn default(service: Service, port: u16) {
    env_logger::init();
    // New event loop.
    let env = Arc::new(Environment::new(1));
    // Build the server.
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("localhost", port)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("Listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
