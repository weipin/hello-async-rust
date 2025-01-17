# std::pin::Pin

The Future method [`poll`][1] involves [std::pin::Pin][2]. A Pin is a struct
which wraps a pointer to a given value.

When a pointer is wrapped in a Pin, its access is restricted. The restriction
prevents the value, which the pointer refers to, from being "moved" (change of
memory location).

## The problem `Pin` solves

A Future may define fields which refer to the future itself. The references
will become invalidated when the containing future moves.

To allow such "self-reference", Futures must not move, hence the requirement
of `Pin` for `poll`.


[1]: https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll
[2]: https://doc.rust-lang.org/std/pin/struct.Pin.html