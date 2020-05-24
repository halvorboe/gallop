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

pub mod errors;
pub mod serve;
