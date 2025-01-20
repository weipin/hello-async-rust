//! A rewrite of "continue-hello" using `async` and `await`.
//!
//!
//! Run:
//! `cargo run --bin async-continue-hello`

use std::net::UdpSocket;
use std::str::from_utf8;
use std::time::Instant;

use hello_async::{block_on, RecvWithWaker, ECHO_SOCKET_ADDR, HELLO, HELLO_BIND_SOCKET_ADDR};

async fn a(b: RecvWithWaker) -> Vec<u8> {
    let output = b.await;
    format!("{} world!", from_utf8(&output).unwrap()).into()
}

type B = RecvWithWaker;

fn main() {
    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();
    socket.send(HELLO).expect("couldn't send message");

    let b = unsafe { B::new(socket.try_clone().unwrap()) };
    let start = Instant::now();
    let output = block_on(a(b));
    println!(
        "recv: {} ({:?})",
        std::str::from_utf8(&output).unwrap(),
        start.elapsed()
    );
}
