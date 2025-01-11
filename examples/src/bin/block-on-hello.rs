//! Sends `b"hello"` to a given echo service and receives the response.
//!
//! Using `RecvWithWaker` and `block_on`, this is a rewrite of "reactor-hello.rs".
//!
//! Run `cargo run --bin block-on-hello`

use std::net::UdpSocket;
use std::time::Instant;

use hello_async::{block_on, RecvWithWaker, ECHO_SOCKET_ADDR, HELLO, HELLO_BIND_SOCKET_ADDR};

fn main() {
    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();
    socket.send(HELLO).expect("couldn't send message");

    let recv = unsafe { RecvWithWaker::new(socket.try_clone().unwrap()) };
    let start = Instant::now();
    let output = block_on(recv);
    println!(
        "recv: {} ({:?})",
        std::str::from_utf8(&output).unwrap(),
        start.elapsed()
    );
}
