//! Sends `b"hello"` to a given echo service and receives the response.
//!
//! This example demonstrates:
//! 1. Usage of "reactor".
//! 1. How to block the current thread until data is obtained.
//!
//! This example doesn't rely on wakers -- a dummy "noop_waker" is used.
//!
//! Run
//! `cargo run --bin reactor-hello-udp`

use std::future::Future;
use std::net::UdpSocket;
use std::pin::Pin;
use std::task::Poll;
use std::thread;
use std::time::Instant;

use hello_async::{RecvOnce, ECHO_SOCKET_ADDR, HELLO, HELLO_BIND_SOCKET_ADDR, NOOP_WAKER};
use polling::{Event, Events, Poller};

fn main() {
    // Send HELLO.
    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();

    socket.send(HELLO).expect("couldn't send message");

    let mut recv_once = unsafe { RecvOnce::new(socket.try_clone().unwrap()) };
    let mut recv_once = Pin::new(&mut recv_once);

    let start = Instant::now();
    let mut cx = std::task::Context::from_waker(&NOOP_WAKER);
    // The first poll
    match recv_once.as_mut().poll(&mut cx) {
        Poll::Ready(_) => {
            panic!("Invalid future state");
        }
        Poll::Pending => {
            // Waits for the readable event in a new thread, and park the main
            // thread.

            // Arbitrary key identifying the socket.
            let key = 7;
            // Creates a poller and registers interest in readability on the socket.
            let poller = Poller::new().unwrap();
            unsafe {
                poller.add(&socket, Event::readable(key)).unwrap();
            }
            let main_thread = thread::current();
            thread::spawn(move || {
                let mut events = Events::new();
                poller.wait(&mut events, None).unwrap();
                // The socket is readable, unpark the main thread.
                main_thread.unpark();
            });
            thread::park();
        }
    }

    // The second poll. At this point, the main thread has been unparked, and
    // the socket is ready to read.
    match recv_once.as_mut().poll(&mut cx) {
        Poll::Ready(output) => {
            println!(
                "recv: {} ({:?})",
                std::str::from_utf8(&output).unwrap(),
                start.elapsed()
            );
        }
        Poll::Pending => {
            panic!("Invalid future state");
        }
    }
}
