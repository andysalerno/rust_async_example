#![feature(async_await)]

mod mock_server;

use mock_server::{ MockServer};

#[runtime::main]
async fn main() {
    // Let's pretend we want to read from these two addresses, asynchronously.
    let future_a = MockServer::read_from_address("192.168.1.117");
    let future_b = MockServer::read_from_address("127.0.0.1");

    // By themselves, futures do nothing unless a runtime is executing them.
    // So we use the runtime crate to spawn our futures.
    let handle_a = runtime::spawn(future_a);
    let handle_b = runtime::spawn(future_b);

    // Now that we have spawned both futures, we can await them both.
    // The runtime can decide to execute them concurrently,
    // switching to one while the other is not ready.
    handle_a.await;
    handle_b.await;
}