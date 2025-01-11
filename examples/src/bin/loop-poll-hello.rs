//! Sends `b"hello"` to a given echo service and receives the response.
//!
//! Run
//! `cargo run --bin loop-poll-hello`

use std::future::Future;
use std::net::UdpSocket;
use std::pin::Pin;
use std::task::Poll;
use std::time::Instant;

use hello_async::{Recv, ECHO_SOCKET_ADDR, HELLO, HELLO_BIND_SOCKET_ADDR, NOOP_WAKER};

fn main() {
    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();

    socket.send(HELLO).expect("couldn't send message");

    let mut recv = unsafe { Recv::new(socket) };
    let mut recv = Pin::new(&mut recv);
    let mut cx = std::task::Context::from_waker(&NOOP_WAKER);

    let start = Instant::now();
    let output = loop {
        match recv.as_mut().poll(&mut cx) {
            Poll::Ready(output) => break output,
            Poll::Pending => continue,
        }
    };
    println!(
        "recv: {} ({:?})",
        std::str::from_utf8(&output).unwrap(),
        start.elapsed()
    );
}
