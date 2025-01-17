//! Sends `b"hello"` and `b"world"` separately to two given echo services,
//! receives the responses and concatenates them into a new string.
//!
//! This example demonstrates:
//! "Future B joins futures C and D, waiting for both to complete."
//!
//!
//! Run:
//! `cargo run --bin join-hello`
//!
//!
//! Requires two echo services running at specific ports.
//! ```
//! trap 'kill $PID1; kill $PID2; exit' INT
//! cargo run --bin lazy-echo-udp-smol -- 127.0.0.1:1234 &
//! PID1=$!
//! cargo run --bin lazy-echo-udp-smol -- 127.0.0.1:1235 &
//! PID2=$!
//! wait
//! ```

use std::future::Future;
use std::net::UdpSocket;
use std::pin::Pin;
use std::str::from_utf8;
use std::task::Poll;
use std::time::Instant;

use hello_async::{
    block_on, RecvWithWaker, ECHO_SOCKET_ADDR, ECHO_SOCKET_ADDR2, HELLO, HELLO_BIND_SOCKET_ADDR,
    HELLO_BIND_SOCKET_ADDR2,
};

struct B {
    c: C,
    c_output: Option<Vec<u8>>,
    d: D,
    d_output: Option<Vec<u8>>,
}

impl Future for B {
    type Output = Vec<u8>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { Pin::get_unchecked_mut(self) };

        if this.c_output.is_none() {
            let pinned_c = unsafe { Pin::new_unchecked(&mut this.c) };
            if let Poll::Ready(output) = pinned_c.poll(cx) {
                this.c_output = Some(output);
            }
        }
        if this.d_output.is_none() {
            let pinned_d = unsafe { Pin::new_unchecked(&mut this.d) };
            if let Poll::Ready(output) = pinned_d.poll(cx) {
                this.d_output = Some(output);
            }
        }

        if let (Some(c_output), Some(d_output)) = (&this.c_output, &this.d_output) {
            let processed = format!(
                "{} {}!",
                from_utf8(c_output).unwrap(),
                from_utf8(d_output).unwrap()
            );
            Poll::Ready(processed.into())
        } else {
            Poll::Pending
        }
    }
}

type C = RecvWithWaker;
type D = RecvWithWaker;

fn main() {
    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();
    socket.send(HELLO).expect("couldn't send message");
    let c = unsafe { C::new(socket.try_clone().unwrap()) };

    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR2).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR2)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();
    socket.send(b"world").expect("couldn't send message");
    let d = unsafe { D::new(socket.try_clone().unwrap()) };

    let b = B {
        c,
        c_output: None,
        d,
        d_output: None,
    };
    let start = Instant::now();
    let output = block_on(b);
    println!(
        "recv: {} ({:?})",
        from_utf8(&output).unwrap(),
        start.elapsed()
    );
}
