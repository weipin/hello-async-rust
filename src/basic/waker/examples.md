# Examples

### Example: "reactor-hello-udp"

Source: [reactor-hello-udp.rs](https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/reactor-hello-udp.rs)

### Example: "waker-many-hello-udp"

Source: [waker-many-hello-udp.rs](https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/waker-many-hello-udp.rs)

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

1. Run "reactor-hello-udp". In the "examples" directory, execute:
   ```
   cargo run --bin reactor-hello-udp
   ```
1. Run "waker-many-hello-udp". In the "examples" directory, execute:
   ```
   cargo run --bin waker-many-hello-udp
   ```

---

**NOTE**

"reactor-hello-udp" and "waker-many-hello-udp" should *both finish in about 1
second*.

---


