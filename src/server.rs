use std::sync::Arc;
use std::{io, thread};
use std::io::Read;
use futures_channel::oneshot;
use futures_executor::block_on;
use futures_util::{FutureExt as _, TryFutureExt as _};
use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, ServerCredentials, UnarySink};
use ::proto::helloworld::HelloReply;
use ::proto::helloworld::HelloRequest;
use ::proto::helloworld_grpc::Greeter;
use proto::helloworld_grpc::create_greeter;

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext,
                 req: HelloRequest,
                 sink: UnarySink<HelloReply>) {
        let msg = format!("Hello {}", req.name);
        let mut resp = HelloReply::default();
        resp.message = msg;
        let f = sink
            .success(resp)
            .map_err(move |e| eprintln!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }
}


fn main() {
    let env = Arc::new(Environment::new(1));
    let service = create_greeter(GreeterService);
    let addr = "127.0.0.1:50051";

    let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server
        .add_listening_port(addr, ServerCredentials::insecure())
        .unwrap();
    server.start();
    println!("listening on {addr}");

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}