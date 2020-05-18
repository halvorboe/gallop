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

pub mod errors;
pub mod serve;
