# std::future::Future

Rust futures are [`std::future::Future`][1]s. Types which implement the trait
expose the represented values through the method [`poll`][2]. The method returns
a Enum [`Poll`][3]: `Ready(T)` when the value is ready, or `Pending` if not.

`Poll` definition:
```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```


Diagram: Obtaining `value` of a `std::future::Future`.
```d2
code: Code processes value {
    getting_value: Obtaining value
    handling_value: Processing value
}

future: a std::future::Future
poll: std::task::Poll

code.getting_value -> future: "poll(...)"
future.ready: Is value ready?

future.ready.shape: diamond

future.ready -> poll.Pending: {
    source-arrowhead.label: NO
}
poll.Pending -> code.getting_value

future.ready-> poll.Ready: {
    source-arrowhead.label: YES
}
poll.Ready -> code.handling_value

```

[1]: https://doc.rust-lang.org/std/future/trait.Future.html
[2]: https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll
[3]: https://doc.rust-lang.org/std/task/enum.Poll.html