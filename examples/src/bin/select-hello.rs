//! Sends three different pieces of data to three given echo services. Exits
//! upon receiving any of the responses.
//!
//! This example demonstrates:
//! "Future D selects future E, F, and G, waiting for any of the three to complete."
//!
//!
//! Run:
//! `cargo run --bin select-hello`
//!
//!
//! Requires three echo services running at specific ports.
//! ```
//! trap 'kill $PID1; kill $PID2; kill $PID3; exit' INT
//! cargo run --bin lazy-echo-udp-tokio -- 127.0.0.1:1234 &
//! PID1=$!
//! cargo run --bin lazy-echo-udp-tokio -- 127.0.0.1:1235 &
//! PID2=$!
//! cargo run --bin lazy-echo-udp-tokio -- 127.0.0.1:1236 &
//! PID3=$!
//! wait
//! ```

use std::future::Future;
use std::net::UdpSocket;
use std::pin::Pin;
use std::str::from_utf8;
use std::task::Poll;
use std::time::Instant;

use hello_async::{
    block_on, RecvWithWaker, ECHO_SOCKET_ADDR, ECHO_SOCKET_ADDR2, ECHO_SOCKET_ADDR3,
    HELLO_BIND_SOCKET_ADDR, HELLO_BIND_SOCKET_ADDR2, HELLO_BIND_SOCKET_ADDR3,
};

struct D {
    e: E,
    f: F,
    g: G,
}

impl Future for D {
    type Output = Vec<u8>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { Pin::get_unchecked_mut(self) };

        if let Poll::Ready(output) = unsafe { Pin::new_unchecked(&mut this.e) }.poll(cx) {
            return Poll::Ready(output);
        }
        if let Poll::Ready(output) = unsafe { Pin::new_unchecked(&mut this.f) }.poll(cx) {
            return Poll::Ready(output);
        }
        if let Poll::Ready(output) = unsafe { Pin::new_unchecked(&mut this.g) }.poll(cx) {
            return Poll::Ready(output);
        }
        Poll::Pending
    }
}

type E = RecvWithWaker;
type F = RecvWithWaker;
type G = RecvWithWaker;

fn main() {
    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();
    socket.send(b"hello 1").expect("couldn't send message");
    let e = unsafe { E::new(socket.try_clone().unwrap()) };

    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR2).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR2)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();
    socket.send(b"hello 2").expect("couldn't send message");
    let f = unsafe { F::new(socket.try_clone().unwrap()) };

    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR3).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR3)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();
    socket.send(b"hello 3").expect("couldn't send message");
    let g = unsafe { G::new(socket.try_clone().unwrap()) };

    let d = D { e, f, g };
    let start = Instant::now();
    let output = block_on(d);
    println!(
        "recv: {} ({:?})",
        from_utf8(&output).unwrap(),
        start.elapsed()
    );
}
