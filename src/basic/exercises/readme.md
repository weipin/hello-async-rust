# Exercises

1. Write function `waker_fn` which "converts a closure into a Waker".
1. Write function `waker_unparks` which creates a waker. The waker unparks a
   given thread when waking.
1. Write function `block_on` which "blocks the current thread on a future".
1. Rewrite `Recv` to support the Waker pattern. Name the new struct
   `RecvWithWaker`.
1. Rewrite "reactor-hello" using `RecvWithWaker` and `block_on`.

## Solutions

1. See [examples/src/waker_fn.rs][1]
1. See [examples/src/block_on.rs][2]
1. See [examples/src/block_on.rs][2]
1. See [examples/src/recv_with_waker.rs][3]
1. See [examples/src/bin/block-on-hello.rs][4]

[1]: https://github.com/weipin/hello-async-rust/blob/main/examples/src/waker_fn.rs
[2]: https://github.com/weipin/hello-async-rust/blob/main/examples/src/block_on.rs
[3]: https://github.com/weipin/hello-async-rust/blob/main/examples/src/recv_with_waker.rs
[4]: https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/block-on-hello.rs


