## Rust experiments for selecting server-side gRPC library

Here are two servers and clients just for testing purpose. Project was used for testing and comparing 'tonic' and 'grpcio' libraries. Finally, the [Tonic](https://github.com/hyperium/tonic) was selected because of reliability. grpcio had work failures during load tests.

### Build (for release)

> cargo build --release

#### Running Ping pong client and server

A gRPC ping pong example. There are two streams inside bidirectional channel exchanging 'ping-pong counter' value between them.

run by commands:

> RUST_BACKTRACE=1 cargo run --bin ping-pong-server

> RUST_BACKTRACE=1 cargo run --bin ping-pong-client

### Hello world client and server

It was used for checking performance, load, reliability check and comparing against 'tonic'.

> cargo run --bin hello-server

OR run release build by

> cd /target/release
> ./hello-server

> cargo run --bin hello-client

### Request per secons testing

The [ghz](https://ghz.sh/docs/usage) utility has been used for load, reliability check.

Use it instead of 'hello-client' by command:

> /path/to/ghz/ghz --insecure --proto ./proto/hello.proto --call hello.Greeter.SayHello -c 10000 -n 100000 -d '{"name": "Joe"}' 127.0.0.1:50051

-c 10000 -n 100000 parameters mean create 10000 concurrent clients and do 100000 run passes