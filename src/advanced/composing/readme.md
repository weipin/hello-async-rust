# Composing Futures

A Future may compose with other Futures in order to be used. In other words,
before such a future returns its data, it needs to access and process data of
other given futures.

Diagram: Future A composes with Future B, and Future B composes with Future C
and D.
```d2
A {
    B {
        C
        D
    }
}
```
