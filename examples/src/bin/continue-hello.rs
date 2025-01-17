//! Sends `b"hello"` to a given echo service, receives the response and formats
//! it into a new string.
//!
//! This example demonstrates:
//! "Future A waits for Future B, and continues with data obtained from B."
//!
//!
//! Run:
//! `cargo run --bin continue-hello`

use std::future::Future;
use std::net::UdpSocket;
use std::str::from_utf8;
use std::task::Poll;
use std::time::Instant;

use hello_async::{block_on, RecvWithWaker, ECHO_SOCKET_ADDR, HELLO, HELLO_BIND_SOCKET_ADDR};

struct A {
    b: B,
}

impl Future for A {
    type Output = Vec<u8>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let pinned_b = unsafe { self.map_unchecked_mut(|s| &mut s.b) };
        match pinned_b.poll(cx) {
            Poll::Ready(output) => {
                let processed = format!("{} world!", from_utf8(&output).unwrap()).into();
                Poll::Ready(processed)
            }
            Poll::Pending => Poll::Pending,
        }
    }
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
    let a = A { b };
    let start = Instant::now();
    let output = block_on(a);
    println!(
        "recv: {} ({:?})",
        std::str::from_utf8(&output).unwrap(),
        start.elapsed()
    );
}
