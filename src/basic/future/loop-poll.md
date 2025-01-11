# Polling in a loop

To obtain the value of a Future, one obvious approach is to call the method
`poll` in a loop until it returns `Poll::Ready(T)`.

## Example

### "UDP Hello" examples

The "UDP Hello" examples, which run on UDP protocol, send data
`b"hello"`(repeatedly) to a given echo service and receive the response(s).

The examples include "loop-poll-hello" and "loop-poll-many-hello".

### `Recv`

`Recv`, as a major part of the "UDP Hello" examples, is a Future which receives
one response from a given service. Its `poll` implementation reads data from a
given UDP socket. `Recv` requires and assumes that the UDP socket is in
nonblocking mode.

Source: [examples/src/recv.rs](https://github.com/weipin/hello-async-rust/blob/main/examples/src/recv.rs)

```rust
{{#include ../../../examples/src/recv.rs:Recv}}
```

### Example: "loop-poll-hello"

"loop-poll-hello" sends `b"hello"` to a given echo service, creates a `Recv`,
and calls `poll` of the `Recv` until `Poll::Ready(Vec<u8>)` is returned.

Source: [examples/src/bin/loop-poll-hello.rs](https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/loop-poll-hello.rs)

### Example: "loop-poll-many-hello"

"loop-poll-many-hello" repeatedly sends `b"hello"` to a given echo service for
1000 times, creates a vector of 1000 `Recv`s, and calls `poll` of each `Recv`
until all returns `Poll::Ready(Vec<u8>)`.

Source: [examples/src/bin/loop-poll-many-hello.rs](https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/loop-poll-many-hello.rs)

### The echo service: "Lazy Echo"

"Lazy Echo" is an UDP echo service: the service sends received data back to
where it comes from. The service waits one second before any sending, hence
lazy.

"Lazy Echo" has two implementations: "lazy-echo-udp-smol" and
"lazy-echo-udp-tokio" -- same behavior but with different async runtimes.

- "lazy-echo-udp-smol" source: [examples/src/bin/lazy-echo-udp-smol.rs](https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/lazy-echo-udp-smol.rs)
- "lazy-echo-udp-tokio" source: [examples/src/bin/lazy-echo-udp-tokio.rs](https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/lazy-echo-udp-tokio.rs)

### Run the examples

1. Start the echo service, either "lazy-echo-udp-smol" or "lazy-echo-udp-tokio"
   * In the "examples" directory, execute:
     ```
     cargo run --bin lazy-echo-udp-smol
     ```

     OR

     ```
     cargo run --bin lazy-echo-udp-tokio
     ```
   * Keep the service running

1. Run "loop-poll-hello". In the "examples" directory, execute:
   ```
   cargo run --bin loop-poll-hello
   ```
1. Run "loop-poll-many-hello". In the "examples" directory, execute:
   ```
   cargo run --bin loop-poll-many-hello
   ```

---

**NOTE**

"loop-poll-hello" and "loop-poll-many-hello" should *both finish in
about 1 second*.

---

## Problems

### Wastes CPU time

As a Future has to be polled repeatedly and aggressively in a loop, CPU time is
wasted when data isn't ready.

Extreme example: when the data of a Future never becomes ready, the polling loop
is infinite.

### Doesn't scale

As all Futures have to be "polled" one after another, when a Future becomes ready,
its data may not be polled promptly.

Extreme example: when there are 1000 Futures and only the last one is ready, a
program has to poll 999 times before the last one can be accessed.