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

const METHOD_COORDINATOR_SELECT: ::grpcio::Method<
    super::coordinator::SelectRequest,
    super::coordinator::SelectResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Coordinator/Select",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_COORDINATOR_INSERT: ::grpcio::Method<
    super::coordinator::InsertRequest,
    super::common::Error,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Coordinator/Insert",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_COORDINATOR_UPDATE: ::grpcio::Method<
    super::coordinator::UpdateRequest,
    super::common::Error,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Coordinator/Update",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_COORDINATOR_DELETE: ::grpcio::Method<
    super::coordinator::DeleteRequest,
    super::common::Error,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Coordinator/Delete",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_COORDINATOR_DISCOVER: ::grpcio::Method<
    super::coordinator::DiscoverRequest,
    super::coordinator::DiscoverResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Coordinator/Discover",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_COORDINATOR_REGISTER: ::grpcio::Method<
    super::coordinator::NodeRequest,
    super::common::Error,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/protos.Coordinator/Register",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

#[derive(Clone)]
pub struct CoordinatorClient {
    client: ::grpcio::Client,
}

impl CoordinatorClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        CoordinatorClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn select_opt(
        &self,
        req: &super::coordinator::SelectRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::coordinator::SelectResponse> {
        self.client.unary_call(&METHOD_COORDINATOR_SELECT, req, opt)
    }

    pub fn select(
        &self,
        req: &super::coordinator::SelectRequest,
    ) -> ::grpcio::Result<super::coordinator::SelectResponse> {
        self.select_opt(req, ::grpcio::CallOption::default())
    }

    pub fn select_async_opt(
        &self,
        req: &super::coordinator::SelectRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::coordinator::SelectResponse>> {
        self.client
            .unary_call_async(&METHOD_COORDINATOR_SELECT, req, opt)
    }

    pub fn select_async(
        &self,
        req: &super::coordinator::SelectRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::coordinator::SelectResponse>> {
        self.select_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn insert_opt(
        &self,
        req: &super::coordinator::InsertRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::common::Error> {
        self.client.unary_call(&METHOD_COORDINATOR_INSERT, req, opt)
    }

    pub fn insert(
        &self,
        req: &super::coordinator::InsertRequest,
    ) -> ::grpcio::Result<super::common::Error> {
        self.insert_opt(req, ::grpcio::CallOption::default())
    }

    pub fn insert_async_opt(
        &self,
        req: &super::coordinator::InsertRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.client
            .unary_call_async(&METHOD_COORDINATOR_INSERT, req, opt)
    }

    pub fn insert_async(
        &self,
        req: &super::coordinator::InsertRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.insert_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_opt(
        &self,
        req: &super::coordinator::UpdateRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::common::Error> {
        self.client.unary_call(&METHOD_COORDINATOR_UPDATE, req, opt)
    }

    pub fn update(
        &self,
        req: &super::coordinator::UpdateRequest,
    ) -> ::grpcio::Result<super::common::Error> {
        self.update_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_async_opt(
        &self,
        req: &super::coordinator::UpdateRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.client
            .unary_call_async(&METHOD_COORDINATOR_UPDATE, req, opt)
    }

    pub fn update_async(
        &self,
        req: &super::coordinator::UpdateRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.update_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(
        &self,
        req: &super::coordinator::DeleteRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::common::Error> {
        self.client.unary_call(&METHOD_COORDINATOR_DELETE, req, opt)
    }

    pub fn delete(
        &self,
        req: &super::coordinator::DeleteRequest,
    ) -> ::grpcio::Result<super::common::Error> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(
        &self,
        req: &super::coordinator::DeleteRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.client
            .unary_call_async(&METHOD_COORDINATOR_DELETE, req, opt)
    }

    pub fn delete_async(
        &self,
        req: &super::coordinator::DeleteRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn discover_opt(
        &self,
        req: &super::coordinator::DiscoverRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::coordinator::DiscoverResponse> {
        self.client
            .unary_call(&METHOD_COORDINATOR_DISCOVER, req, opt)
    }

    pub fn discover(
        &self,
        req: &super::coordinator::DiscoverRequest,
    ) -> ::grpcio::Result<super::coordinator::DiscoverResponse> {
        self.discover_opt(req, ::grpcio::CallOption::default())
    }

    pub fn discover_async_opt(
        &self,
        req: &super::coordinator::DiscoverRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::coordinator::DiscoverResponse>> {
        self.client
            .unary_call_async(&METHOD_COORDINATOR_DISCOVER, req, opt)
    }

    pub fn discover_async(
        &self,
        req: &super::coordinator::DiscoverRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::coordinator::DiscoverResponse>> {
        self.discover_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_opt(
        &self,
        req: &super::coordinator::NodeRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::common::Error> {
        self.client
            .unary_call(&METHOD_COORDINATOR_REGISTER, req, opt)
    }

    pub fn register(
        &self,
        req: &super::coordinator::NodeRequest,
    ) -> ::grpcio::Result<super::common::Error> {
        self.register_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_async_opt(
        &self,
        req: &super::coordinator::NodeRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.client
            .unary_call_async(&METHOD_COORDINATOR_REGISTER, req, opt)
    }

    pub fn register_async(
        &self,
        req: &super::coordinator::NodeRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Error>> {
        self.register_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait Coordinator {
    fn select(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::coordinator::SelectRequest,
        sink: ::grpcio::UnarySink<super::coordinator::SelectResponse>,
    );
    fn insert(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::coordinator::InsertRequest,
        sink: ::grpcio::UnarySink<super::common::Error>,
    );
    fn update(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::coordinator::UpdateRequest,
        sink: ::grpcio::UnarySink<super::common::Error>,
    );
    fn delete(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::coordinator::DeleteRequest,
        sink: ::grpcio::UnarySink<super::common::Error>,
    );
    fn discover(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::coordinator::DiscoverRequest,
        sink: ::grpcio::UnarySink<super::coordinator::DiscoverResponse>,
    );
    fn register(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::coordinator::NodeRequest,
        sink: ::grpcio::UnarySink<super::common::Error>,
    );
}

pub fn create_coordinator<S: Coordinator + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_COORDINATOR_SELECT, move |ctx, req, resp| {
        instance.select(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_COORDINATOR_INSERT, move |ctx, req, resp| {
        instance.insert(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_COORDINATOR_UPDATE, move |ctx, req, resp| {
        instance.update(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_COORDINATOR_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_COORDINATOR_DISCOVER, move |ctx, req, resp| {
        instance.discover(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_COORDINATOR_REGISTER, move |ctx, req, resp| {
        instance.register(ctx, req, resp)
    });
    builder.build()
}
