use std::future::Future;
use std::net::UdpSocket;
use std::task::Poll;
use std::{io, thread};

use polling::{Event, Events, Poller};

pub struct RecvWithWaker {
    socket: UdpSocket,
}

impl RecvWithWaker {
    /// Creates a new `RecvWithWaker`.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `socket` has been moved into nonblocking mode.
    #[must_use]
    pub unsafe fn new(socket: UdpSocket) -> Self {
        Self { socket }
    }
}

impl Future for RecvWithWaker {
    type Output = Vec<u8>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut buf = [0; 1024];
        match self.socket.recv(&mut buf) {
            Ok(n) => Poll::Ready(buf[..n].to_vec()),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                let key = 7;
                let poller = Poller::new().unwrap();
                unsafe {
                    poller.add(&self.socket, Event::readable(key)).unwrap();
                }
                let waker = cx.waker().clone();
                thread::spawn(move || {
                    let mut events = Events::new();
                    poller.wait(&mut events, None).unwrap();
                    waker.wake();
                });
                std::task::Poll::Pending
            }
            Err(e) => panic!("IO error: {e}"),
        }
    }
}
