use std::sync::Arc;
use grpcio::{ChannelBuilder, Environment, WriteFlags};
use ::proto::ping_pong_server::PingRequest;
use ::proto::ping_pong_server_grpc::PingPongServiceClient;
use futures_util::{SinkExt as _, TryStreamExt as _};
use futures_util::lock::Mutex;
use ::proto::error::{Result};

async fn run_ping_pong(client: &PingPongServiceClient) -> Result<()> {
    let (sink, mut receiver) = client.pong()?;
    let arc_sync = Arc::new(Mutex::new(sink));
    let mut request: PingRequest = PingRequest::new();
    request.set_countIn(0);
    println!("prepared first request: {:?}", &request);

    let sink2 = arc_sync.clone();
    let send = async move {
        println!("Sending first request... {:?}", &request);
        sink2.lock().await.send((request, WriteFlags::default())).await?;
        println!("Sent first request - OK !");
        Ok(()) as Result<_>
    };

    let sink3 = arc_sync.clone();
    let receive = async move {
        while let Some(pong) = receiver.try_next().await? {
            println!("Got pong = {:?}", &pong);
            let updated_count = pong.get_countOut() + 1;
            let mut replay: PingRequest = PingRequest::new();
            replay.set_countIn(updated_count);
            sink3.lock().await.send((replay.clone(), WriteFlags::default())).await?;
        }
        Ok(()) as Result<_>
    };
    let (sr, rr) = futures_util::join!(send, receive);
    sr.and(rr)?;
    Ok(())
}

async fn async_main() -> Result<()> {
    let env = Arc::new(Environment::new(2));
    let address = "127.0.0.1:50051";
    let channel = ChannelBuilder::new(env).connect(address);
    let client = PingPongServiceClient::new(channel);
    println!("Client connected to address = {:?}", &address);
    run_ping_pong(&client).await?;
    Ok(())
}

fn main() {
    println!("Starting client...");

    match futures_executor::block_on(async_main()) {
        Ok(_) => {
            println!("Works fine...")
        },
        Err(err) => {
            eprintln!("Error: {err}")
        }
    }
}