//! Repeatedly sends a incremental integer to a given echo service and receives
//! all the response. Each response is checked against its original sending.
//! For every `MATCH_PROGRESS_SIZE` matches, a "." is printed for progress
//! reporting.
//!
//! This example demonstrates:
//! 1. Usage of "reactor".
//! 1. Storage of futures and wakers.
//! 1. "Waking" implementation.
//!
//! This example doesn't rely on "polling queue" and "channel". The "waking"
//! code polls a future directly and processes the data immediately.
//!
//! Run
//! `cargo run --bin waker-many-hello-udp`

use std::collections::HashMap;
use std::future::Future;
use std::net::UdpSocket;
use std::pin::Pin;
use std::sync::{Arc, LazyLock, Mutex};
use std::task::{Poll, RawWaker, RawWakerVTable, Waker};
use std::time::Instant;
use std::{io, mem, thread};

use hello_async::{ECHO_SOCKET_ADDR, HELLO_BIND_SOCKET_ADDR, TASKS_TOTAL_NUM};
use polling::{Event, Events, Poller};

type FutureID = usize;
type FutureMap = Arc<Mutex<HashMap<FutureID, Pin<Box<MyFuture>>>>>;
type WakerMap = Arc<Mutex<HashMap<FutureID, Waker>>>;

static FUTURES: LazyLock<FutureMap> =
    LazyLock::new(|| FutureMap::new(Mutex::new(HashMap::with_capacity(TASKS_TOTAL_NUM))));
static WAKERS: LazyLock<WakerMap> =
    LazyLock::new(|| WakerMap::new(Mutex::new(HashMap::with_capacity(TASKS_TOTAL_NUM))));

const BUF_SIZE: usize = mem::size_of::<usize>();
const MATCH_PROGRESS_SIZE: usize = 20;

/// Future which receives one response from a given service.
struct MyFuture {
    socket: UdpSocket,
}

impl MyFuture {
    /// Creates a new RecvOnce.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `socket` has been moved into nonblocking mode.
    pub unsafe fn new(socket: UdpSocket) -> Self {
        Self { socket }
    }
}

impl Future for MyFuture {
    type Output = FutureID;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut buf = [0; BUF_SIZE];
        match self.socket.recv(&mut buf) {
            Ok(n) => {
                assert_eq!(n, BUF_SIZE);
                Poll::Ready(usize::from_be_bytes(buf))
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                let mut wakers = WAKERS.lock().unwrap();
                wakers.insert(cx.waker().data() as FutureID, cx.waker().clone());

                std::task::Poll::Pending
            }
            Err(e) => panic!("IO error: {e}"),
        }
    }
}

const VTABLE: RawWakerVTable = RawWakerVTable::new(
    // Cloning returns a new raw waker
    |data| new_raw_waker(data as FutureID),
    // `wake` not implemented
    |_| unimplemented!(),
    // `wake_by_ref`
    |data| {
        let future_id = data as FutureID;
        wake(future_id);
    },
    // Dropping does nothing as we don't allocate anything
    |_| {},
);

/// Polls a future and processes the obtained data.
fn wake(future_id: FutureID) {
    assert!(future_id < TASKS_TOTAL_NUM);
    let mut futures = FUTURES.lock().unwrap();
    let recv_once = futures.get_mut(&future_id).unwrap();
    let waker = new_waker(future_id);
    let mut cx = std::task::Context::from_waker(&waker);

    match recv_once.as_mut().poll(&mut cx) {
        Poll::Ready(output) => {
            assert_eq!(output, future_id);
            if future_id % MATCH_PROGRESS_SIZE == 0 {
                print!(".");
            }
            futures.remove(&future_id);
        }
        Poll::Pending => {
            panic!("Invalid state");
        }
    }
}

/// Creates a new RawWaker.
fn new_raw_waker(future_id: FutureID) -> RawWaker {
    RawWaker::new(future_id as *const (), &VTABLE)
}

/// Creates a new Waker
fn new_waker(future_id: FutureID) -> Waker {
    let raw = new_raw_waker(future_id);
    unsafe { Waker::from_raw(raw) }
}

fn main() {
    let socket = UdpSocket::bind(HELLO_BIND_SOCKET_ADDR).expect("couldn't bind to address");
    socket
        .connect(ECHO_SOCKET_ADDR)
        .expect("connecting echo failed");
    socket.set_nonblocking(true).unwrap();

    {
        let mut futures = FUTURES.lock().unwrap();
        for i in 0..TASKS_TOTAL_NUM {
            let socket = socket.try_clone().expect("couldn't clone the socket");
            socket
                .send(&i.to_be_bytes())
                .expect("couldn't send message");
            let recv_once = Box::new(unsafe { MyFuture::new(socket) });
            let recv_once = Pin::new(recv_once);
            futures.insert(i, recv_once);
        }

        // The first poll to all futures.
        for (&future_id, recv_once) in futures.iter_mut() {
            let waker = new_waker(future_id);
            let mut cx = std::task::Context::from_waker(&waker);

            // Registers the waker.
            match recv_once.as_mut().poll(&mut cx) {
                Poll::Ready(_) => {
                    panic!("Invalid state");
                }
                Poll::Pending => {
                    // noop
                }
            }
        }
    }

    // Starts event loop

    // Arbitrary key identifying the socket.
    let key = 7;
    // Create a poller and register interest in readability on the socket.
    let poller = Poller::new().unwrap();
    unsafe {
        poller.add(&socket, Event::readable(key)).unwrap();
    }
    let main_thread = thread::current();
    thread::spawn(move || {
        let mut events = Events::new();
        let mut buf = [0; BUF_SIZE];

        loop {
            events.clear();
            poller.wait(&mut events, None).unwrap();

            match socket.peek(&mut buf) {
                Ok(n) => {
                    assert_eq!(n, BUF_SIZE);
                    let future_id = usize::from_be_bytes(buf);
                    let wakers = WAKERS.lock().unwrap();
                    let waker = wakers.get(&future_id).unwrap();
                    waker.wake_by_ref();

                    let futures = FUTURES.lock().unwrap();
                    if futures.is_empty() {
                        main_thread.unpark();
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => panic!("Invalid state"),
                Err(e) => panic!("IO error: {e}"),
            }

            poller.modify(&socket, Event::readable(key)).unwrap();
        }
    });

    let start = Instant::now();
    thread::park();

    println!("\nrecv total: {} ({:?})", TASKS_TOTAL_NUM, start.elapsed());
}
