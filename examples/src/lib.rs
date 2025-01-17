//! Code examples for "Hello async Rust"

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::task::Waker;

mod block_on;
mod recv;
mod recv_with_waker;
mod waker_fn;

pub use block_on::block_on;
pub use recv::Recv;
pub use recv_with_waker::RecvWithWaker;
pub use waker_fn::waker_fn;

pub const TASKS_TOTAL_NUM: usize = 1_000;

// Default address of the echo service.
const ECHO_IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const ECHO_SOCKET_ADDR: SocketAddr = SocketAddr::new(ECHO_IP_ADDR, 1234);
pub const ECHO_SOCKET_ADDR2: SocketAddr = SocketAddr::new(ECHO_IP_ADDR, 1235);
pub const ECHO_SOCKET_ADDR3: SocketAddr = SocketAddr::new(ECHO_IP_ADDR, 1236);

// Default address of the "Hello UDP" examples.
const HELLO_BIND_IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const HELLO_BIND_SOCKET_ADDR: SocketAddr = SocketAddr::new(HELLO_BIND_IP_ADDR, 12345);
pub const HELLO_BIND_SOCKET_ADDR2: SocketAddr = SocketAddr::new(HELLO_BIND_IP_ADDR, 12346);
pub const HELLO_BIND_SOCKET_ADDR3: SocketAddr = SocketAddr::new(HELLO_BIND_IP_ADDR, 12347);

// Data that the "Hello UDP" examples send.
pub const HELLO: &[u8; 5] = b"hello";

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
