// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_INDEXER_COUNT: ::grpcio::Method<super::indexer::CountRequest, super::indexer::CountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Indexer/Count",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEXER_QUERY: ::grpcio::Method<super::indexer::QueryRequest, super::indexer::QueryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Indexer/Query",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEXER_BIND: ::grpcio::Method<super::indexer::BindRequest, super::common::Error> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Indexer/Bind",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEXER_UN_BIND: ::grpcio::Method<super::indexer::UnBindRequest, super::common::Error> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Indexer/UnBind",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct IndexerClient {
    client: ::grpcio::Client,
}

impl IndexerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        IndexerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn count_opt(&self, req: &super::indexer::CountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexer::CountResponse> {
        self.client.unary_call(&METHOD_INDEXER_COUNT, req, opt)
    }

    pub fn count(&self, req: &super::indexer::CountRequest) -> ::grpcio::Result<super::indexer::CountResponse> {
        self.count_opt(req, ::grpcio::CallOption::default())
    }

    pub fn count_async_opt(&self, req: &super::indexer::CountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexer::CountResponse>> {
        self.client.unary_call_async(&METHOD_INDEXER_COUNT, req, opt)
    }

    pub fn count_async(&self, req: &super::indexer::CountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexer::CountResponse>> {
        self.count_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_opt(&self, req: &super::indexer::QueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexer::QueryResponse> {
        self.client.unary_call(&METHOD_INDEXER_QUERY, req, opt)
    }

    pub fn query(&self, req: &super::indexer::QueryRequest) -> ::grpcio::Result<super::indexer::QueryResponse> {
        self.query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_async_opt(&self, req: &super::indexer::QueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexer::QueryResponse>> {
        self.client.unary_call_async(&METHOD_INDEXER_QUERY, req, opt)
    }

    pub fn query_async(&self, req: &super::indexer::QueryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexer::QueryResponse>> {
        self.query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bind_opt(&self, req: &super::indexer::BindRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::Error> {
        self.client.unary_call(&METHOD_INDEXER_BIND, req, opt)
    }

    pub fn bind(&self, req: &super::indexer::BindRequest) -> ::grpcio::Result<super::common::Error> {
        self.bind_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bind_async_opt(&self, req: &super::indexer::BindRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.client.unary_call_async(&METHOD_INDEXER_BIND, req, opt)
    }

    pub fn bind_async(&self, req: &super::indexer::BindRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.bind_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn un_bind_opt(&self, req: &super::indexer::UnBindRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::Error> {
        self.client.unary_call(&METHOD_INDEXER_UN_BIND, req, opt)
    }

    pub fn un_bind(&self, req: &super::indexer::UnBindRequest) -> ::grpcio::Result<super::common::Error> {
        self.un_bind_opt(req, ::grpcio::CallOption::default())
    }

    pub fn un_bind_async_opt(&self, req: &super::indexer::UnBindRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.client.unary_call_async(&METHOD_INDEXER_UN_BIND, req, opt)
    }

    pub fn un_bind_async(&self, req: &super::indexer::UnBindRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.un_bind_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Indexer {
    fn count(&mut self, ctx: ::grpcio::RpcContext, req: super::indexer::CountRequest, sink: ::grpcio::UnarySink<super::indexer::CountResponse>);
    fn query(&mut self, ctx: ::grpcio::RpcContext, req: super::indexer::QueryRequest, sink: ::grpcio::UnarySink<super::indexer::QueryResponse>);
    fn bind(&mut self, ctx: ::grpcio::RpcContext, req: super::indexer::BindRequest, sink: ::grpcio::UnarySink<super::common::Error>);
    fn un_bind(&mut self, ctx: ::grpcio::RpcContext, req: super::indexer::UnBindRequest, sink: ::grpcio::UnarySink<super::common::Error>);
}

pub fn create_indexer<S: Indexer + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEXER_COUNT, move |ctx, req, resp| {
        instance.count(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEXER_QUERY, move |ctx, req, resp| {
        instance.query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEXER_BIND, move |ctx, req, resp| {
        instance.bind(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_INDEXER_UN_BIND, move |ctx, req, resp| {
        instance.un_bind(ctx, req, resp)
    });
    builder.build()
}
