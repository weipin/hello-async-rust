# Size

An `async` generated Future (tree) is "the perfectly sized stack", with
everything it needs across `yield`s.

The lint [large_futures][1] checks for such sizes, as they can become
unexpectedly large. You'll have to enable the lint manually as it's in the
group [clippy::pedantic][2].

See [examples/src/bin/large-future.rs][3] for an example. To examine the
warning, in the "examples" directory, execute:
```
cargo clippy -- -W clippy::pedantic
```

[1]: https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/large_futures.rs
[2]: https://doc.rust-lang.org/stable/clippy/usage.html#clippypedantic
[3]: https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/large-future.rs