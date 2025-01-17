# Example: join

Future B joins futures C and D, waiting for both to complete.
```d2
caller
b: B {
    continue: C and D both ready? {
        shape: diamond
    }
    c: C
    d: D
}
poll {
    ready: Ready
    pending: Pending
}

caller -> b: poll(..)
b.continue -> b.c: poll(..) {
    style.stroke-dash: 5
}
b.continue -> b.d: poll(..) {
    style.stroke-dash: 5
}
b.continue -> poll.pending: NO
b.continue -> poll.ready: YES
```

## Code
See [examples/src/bin/join-hello.rs][1]

[1]: https://github.com/weipin/hello-async-rust/blob/main/examples/src/bin/join-hello.rs

---

**NOTE**

C and D shares the same Waker.

---