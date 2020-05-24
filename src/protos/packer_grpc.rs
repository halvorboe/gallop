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

const METHOD_PACKER_INSERT: ::grpcio::Method<super::packer::PackerInsertRequest, super::common::Error> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Packer/Insert",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PACKER_SEGMENT: ::grpcio::Method<super::packer::SegmentRequest, super::packer::SegmentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Packer/Segment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PACKER_SEGMENTS: ::grpcio::Method<super::packer::SegmentsRequest, super::packer::SegmentsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Packer/Segments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct PackerClient {
    client: ::grpcio::Client,
}

impl PackerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        PackerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn insert_opt(&self, req: &super::packer::PackerInsertRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::Error> {
        self.client.unary_call(&METHOD_PACKER_INSERT, req, opt)
    }

    pub fn insert(&self, req: &super::packer::PackerInsertRequest) -> ::grpcio::Result<super::common::Error> {
        self.insert_opt(req, ::grpcio::CallOption::default())
    }

    pub fn insert_async_opt(&self, req: &super::packer::PackerInsertRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.client.unary_call_async(&METHOD_PACKER_INSERT, req, opt)
    }

    pub fn insert_async(&self, req: &super::packer::PackerInsertRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.insert_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn segment_opt(&self, req: &super::packer::SegmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::packer::SegmentResponse> {
        self.client.unary_call(&METHOD_PACKER_SEGMENT, req, opt)
    }

    pub fn segment(&self, req: &super::packer::SegmentRequest) -> ::grpcio::Result<super::packer::SegmentResponse> {
        self.segment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn segment_async_opt(&self, req: &super::packer::SegmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::packer::SegmentResponse>> {
        self.client.unary_call_async(&METHOD_PACKER_SEGMENT, req, opt)
    }

    pub fn segment_async(&self, req: &super::packer::SegmentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::packer::SegmentResponse>> {
        self.segment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn segments_opt(&self, req: &super::packer::SegmentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::packer::SegmentsResponse> {
        self.client.unary_call(&METHOD_PACKER_SEGMENTS, req, opt)
    }

    pub fn segments(&self, req: &super::packer::SegmentsRequest) -> ::grpcio::Result<super::packer::SegmentsResponse> {
        self.segments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn segments_async_opt(&self, req: &super::packer::SegmentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::packer::SegmentsResponse>> {
        self.client.unary_call_async(&METHOD_PACKER_SEGMENTS, req, opt)
    }

    pub fn segments_async(&self, req: &super::packer::SegmentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::packer::SegmentsResponse>> {
        self.segments_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Packer {
    fn insert(&mut self, ctx: ::grpcio::RpcContext, req: super::packer::PackerInsertRequest, sink: ::grpcio::UnarySink<super::common::Error>);
    fn segment(&mut self, ctx: ::grpcio::RpcContext, req: super::packer::SegmentRequest, sink: ::grpcio::UnarySink<super::packer::SegmentResponse>);
    fn segments(&mut self, ctx: ::grpcio::RpcContext, req: super::packer::SegmentsRequest, sink: ::grpcio::UnarySink<super::packer::SegmentsResponse>);
}

pub fn create_packer<S: Packer + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PACKER_INSERT, move |ctx, req, resp| {
        instance.insert(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PACKER_SEGMENT, move |ctx, req, resp| {
        instance.segment(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_PACKER_SEGMENTS, move |ctx, req, resp| {
        instance.segments(ctx, req, resp)
    });
    builder.build()
}
