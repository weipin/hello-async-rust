//! Code examples for "Hello async Rust"

use std::future::Future;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::task::{Poll, Waker};

pub const TASKS_TOTAL_NUM: usize = 1_000;

// Default address of the echo service.
const ECHO_IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const ECHO_SOCKET_ADDR: SocketAddr = SocketAddr::new(ECHO_IP_ADDR, 1234);

// Default address of the "Hello UDP" examples.
const HELLO_BIND_IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const HELLO_BIND_SOCKET_ADDR: SocketAddr = SocketAddr::new(HELLO_BIND_IP_ADDR, 12345);

// Data that the "Hello UDP" examples send.
pub const HELLO: &[u8; 5] = b"hello";

// ANCHOR: RecvOnce

/// Future which receives one response from a given service.
pub struct RecvOnce {
    socket: UdpSocket,
}

impl RecvOnce {
    /// Creates a new RecvOnce.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `socket` has been moved into nonblocking mode.
    pub unsafe fn new(socket: UdpSocket) -> Self {
        Self { socket }
    }
}

impl Future for RecvOnce {
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
// ANCHOR_END: RecvOnce

/// Returns a Waker that does nothing when used.
const fn noop_waker() -> Waker {
    const VTABLE: std::task::RawWakerVTable = std::task::RawWakerVTable::new(
        // Cloning just returns a new no-op raw waker
        |_| RAW,
        // `wake` does nothing
        |_| {},
        // `wake_by_ref` does nothing
        |_| {},
        // Dropping does nothing as we don't allocate anything
        |_| {},
    );
    const RAW: std::task::RawWaker = std::task::RawWaker::new(std::ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(RAW) }
}

pub static NOOP_WAKER: Waker = noop_waker();
