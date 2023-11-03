use std::sync::Arc;
use std::thread;
use std::io::Read;
use futures_channel::oneshot;
use futures_executor::block_on;
use futures_util::{FutureExt as _, SinkExt as _, TryFutureExt as _, TryStreamExt as _};
use ::proto::ping_pong_server::{PingRequest, PongReply};
use ::proto::ping_pong_server_grpc::{create_ping_pong_service, PingPongService};
use grpcio::{DuplexSink, RequestStream, RpcContext};
use grpcio::*;

// Struct for grpc service
#[derive(Clone)]
struct Service;

// grpc service implementation
impl PingPongService for Service {

    fn pong(
        &mut self,
        ctx: RpcContext,
        mut stream: RequestStream<PingRequest>,
        mut sink: DuplexSink<PongReply>,
    ) {
        println!("Receiving request stream....");
        let f = async move {
            while let Some(ping) = stream.try_next().await? {
                let ping_value = ping.get_countIn();
                println!("Got ping = {:?}", &ping_value);
                let updated_count = ping_value + 1;
                let mut replay: PongReply = PongReply::new();
                replay.set_countOut(updated_count);
                println!("Trying to send pong = {:?}", &replay);
                sink.send((replay.clone(), WriteFlags::default())).await?;
                println!("Sent pong = {:?}", &replay);
            }
            sink.close().await?;
            Ok(())
        }.map_err(|e: grpcio::Error| eprintln!("failed to record route: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }
}

fn main() {
    println!("Starting server...");
    let addr = "127.0.0.1:50051";
    let instance = Service{};
    let service = create_ping_pong_service(instance);
    let env = Arc::new(Environment::new(2));
    let mut server = ServerBuilder::new(env)
        .register_service(service)
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
        let _ = std::io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    block_on(rx).unwrap();
    block_on(server.shutdown()).unwrap();
}
