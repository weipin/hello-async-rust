# Task

## Internally

Runtimes use Tasks internally to manage Futures. A Task is an object which
contains a Future and "everything else" needed to manage the Future.


## Externally

While "Task" is internal to a runtime, knowing that it exists can help us
understand the runtimes better, and thus use them well.

Like a Future, a Task is also [procedural](../composing/procedural.md). To have
Futures polled in a "multi-threaded" way, they must be spawned (see
[smol::spawn][1] or [tokio::task::spawn][2]).

[1]: https://docs.rs/smol/latest/smol/fn.spawn.html
[2]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
