# std::task::Waker

Rust wakers are [`std::task::Waker`][1]s. A Waker wraps a
[`std::task::RawWaker`][2] which provides the waking functionality.

Reasons behind the "delegation":

1. RawWakers are unsafe to use. By wrapping RawWakers in Wakers, it possible to
   avoid the unsafe part.
1. [`std::task::LocalWaker`][3] (nightly) is the thread *unsafe* version of
   Waker, meaning that a LocalWaker have to be accessed from the same thread
where it was created. The wrapping enables code sharing between Waker and
LocalWaker.

# std::task::Context

Polling a Future is to call its method [poll][4], passing a Waker which is
wrapped in a [std::task::Context][5]. While "currently, Context only serves to
provide access to a &Waker", it's possible to add more fields to Context when
there is need to pass additional data to `poll()`. The API was designed this
way for future expanding in a backwards-compatible way.

## std::task::RawWaker

RawWakers function like [Trait Objects][6]. The [RawWakerVTable][7] provides a
common behavior, and the `*const ()` pointer stores arbitrary data that the
common behavior applies on.

While inconvenient, this "vtable strategy" has its benefits:
1. Breaks away from the requirements that [dyn compatibility][8] (formerly
   "object safety") enforces.
1. Enables [reducing allocations][9].


[1]: https://doc.rust-lang.org/std/task/struct.Waker.html
[2]: https://doc.rust-lang.org/std/task/struct.RawWaker.html
[3]: https://doc.rust-lang.org/std/task/struct.LocalWaker.html
[4]: https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll
[5]: https://doc.rust-lang.org/std/task/struct.Context.html
[6]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
[7]: https://doc.rust-lang.org/std/task/struct.RawWakerVTable.html
[8]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
[9]: https://tokio.rs/blog/2019-10-scheduler#reducing-allocations