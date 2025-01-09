# Runtime

A runtime provides what Rust left. Only with a runtime, async Rust can actually
be used to program.

## Features

Runtimes are all third-party. Some of the major features they provide:

1. Future implementations.
1. Reactor implementations.
1. Managing Futures.
1. Managing Wakers.
1. Scheduling Future polling.
1. Utilities.

## Links

1. [Tokio](https://github.com/tokio-rs/tokio)
1. [smol](https://github.com/smol-rs/smol)
1. [Embassy](https://github.com/embassy-rs/embassy)