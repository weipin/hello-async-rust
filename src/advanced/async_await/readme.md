# `async` and `await`

`async` turns a function into a Future. `await`, which is "valid only within an
async context", composes Futures.

## Examples
### `async`

* An async-prefixed function `foo` as defined in its original form:
    ```rust
    async fn foo(v: usize) -> usize {
        // ...
    }
    ```
* The pseudocode mimics what the compiler generates:
    ```rust
    fn foo(v: usize) -> impl Future<Output = usize> {
        // ...
    }
    ```

### `await`

* A Future "awaits" in its original form:
    ```rust
    // ...
    // within an async context
    // ...
    let v = future.await;
    // ...
    ```
* The pseudocode mimics what the compiler generates (the keyword `yield` ties to a
[Coroutine][1]):
    ```rust
    // ...
    let mut future = pin!(future);
    let v = loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => break output,
            Poll::Pending => yield,
        }
    };
    // ...
    ```

## Closure and block

`async` and `await` can also be applied to a closure or a block.




[1]: https://doc.rust-lang.org/std/ops/trait.Coroutine.html