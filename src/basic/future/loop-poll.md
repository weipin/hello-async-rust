# Polling in a loop

To obtain the value of a Future, one obvious approach is to call the method
`poll` in a loop until it returns `Poll::Ready(T)`.

## Example

### "UDP Hello" examples

The "UDP Hello" examples, which run on UDP protocol, send data `b"hello"`(repeatedly)
to a given echo service and receive the response(s).

The examples include "loop-poll-hello-udp" and "loop-poll-many-hello-udp".

### `RecvOnce`

`RecvOnce`, as a major part of the "UDP Hello" examples, is a Future which receives one response from a given service.
Its `poll` implementation reads data from a given UDP socket. `RecvOnce` requires and assumes that the UDP socket is in nonblocking mode.

Source: [lib.rs](examples/src/lib.rs)

```rust
{{#include ../../../examples/src/lib.rs:RecvOnce}}
```

### Example: "loop-poll-hello-udp"

"loop-poll-hello-udp" sends `b"hello"` to a given echo service, creates a `RecvOnce`, and calls `poll` of
the `RecvOnce` until `Poll::Ready(Vec<u8>)` is returned.

Source: [loop-poll-hello-udp.rs](examples/src/bin/loop-poll-hello-udp.rs)

### Example: "loop-poll-many-hello-udp"

"loop-poll-many-hello-udp" repeatedly sends `b"hello"` to a given echo service for 1000 times, creates
a vector of 1000 `RecvOnce`s, and calls `poll` of each `RecvOnce` until all
returns `Poll::Ready(Vec<u8>)`.

Source: [loop-poll-many-hello-udp.rs](examples/src/bin/loop-poll-hello-udp.rs)

### The echo service: "Lazy Echo"

"Lazy Echo" is an UDP echo service: the service sends received data back to where it comes from.
The service waits one second before any sending, hence lazy.

"Lazy Echo" has two implementations: "lazy-echo-udp-smol" and "lazy-echo-udp-tokio" --
same behavior but with different async runtimes.

- "lazy-echo-udp-smol" source: [lazy-echo-udp-smol.rs](examples/src/bin/lazy-echo-udp-smol.rs)
- "lazy-echo-udp-tokio" source: [lazy-echo-udp-tokio.rs](examples/src/bin/lazy-echo-udp-tokio.rs)

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

1. Run "loop-poll-hello-udp". In the "examples" directory, execute:
   ```
   cargo run --bin loop-poll-hello-udp
   ```
1. Run "loop-poll-many-hello-udp". In the "examples" directory, execute:
   ```
   cargo run --bin loop-poll-many-hello-udp
   ```

Note: "loop-poll-hello-udp" and "loop-poll-many-hello-udp" should both finish
in about 1 second.

## Problems

### Wastes CPU time

As a future has to be polled repeatedly and aggressively in a loop, CPU time is wasted
when data isn't ready.

Extreme example: when the data of a future never becomes ready, the polling loop is infinite.

### Doesn't scale

As all futures have to be "polled" one after another, when a Future becomes ready,
its data may not be polled promptly.

Extreme example: when there are 1000 futures and only the last one is ready, a program has to poll 999 times
before the last one can be accessed.