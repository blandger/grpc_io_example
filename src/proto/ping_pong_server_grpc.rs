// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_PING_PONG_SERVICE_PONG: ::grpcio::Method<super::ping_pong_server::PingRequest, super::ping_pong_server::PongReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc_io_example.PingPongService/pong",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct PingPongServiceClient {
    client: ::grpcio::Client,
}

impl PingPongServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        PingPongServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn pong_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::ping_pong_server::PingRequest>, ::grpcio::ClientDuplexReceiver<super::ping_pong_server::PongReply>)> {
        self.client.duplex_streaming(&METHOD_PING_PONG_SERVICE_PONG, opt)
    }

    pub fn pong(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::ping_pong_server::PingRequest>, ::grpcio::ClientDuplexReceiver<super::ping_pong_server::PongReply>)> {
        self.pong_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait PingPongService {
    fn pong(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::ping_pong_server::PingRequest>, sink: ::grpcio::DuplexSink<super::ping_pong_server::PongReply>);
}

pub fn create_ping_pong_service<S: PingPongService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(&METHOD_PING_PONG_SERVICE_PONG, move |ctx, req, resp| {
        instance.pong(ctx, req, resp)
    });
    builder.build()
}
