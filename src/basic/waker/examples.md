# Examples

### Example: "reactor-hello"

Source: [examples/src/bin/reactor-hello.rs](https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/reactor-hello.rs)

### Example: "waker-many-hello"

Source: [examples/src/bin/waker-many-hello.rs](https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/waker-many-hello.rs)

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

1. Run "reactor-hello". In the "examples" directory, execute:
   ```
   cargo run --bin reactor-hello
   ```
1. Run "waker-many-hello". In the "examples" directory, execute:
   ```
   cargo run --bin waker-many-hello
   ```

---

**NOTE**

"reactor-hello" and "waker-many-hello" should *both finish in about 1 second*.

---
