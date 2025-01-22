# Procedural

A Future (tree) is procedural.

Externally, as a whole, a Future is "single-threaded" -- at any given time, a
Future should be polled in *one* thread.

Internally, as a tree of Futures, the Futures involved in the "current external
poll" should be polled *one by one*, in the given thread.
