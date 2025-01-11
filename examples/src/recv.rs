// ANCHOR: Recv

use std::future::Future;
use std::io;
use std::net::UdpSocket;
use std::task::Poll;

/// Future which receives one response from a given service.
pub struct Recv {
    socket: UdpSocket,
}

impl Recv {
    /// Creates a new Recv.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `socket` has been moved into nonblocking mode.
    pub unsafe fn new(socket: UdpSocket) -> Self {
        Self { socket }
    }
}

impl Future for Recv {
    type Output = Vec<u8>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        let mut buf = [0; 1024];
        match self.socket.recv(&mut buf) {
            Ok(n) => Poll::Ready(buf[..n].to_vec()),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => std::task::Poll::Pending,
            Err(e) => panic!("IO error: {e}"),
        }
    }
}
// ANCHOR_END: Recv
