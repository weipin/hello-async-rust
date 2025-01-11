//! Repeatedly sends `b"hello"` to a given echo service and receives all the
//! response.
//!
//! Run
//! `cargo run --bin loop-poll-many-hello`

use std::future::Future;
use std::net::UdpSocket;
use std::pin::Pin;
use std::task::Poll;
use std::time::Instant;

use hello_async::{
    Recv, ECHO_SOCKET_ADDR, HELLO, HELLO_BIND_SOCKET_ADDR, NOOP_WAKER, TASKS_TOTAL_NUM,
};

fn main() {
    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();

    let mut cx = std::task::Context::from_waker(&NOOP_WAKER);

    let mut futures = Vec::with_capacity(TASKS_TOTAL_NUM);
    for _ in 0..TASKS_TOTAL_NUM {
        let socket = socket.try_clone().expect("couldn't clone the socket");
        socket.send(HELLO).expect("couldn't send message");
        let recv = Box::new(unsafe { Recv::new(socket) });
        let recv = Pin::new(recv);
        futures.push(recv);
    }

    let start = Instant::now();
    loop {
        futures.retain_mut(|recv| match recv.as_mut().poll(&mut cx) {
            Poll::Ready(output) => {
                assert_eq!(output, HELLO, "hello does not match");
                false
            }
            Poll::Pending => true,
        });
        if futures.is_empty() {
            break;
        }
    }
    println!("recv total: {} ({:?})", TASKS_TOTAL_NUM, start.elapsed());
}
