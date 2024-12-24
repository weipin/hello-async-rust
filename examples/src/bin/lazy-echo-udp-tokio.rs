//! An UDP echo service which sends received data back to where it comes from.
//! The service waits one second before any sending, hence lazy.
//!
//! Implemented with the async runtime "tokio".
//!
//! Run
//! `cargo run --bin lazy-echo-udp-tokio`
//!
//! Test
//! `nc 127.0.0.1 1234 -u`

use hello_async::ECHO_SOCKET_ADDR;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(&ECHO_SOCKET_ADDR).await?;
    let socket = Arc::new(socket);

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("{len} bytes received from {addr}");

        tokio::spawn({
            let buf = buf[..len].to_vec();
            let socket = socket.clone();
            async move {
                sleep(Duration::from_secs(1)).await;
                let len = socket.send_to(&buf, &addr).await.unwrap();
                println!("{len} bytes sent");
            }
        });
    }
}
