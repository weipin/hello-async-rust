//! An UDP echo service which sends received data back to where it comes from.
//! The service waits one second before any sending, hence lazy.
//!
//! Implemented with the async runtime "smol".
//!
//! Run:
//! `cargo run --bin lazy-echo-udp-smol`
//!
//! Test:
//! `nc 127.0.0.1 1234 -u`

use async_io::{Async, Timer};
use hello_async::ECHO_SOCKET_ADDR;
use smol_macros::{main, Executor};
use std::net::UdpSocket;
use std::sync::Arc;
use std::time::Duration;

main! {
async fn main(ex: &Executor<'_>) {
    let socket = Async::<UdpSocket>::bind(ECHO_SOCKET_ADDR).unwrap();
    let socket = Arc::new(socket);

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
        println!("{len} bytes received from {addr}");

        ex.spawn({
            let buf = buf[..len].to_vec();
            let socket = socket.clone();
            async move {
                Timer::after(Duration::from_secs(1)).await;
                let len = socket.send_to(&buf, addr).await.unwrap();
                println!("{len} bytes sent");
            }
        }).detach();
    }
}
}
